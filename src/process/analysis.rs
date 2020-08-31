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
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction,multitrace_length};
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;

use crate::process::verdicts::CoverageVerdict;
use crate::process::hibou_process::*;
use crate::process::process_manager::*;

pub enum GlobalVerdict {
    Pass,
    Fail
}

impl std::string::ToString for GlobalVerdict {
    fn to_string(&self) -> String {
        match self {
            GlobalVerdict::Pass => {
                return "Pass".to_string();
            },
            GlobalVerdict::Fail => {
                return "Fail".to_string();
            }
        }
    }
}


fn check_cov(state_node : &ProcessStateNode, manager : &mut HibouProcessManager) -> bool {
    match &state_node.multitrace {
        None => {
            panic!();
        },
        Some(ref multitrace) => {
            if multitrace_length(multitrace) == 0 {
                if (&state_node.interaction).express_empty() {
                    manager.verdict_loggers(&CoverageVerdict::Cov,&state_node.state_id);
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn analyze(interaction : Interaction,
               multitrace : AnalysableMultiTrace,
               gen_ctx : GeneralContext,
               pre_filters : Vec<HibouPreFilter>,
               strategy : HibouSearchStrategy,
               loggers : Vec<Box<dyn ProcessLogger>>) -> GlobalVerdict {
    // ***
    // ***
    let mut manager = HibouProcessManager::new(gen_ctx,strategy,pre_filters,loggers);
    let multitrace_option = Some(multitrace);
    manager.init_loggers(&interaction,&multitrace_option);
    let multitrace = multitrace_option.unwrap();
    // ***
    // ***
    let mut node_counter : u32 = 1;
    let mut verdict = GlobalVerdict::Fail;
    // ***
    let mut analysis_queue : Vec<ProcessStateNode> = Vec::new();
    {

        let mut filtered_front = make_matches(&interaction,&multitrace);
        let first_node = ProcessStateNode::new(vec![0],interaction,filtered_front,1,Some(multitrace),0);
        if check_cov(&first_node,&mut manager) {
            verdict = GlobalVerdict::Pass;
        } else {
            if first_node.rem_front_or_match.len() > 0 {
                analysis_queue.push( first_node );
            } else {
                manager.verdict_loggers(&CoverageVerdict::UnCov,&first_node.state_id);
            }
        }
    }
    // ***
    while analysis_queue.len() > 0 {
        let state_node = manager.extract_from_queue(&mut analysis_queue );
        let next_result : ProcessStepResult = manager.process_next(state_node,node_counter);
        // ***
        let added_in_queue : bool;
        match next_result.new_state_node {
            None => {
                added_in_queue = false;
            },
            Some( new_node ) => {
                if check_cov(&new_node,&mut manager) {
                    verdict = GlobalVerdict::Pass;
                    break;
                } else {
                    if new_node.rem_front_or_match.len() > 0 {
                        analysis_queue.push( new_node );
                        node_counter = node_counter + 1;
                        added_in_queue = true;
                    } else {
                        manager.verdict_loggers(&CoverageVerdict::UnCov,&new_node.state_id);
                        added_in_queue = false;
                    }
                }
            }
        }
        // ***
        match next_result.put_back_state_node {
            None => {}
            Some( to_put_back ) => {
                manager.put_back_in_queue(&mut analysis_queue, to_put_back,added_in_queue);
            }
        }
    }
    // ***
    manager.term_loggers();
    // ***
    return verdict;
}
