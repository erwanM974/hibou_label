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
               prioritize_action : PrioritizeActionKind,
               loggers : Vec<Box<dyn ProcessLogger>>,
               sem_kind: SemanticKind) -> GlobalVerdict {
    // ***
    // ***
    let mut manager = HibouProcessManager::new(gen_ctx,
                                               strategy,
                                               Some(sem_kind),
                                               pre_filters,
                                               HashMap::new(),
                                               ProcessQueue::new(),
                                               prioritize_action,
                                               loggers);
    // ***
    let multi_trace_option = Some(multi_trace);
    manager.init_loggers(&interaction,&multi_trace_option);
    let multi_trace = multi_trace_option.unwrap();
    // ***
    let mut node_counter : u32 = 1;
    let mut global_verdict = GlobalVerdict::Fail;
    // ***
    {
        match enqueue_next_nodes_in_analysis(&mut manager,
                                             vec![0],
                                             interaction,multi_trace,
                                             0) {
            None => {},
            Some( coverage_verdict ) => {
                global_verdict = update_global_verdict_from_new_coverage_verdict(global_verdict, coverage_verdict);
            }
        }
    }
    // ***
    while let Some(next_to_process) = manager.extract_from_queue() {
        let mut new_state_id = next_to_process.state_id.clone();
        new_state_id.push(next_to_process.id_as_child);
        // ***
        let mut parent_state = manager.get_memorized_state(&next_to_process.state_id).unwrap().clone();
        // ***
        match next_to_process.kind {
            NextToProcessKind::Execute( position_to_execute ) => {
                match manager.process_next(&next_to_process.state_id,
                                           &new_state_id,
                                           &parent_state.interaction,
                                           &parent_state.multi_trace,
                                           position_to_execute,
                                           node_counter,parent_state.previous_loop_instanciations) {
                    None => {},
                    Some( (new_interaction,new_multi_trace,new_loop_depth)) => {
                        node_counter = node_counter + 1;
                        match enqueue_next_nodes_in_analysis(&mut manager,
                                                             new_state_id,
                                                             new_interaction,
                                                             new_multi_trace.unwrap(),
                                                             new_loop_depth) {
                            None => {},
                            Some( coverage_verdict ) => {
                                global_verdict = update_global_verdict_from_new_coverage_verdict(global_verdict, coverage_verdict);
                                match global_verdict {
                                    GlobalVerdict::Pass => {
                                        break;
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        // ***
        parent_state.remaining_ids_to_process.remove(&next_to_process.id_as_child);
        if parent_state.remaining_ids_to_process.len() == 0 {
            manager.forget_state(&next_to_process.state_id);
        } else {
            manager.remember_state(next_to_process.state_id,parent_state);
        }
        // ***
    }
    // ***
    manager.term_loggers(Some(&global_verdict) );
    // ***
    return global_verdict;
}

fn enqueue_next_nodes_in_analysis(manager: &mut HibouProcessManager,
                                  state_id : Vec<u32>,
                                  interaction : Interaction,
                                  multi_trace : AnalysableMultiTrace,
                                  previous_loop_instanciations:u32) -> Option<CoverageVerdict> {
    // ***
    let mut next_child_id : u32 = 0;
    // ***
    let mut to_enqueue : Vec<(u32,Position,TraceActionKind)> = Vec::new();
    for front_pos in make_frontier(&interaction) {
        let front_act = interaction.get_sub_interaction(&front_pos).as_leaf();
        for canal in &multi_trace.canals {
            if canal.trace.len() > 0 {
                let head_act : &TraceAction = canal.trace.get(0).unwrap();
                if head_act.is_match(front_act) {
                    next_child_id = next_child_id +1;
                    to_enqueue.push( (next_child_id,front_pos,head_act.act_kind.clone()) );
                    break;
                }
            }
        }
    }
    manager.enqueue_executions(&state_id,to_enqueue);
    // ***
    if next_child_id > 0 {
        let rem_child_ids : HashSet<u32> = HashSet::from_iter((1..(next_child_id+1)).collect::<Vec<u32>>().iter().cloned() );
        let memo_state = MemorizedState::new(interaction,Some(multi_trace),rem_child_ids, previous_loop_instanciations);
        manager.remember_state( state_id, memo_state );
        return None;
    } else {
        let verdict = manager.get_coverage_verdict(&interaction,&multi_trace);
        manager.verdict_loggers(&verdict,&state_id);
        return Some( verdict );
    }
}


