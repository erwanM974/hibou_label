/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::{HashSet,HashMap};

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction};
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;

use crate::process::verdicts::CoverageVerdict;
use crate::process::hibou_process::*;
use crate::process::process_manager::*;

pub fn explore(interaction : Interaction,
               gen_ctx : GeneralContext,
               pre_filters : Vec<HibouPreFilter>,
               strategy : HibouSearchStrategy,
               loggers : Vec<Box<dyn ProcessLogger>>) {
    // ***
    let mut manager = HibouProcessManager::new(gen_ctx,strategy,pre_filters,loggers);
    manager.init_loggers(&interaction,&None);
    // ***
    // ***
    let mut exploration_queue : Vec<ProcessStateNode> = Vec::new();
    {
        let frontier = make_frontier(&interaction);
        if frontier.len() > 0 {
            let first_node = ProcessStateNode::new(vec![0],interaction,frontier,1,None,0);
            exploration_queue.push( first_node );
        }
    }
    // ***
    let mut node_counter : u32 = 0;
    while exploration_queue.len() > 0 {
        let state_node = manager.extract_from_queue(&mut exploration_queue );
        let next_result : ProcessStepResult = manager.process_next(state_node,node_counter);
        // ***
        let added_in_queue : bool;
        match next_result.new_state_node {
            None => {
                added_in_queue = false;
            },
            Some( new_node ) => {
                if new_node.rem_front_or_match.len() > 0 {
                    exploration_queue.push(new_node);
                    added_in_queue = true;
                } else {
                    added_in_queue = false;
                }
            }
        }
        // ***
        match next_result.put_back_state_node {
            None => {}
            Some( to_put_back ) => {
                manager.put_back_in_queue(&mut exploration_queue, to_put_back,added_in_queue);
            }
        }
    }
    // ***
    manager.term_loggers();
}
