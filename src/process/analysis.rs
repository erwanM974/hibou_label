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
use std::iter::FromIterator;
use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::*;
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;

use crate::process::verdicts::*;
use crate::process::hibou_process::*;
use crate::process::process_manager::*;


pub fn analyze(interaction : Interaction,
               multi_trace : AnalysableMultiTrace,
               gen_ctx : GeneralContext,
               pre_filters : Vec<HibouPreFilter>,
               strategy : HibouSearchStrategy,
               priorities : ProcessPriorities,
               loggers : Vec<Box<dyn ProcessLogger>>,
               sem_kind: SemanticKind,
               goal:GlobalVerdict) -> GlobalVerdict {
    // ***
    // ***
    let mut manager = HibouProcessManager::new(gen_ctx,
                                               strategy,
                                               Some(sem_kind),
                                               pre_filters,
                                               HashMap::new(),
                                               ProcessQueue::new(),
                                               priorities,
                                               loggers);
    // ***
    let multi_trace_option = Some(multi_trace);
    manager.init_loggers(&interaction,&multi_trace_option);
    let multi_trace = multi_trace_option.unwrap();
    // ***
    let mut next_state_id : u32 = 1;
    let mut node_counter : u32 = 0;
    let mut global_verdict = GlobalVerdict::Fail;
    // ***
    match enqueue_next_node_in_analysis(&mut manager,
                                        next_state_id,
                                        interaction,multi_trace,
                                        0,0) {
        None => {},
        Some( coverage_verdict ) => {
            global_verdict = update_global_verdict_from_new_coverage_verdict(global_verdict, coverage_verdict);
        }
    }
    next_state_id = next_state_id +1;
    node_counter = node_counter +1;
    // ***
    if global_verdict < goal {
        while let Some(next_to_process) = manager.extract_from_queue() {
            let new_state_id = next_state_id;
            next_state_id = next_state_id + 1;
            // ***
            let mut parent_state = manager.get_memorized_state(next_to_process.state_id).unwrap().clone();
            // ***
            match manager.process_next(&parent_state,
                                       &next_to_process,
                                       new_state_id,
                                       node_counter) {
                None => {},
                Some( (new_interaction,new_multi_trace,new_depth,new_loop_depth) ) => {
                    node_counter = node_counter + 1;
                    match enqueue_next_node_in_analysis(&mut manager,
                                                        new_state_id,
                                                        new_interaction,
                                                        new_multi_trace.unwrap(),
                                                        new_depth,
                                                        new_loop_depth) {
                        None => {},
                        Some( coverage_verdict ) => {
                            global_verdict = update_global_verdict_from_new_coverage_verdict(global_verdict, coverage_verdict);
                            if global_verdict >= goal {
                                break;
                            }
                        }
                    }
                }
            }
            // ***
            parent_state.remaining_ids_to_process.remove(&next_to_process.id_as_child);
            if parent_state.remaining_ids_to_process.len() == 0 {
                manager.forget_state(next_to_process.state_id);
            } else {
                manager.remember_state(next_to_process.state_id,parent_state);
            }
            // ***
        }
    }
    // ***
    manager.term_loggers(Some((&goal,&global_verdict)) );
    // ***
    return global_verdict;
}

fn enqueue_next_node_in_analysis(manager     : &mut HibouProcessManager,
                                 state_id    : u32,
                                 interaction : Interaction,
                                 multi_trace : AnalysableMultiTrace,
                                 depth       : u32,
                                 loop_depth  : u32) -> Option<CoverageVerdict> {
    // ***
    let mut next_child_id : u32 = 0;
    // ***
    let mut to_enqueue : Vec<(u32,NextToProcessKind)> = Vec::new();
    for front_pos in make_frontier(&interaction) {
        let front_act = interaction.get_sub_interaction(&front_pos).as_leaf();
        for canal in &multi_trace.canals {
            if canal.trace.len() > 0 {
                let head_act : &TraceAction = canal.trace.get(0).unwrap();
                if head_act.is_match(front_act) {
                    next_child_id = next_child_id +1;
                    let child_kind = NextToProcessKind::Execute(front_pos);
                    to_enqueue.push( (next_child_id,child_kind) );
                    break;
                }
            }
        }
    }
    // ***
    if next_child_id > 0 {
        let rem_child_ids : HashSet<u32> = HashSet::from_iter((1..(next_child_id+1)).collect::<Vec<u32>>().iter().cloned() );
        let memo_state = MemorizedState::new(interaction,Some(multi_trace),rem_child_ids, loop_depth, depth);
        manager.remember_state( state_id, memo_state );
        manager.enqueue_executions(state_id,to_enqueue);
        return None;
    } else {
        let verdict = manager.get_coverage_verdict(&interaction,&multi_trace);
        manager.verdict_loggers(&verdict,state_id);
        return Some( verdict );
    }
}


