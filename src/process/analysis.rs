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

use crate::core::semantics::execute::execute_interaction;

use crate::process::verdicts::*;
use crate::process::hibou_process::*;
use crate::process::process_manager::*;
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
                                               hoptions.frontier_priorities,
                                               hoptions.loggers);
    // ***
    let mut multi_trace = multi_trace;
    manager.init_loggers(&interaction,&Some(&multi_trace));
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
    for frt_elt in global_frontier(interaction,&Some(head_actions)) {
        *next_child_id = *next_child_id +1;
        let child_kind = NextToProcessKind::Execute(frt_elt);
        to_enqueue.push( (*next_child_id,child_kind) );
    }
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect()
}


fn add_simulation_matches_in_analysis(interaction : &Interaction,
                                  multi_trace : &AnalysableMultiTrace,
                                      sim_before:bool,
                                  next_child_id : &mut u32,
                                  to_enqueue : &mut Vec<(u32,NextToProcessKind)>) {
    // ***
    for frt_elt in global_frontier(&interaction,&None) {
        let mut match_on_canal : Vec<(usize,usize)> = vec!{}; // ids of the canals on which there is a match
        let mut ok_lifelines : HashSet<usize> = hashset!{}; // lifelines in which we already do something match or simu
        let mut act_left_to_match : HashSet<&TraceAction> = frt_elt.target_actions.iter().collect();
        for (canal_id, canal) in multi_trace.canals.iter().enumerate() {
            match canal.trace.get(0) {
                None => {},
                Some( got_act ) => {
                    if act_left_to_match.contains(got_act) {
                        match_on_canal.push((got_act.lf_id,canal_id) );
                        act_left_to_match.remove(got_act);
                        ok_lifelines.extend(&canal.lifelines);
                    }
                }
            }
        }
        if multi_trace.length() > 0 {
            let mut to_simulate : HashMap<usize,SimulationStepKind> = hashmap!{};
            let mut ok_to_simulate = true;
            if act_left_to_match.len() > 0 && interaction.get_loop_depth_at_pos(&frt_elt.position) > multi_trace.remaining_loop_instantiations_in_simulation {
                ok_to_simulate = false;
            }
            for tract in act_left_to_match {
                if !ok_to_simulate {
                    break;
                }
                if ok_lifelines.contains(&tract.lf_id) {
                    println!("analysis line 199 : several actions on the same lifeline ?");
                } else {
                    let mut gotit = false;
                    for canal in &multi_trace.canals {
                        if canal.lifelines.contains(&tract.lf_id) {
                            if canal.trace.len() == 0 {
                                to_simulate.insert( tract.lf_id, SimulationStepKind::AfterEnd);
                                gotit = true;
                                break;
                            } else {
                                if sim_before && (canal.consumed == 0) {
                                    to_simulate.insert(tract.lf_id,SimulationStepKind::BeforeStart);
                                    gotit = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !gotit {
                        ok_to_simulate = false;
                    }
                }
            }
            if ok_to_simulate {
                {
                    *next_child_id = *next_child_id +1;
                    let child_kind = NextToProcessKind::Simulate(frt_elt.clone(),to_simulate.clone());
                    to_enqueue.push( (*next_child_id,child_kind) );
                }
                if match_on_canal.len() > 0 && interaction.get_loop_depth_at_pos(&frt_elt.position) <= multi_trace.remaining_loop_instantiations_in_simulation {
                    for combinations in powerset(&match_on_canal) {
                        if combinations.len() > 0 {
                            let mut ok_to_simulate = true;
                            let mut to_simulate_more = to_simulate.clone();
                            for (lf_id,canal_id) in combinations {
                                if !ok_to_simulate{
                                    break;
                                }
                                let canal = multi_trace.canals.get(canal_id).unwrap();
                                if canal.trace.len() == 0 {
                                    to_simulate_more.insert( lf_id, SimulationStepKind::AfterEnd);
                                } else {
                                    if sim_before && (canal.consumed == 0) {
                                        to_simulate_more.insert(lf_id,SimulationStepKind::BeforeStart);
                                    } else {
                                        ok_to_simulate = false;
                                    }
                                }
                            }
                            if ok_to_simulate {
                                {
                                    *next_child_id = *next_child_id +1;
                                    let child_kind = NextToProcessKind::Simulate(frt_elt.clone(),to_simulate_more.clone());
                                    to_enqueue.push( (*next_child_id,child_kind) );
                                }
                            }
                        }
                    }
                }
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
            let mut to_hide : HashSet<usize> = HashSet::new();
            for canal in &multi_trace.canals {
                if (canal.flag_hidden == false) && (canal.trace.len() == 0) { // && (interaction.involves_any_of(&canal.lifelines)) {
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
        },
        &AnalysisKind::Simulate(sim_before) => {
            add_simulation_matches_in_analysis(&interaction,&multi_trace,sim_before,&mut next_child_id, &mut to_enqueue);
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


