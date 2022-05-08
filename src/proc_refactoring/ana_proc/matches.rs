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


use std::collections::{HashMap, HashSet};
use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::Interaction;
use crate::core::trace::{AnalysableMultiTrace, TraceAction};
use crate::proc_refactoring::abstract_proc::GenericStep;
use crate::proc_refactoring::ana_proc::conf::AnalysisConfig;
use crate::proc_refactoring::ana_proc::step::AnalysisStepKind;
use crate::process::hibou_process::SimulationStepKind;

pub fn add_action_matches_in_analysis(parent_id    : u32,
                                      interaction : &Interaction,
                                      head_actions : &HashSet<&TraceAction>,
                                      id_as_child : &mut u32,
                                      to_enqueue : &mut Vec<GenericStep<AnalysisConfig>>) {
    // ***
    for frt_elt in global_frontier(&interaction,&Some(head_actions)) {
        *id_as_child = *id_as_child +1;
        let generic_step = GenericStep{parent_id,id_as_child:*id_as_child,kind:AnalysisStepKind::Simulate(frt_elt,hashmap!{})};
        to_enqueue.push( generic_step );
    }
}


fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect()
}


pub fn add_simulation_matches_in_analysis(parent_id    : u32,
                                          interaction : &Interaction,
                                      multi_trace : &AnalysableMultiTrace,
                                      sim_before:bool,
                                      next_child_id : &mut u32,
                                      to_enqueue : &mut Vec<GenericStep<AnalysisConfig>>) {
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
                    let generic_step = GenericStep{parent_id,id_as_child:*next_child_id,kind:AnalysisStepKind::Simulate(frt_elt.clone(),to_simulate.clone())};
                    to_enqueue.push( generic_step );
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
                                    let generic_step = GenericStep{parent_id,id_as_child:*next_child_id,kind:AnalysisStepKind::Simulate(frt_elt.clone(),to_simulate_more.clone())};
                                    to_enqueue.push( generic_step );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

