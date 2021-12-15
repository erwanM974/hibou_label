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
use crate::core::semantics::frontier::global_frontier;
use crate::core::semantics::locfront::local_frontier;

use crate::core::semantics::execute::execute;

use crate::process::verdicts::*;
use crate::process::hibou_process::*;
use crate::process::process_manager::*;
use crate::process::queue::*;
use crate::process::priorities::ProcessPriorities;


use crate::process::anakind::{AnalysisKind,UseLocalAnalysis};
use crate::from_hfiles::hibou_options::HibouOptions;

pub fn analyze(interaction : Interaction,
               multi_trace : AnalysableMultiTrace,
               gen_ctx : GeneralContext,
               hoptions : HibouOptions) -> (GlobalVerdict,u32) {
    // ***
    let goal = hoptions.goal;
    // ***
    let mut manager = HibouProcessManager::new(gen_ctx,
                                               hoptions.strategy,
                                               Some(hoptions.ana_kind.unwrap()),
                                               hoptions.local_analysis,
                                               hoptions.pre_filters,
                                               HashMap::new(),
                                               Box::new(SimpleProcessQueue::new()),
                                               hoptions.frontier_priorities,
                                               hoptions.loggers);
    // ***
    let multi_trace_option = Some(multi_trace);
    manager.init_loggers(&interaction,&multi_trace_option);
    let mut multi_trace = multi_trace_option.unwrap();
    multi_trace.remaining_loop_instantiations_in_simulation = interaction.max_nested_loop_depth();
    // ***
    let mut next_state_id : u32 = 1;
    let mut node_counter : u32 = 0;
    let mut global_verdict = GlobalVerdict::Fail;
    // ***
    // ***
    let pursue_analysis : bool;
    match enqueue_next_node_in_analysis(&mut manager,
                                        next_state_id,
                                        interaction,multi_trace,
                                        0,0) {
        None => {
            pursue_analysis = true;
        },
        Some( coverage_verdict ) => {
            global_verdict = update_global_verdict_from_new_coverage_verdict(global_verdict, coverage_verdict);
            match goal.as_ref() {
                None => {
                    pursue_analysis = true;
                },
                Some( target_goal ) => {
                    if &global_verdict < target_goal {
                        pursue_analysis = true;
                    } else {
                        pursue_analysis = false;
                    }
                }
            }
        }
    }
    next_state_id = next_state_id +1;
    node_counter = node_counter +1;
    // ***
    if pursue_analysis {
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
                            match goal.as_ref() {
                                None => {},
                                Some( target_goal ) => {
                                    if &global_verdict >= target_goal {
                                        break;
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
    return (global_verdict,node_counter);
}


fn add_action_matches_in_analysis(interaction : &Interaction,
                                  //multi_trace : &AnalysableMultiTrace,
                                  head_actions : &HashSet<&TraceAction>,
                                  next_child_id : &mut u32,
                                  to_enqueue : &mut Vec<(u32,NextToProcessKind)>) {
    // ***
    for front_pos in global_frontier(interaction) {
        let front_act = interaction.get_sub_interaction(&front_pos).as_leaf();
        for head_act in head_actions {
            if head_act.is_match(front_act) {
                *next_child_id = *next_child_id +1;
                let child_kind = NextToProcessKind::Execute(front_pos);
                to_enqueue.push( (*next_child_id,child_kind) );
                break;
            }
        }
    }
}

fn enqueue_next_node_in_analysis(manager     : &mut HibouProcessManager,
                                 state_id    : u32,
                                 interaction : Interaction,
                                 mut multi_trace : AnalysableMultiTrace,
                                 depth       : u32,
                                 loop_depth  : u32) -> Option<CoverageVerdict> {
    // ***
    let mut next_child_id : u32 = 0;
    // ***
    if manager.is_dead_local_analysis(&interaction,&mut multi_trace) {
        let verdict = CoverageVerdict::Dead;
        manager.verdict_loggers(&verdict,state_id);
        return Some( verdict );
    }
    // ***
    let mut to_enqueue : Vec<(u32,NextToProcessKind)> = Vec::new();
    let head_actions = multi_trace.head_actions();
    // ***
    match manager.get_ana_kind() {
        &AnalysisKind::Accept => {
            add_action_matches_in_analysis(&interaction,&head_actions,&mut next_child_id, &mut to_enqueue);
        },
        &AnalysisKind::Prefix => {
            add_action_matches_in_analysis(&interaction,&head_actions,&mut next_child_id, &mut to_enqueue);
        },
        &AnalysisKind::Hide => {
            if multi_trace.length() > 0 {
                let mut to_hide : HashSet<usize> = HashSet::new();
                for canal in &multi_trace.canals {
                    if (canal.trace.len() == 0) && (canal.flag_hidden == false) && (interaction.involves_any_of(&canal.lifelines)) {
                        to_hide.extend( canal.lifelines.clone() )
                    }
                }
                //
                if to_hide.len() > 0 {
                    next_child_id = next_child_id +1;
                    let child_kind = NextToProcessKind::Hide( to_hide );
                    to_enqueue.push( (next_child_id,child_kind) );
                } else {
                    add_action_matches_in_analysis(&interaction,&head_actions,&mut next_child_id, &mut to_enqueue);
                }
            }
        },
        &AnalysisKind::Simulate(sim_before) => {
            add_action_matches_in_analysis(&interaction,&head_actions,&mut next_child_id, &mut to_enqueue);
            if multi_trace.length() > 0 {
                for front_pos in global_frontier(&interaction) {
                    if interaction.get_loop_depth_at_pos(&front_pos) <= multi_trace.remaining_loop_instantiations_in_simulation {
                        let front_act = interaction.get_sub_interaction(&front_pos).as_leaf();
                        for canal in &multi_trace.canals {
                            if canal.lifelines.contains(&front_act.lf_id) {
                                // ***
                                let ok_to_simulate : Option<SimulationStepKind>;
                                // ***
                                if canal.trace.len() == 0 {
                                    ok_to_simulate = Some(SimulationStepKind::AfterEnd);
                                } else {
                                    // ***
                                    if sim_before && (canal.consumed == 0) {
                                        ok_to_simulate = Some(SimulationStepKind::BeforeStart);
                                    } else {
                                        ok_to_simulate = None;
                                    }
                                }
                                match ok_to_simulate {
                                    None => {},
                                    Some( sim_step_kind ) => {
                                        // ***
                                        // additional checking; we instanciate content from loops
                                        // iff it contains actions susceptible to allow duther consumption of the multi-trace
                                        let mut confirm_simulate : bool = false;
                                        match interaction.get_outermost_loop_content(&front_pos) {
                                            None => {
                                                confirm_simulate = true;
                                            },
                                            Some( (loop_content,relative_front_pos) ) => {
                                                let (after_execute_content,_) = execute(loop_content,relative_front_pos, front_act.lf_id);
                                                let contained_actions = after_execute_content.contained_actions();
                                                for head_act in &head_actions {
                                                    if contained_actions.contains(head_act){
                                                        confirm_simulate = true;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        // ***
                                        if confirm_simulate {
                                            next_child_id = next_child_id +1;
                                            let child_kind = NextToProcessKind::Simulate(front_pos,sim_step_kind);
                                            to_enqueue.push( (next_child_id,child_kind) );
                                        }
                                        // ***
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    // ***
    if next_child_id > 0 {
        let rem_child_ids : HashSet<u32> = HashSet::from_iter((1..(next_child_id+1)).collect::<Vec<u32>>().iter().cloned() );
        let memo_state = MemorizedState::new(interaction,Some(multi_trace),rem_child_ids, loop_depth, depth);
        manager.remember_state( state_id, memo_state );
        manager.enqueue_executions(state_id,to_enqueue,depth);
        return None;
    } else {
        let verdict = manager.get_coverage_verdict(&interaction,&multi_trace);
        manager.verdict_loggers(&verdict,state_id);
        return Some( verdict );
    }
}


