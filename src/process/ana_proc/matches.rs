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
use std::convert::TryFrom;
use std::iter::FromIterator;
use crate::core::general_context::GeneralContext;

use crate::core::semantics::frontier::{FrontierElement, global_frontier};
use crate::core::syntax::interaction::Interaction;
use crate::core::trace::TraceAction;
use crate::process::abstract_proc::generic::GenericStep;
use crate::process::ana_proc::interface::conf::AnalysisConfig;
use crate::process::ana_proc::interface::step::{AnalysisStepKind, SimulationStepKind};
use crate::process::ana_proc::manager::AnalysisProcessManager;
use crate::process::ana_proc::multitrace::AnalysableMultiTrace;
use crate::util::powerset::powerset;
use crate::process::ana_proc::anakind::{AnalysisKind, SimulationActionCriterion, SimulationLoopCriterion, UseLocalAnalysis};

impl AnalysisProcessManager {

    pub fn is_ok_to_simulate(&self,
                             frt_elt : &FrontierElement,
                             interaction : &Interaction,
                             multi_trace : &AnalysableMultiTrace) -> bool {
        match &self.ana_kind {
            AnalysisKind::Simulate(ref config) => {
                let mut ok_to_simulate = true;
                match config.act_crit {
                    SimulationActionCriterion::None => {
                        // nothing
                    },
                    _ => {
                        if multi_trace.rem_act_in_sim <= 0 {
                            ok_to_simulate = false;
                        }
                    }
                }
                match config.loop_crit {
                    SimulationLoopCriterion::None => {
                        // nothing
                    },
                    _ => {
                        let loop_depth = interaction.get_loop_depth_at_pos(&frt_elt.position);
                        if loop_depth > multi_trace.rem_loop_in_sim {
                            ok_to_simulate = false;
                        }
                    }
                }
                return ok_to_simulate;
            },
            _ => {
                panic!();
            }
        }
    }

    pub fn add_simulation_matches_in_analysis(&self,
                                              parent_id    : u32,
                                              interaction : &Interaction,
                                              multi_trace : &AnalysableMultiTrace,
                                              next_child_id : &mut u32,
                                              to_enqueue : &mut Vec<GenericStep<AnalysisConfig>>) {
        // ***
        for frt_elt in global_frontier(&interaction,&None) {
            let mut canal_ids_of_targets : HashSet<usize> = hashset!{}; // ids of the canals of the actions part of the frontier element
            for lf_id in &frt_elt.target_lf_ids {
                canal_ids_of_targets.insert( self.manager.gen_ctx.get_lf_coloc_id(*lf_id).unwrap() );
            }
            // ***
            let mut match_on_canal : Vec<usize> = vec!{}; // ids of the canals on which there is a match
            let mut ok_canals : HashSet<usize> = hashset!{}; // canals in which we already do something match or simu
            let mut act_left_to_match : HashSet<&TraceAction> = frt_elt.target_actions.iter().collect();
            for (canal_id, canal) in multi_trace.canals.iter().enumerate() {
                match canal.trace.get(0) {
                    None => {},
                    Some( got_multiact ) => {
                        let mut intersect_with_front_elt = false;
                        let mut entirely_included_in_front_elt = true;
                        for got_act in got_multiact {
                            if act_left_to_match.contains(got_act) {
                                intersect_with_front_elt = true;
                            } else {
                                entirely_included_in_front_elt = false;
                            }
                        }
                        // ***
                        if intersect_with_front_elt && entirely_included_in_front_elt {
                            match_on_canal.push(canal_id );
                            ok_canals.insert(canal_id);
                            for got_act in got_multiact {
                                act_left_to_match.remove(got_act);
                            }
                        }
                    }
                }
            }
            if multi_trace.length() > 0 {
                let mut to_simulate : HashMap<usize,SimulationStepKind> = hashmap!{}; // id of the canal on which the simulation step is done, which kind of simulation step
                let mut ok_to_simulate = true;
                if act_left_to_match.len() > 0 {
                    ok_to_simulate = self.is_ok_to_simulate(&frt_elt,interaction,multi_trace);
                }
                for tract in act_left_to_match {
                    if !ok_to_simulate {
                        break;
                    }
                    let tract_coloc_id = self.manager.gen_ctx.get_lf_coloc_id(tract.lf_id).unwrap();
                    if ok_canals.contains(&tract_coloc_id) {
                        println!("analysis line 101 : an action left to simulate on a coloc on which we already do some match-execution");
                    } else {
                        let mut gotit = false;
                        for (canal_id, canal) in multi_trace.canals.iter().enumerate() {
                            let canal_lifelines = self.manager.gen_ctx.co_localizations.get(canal_id).unwrap();
                            if canal_lifelines.contains(&tract.lf_id) {
                                if canal.trace.len() == 0 {
                                    to_simulate.insert( canal_id, SimulationStepKind::AfterEnd);
                                    gotit = true;
                                    break;
                                } else {
                                    if self.ana_kind.sim_before() && (canal.consumed == 0) {
                                        to_simulate.insert(canal_id,SimulationStepKind::BeforeStart);
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
                        let consu_set : HashSet<usize>;
                        {
                            let simu_set : HashSet<usize> = HashSet::from_iter(to_simulate.keys().cloned());
                            consu_set = HashSet::from_iter( canal_ids_of_targets.difference( &simu_set ).cloned() );
                        }
                        let generic_step = GenericStep{parent_id,
                            id_as_child:*next_child_id,
                            kind:AnalysisStepKind::Simulate(frt_elt.clone(),consu_set,
                                                            to_simulate.clone())};
                        to_enqueue.push( generic_step );
                    }
                    if match_on_canal.len() > 0 && self.is_ok_to_simulate(&frt_elt,interaction,multi_trace) {
                        for combinations in powerset(&match_on_canal) {
                            if combinations.len() > 0 {
                                let mut ok_to_simulate = true;
                                let mut to_simulate_more = to_simulate.clone();
                                for canal_id in combinations {
                                    if !ok_to_simulate{
                                        break;
                                    }
                                    let canal = multi_trace.canals.get(canal_id).unwrap();
                                    if canal.trace.len() == 0 {
                                        to_simulate_more.insert( canal_id, SimulationStepKind::AfterEnd);
                                    } else {
                                        if self.ana_kind.sim_before() && (canal.consumed == 0) {
                                            to_simulate_more.insert(canal_id,SimulationStepKind::BeforeStart);
                                        } else {
                                            ok_to_simulate = false;
                                        }
                                    }
                                }
                                if ok_to_simulate {
                                    {
                                        *next_child_id = *next_child_id +1;
                                        let consu_set : HashSet<usize>;
                                        {
                                            let simu_set : HashSet<usize> = HashSet::from_iter(to_simulate_more.keys().cloned());
                                            consu_set = HashSet::from_iter( canal_ids_of_targets.difference( &simu_set ).cloned() );
                                        }
                                        let generic_step = GenericStep{parent_id,
                                            id_as_child:*next_child_id,
                                            kind:AnalysisStepKind::Simulate(frt_elt.clone(),consu_set,
                                                                            to_simulate_more.clone())};
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


    pub fn add_action_matches_in_analysis(&self,
                                          parent_id    : u32,
                                          interaction : &Interaction,
                                          head_actions : &HashSet<&TraceAction>,
                                          id_as_child : &mut u32,
                                          to_enqueue : &mut Vec<GenericStep<AnalysisConfig>>) {
        // ***
        for frt_elt in global_frontier(&interaction,&Some(head_actions)) {
            *id_as_child = *id_as_child +1;
            let mut canal_ids_of_targets : HashSet<usize> = hashset!{}; // ids of the canals of the actions part of the frontier element
            for lf_id in &frt_elt.target_lf_ids {
                canal_ids_of_targets.insert( self.manager.gen_ctx.get_lf_coloc_id(*lf_id).unwrap() );
            }
            let generic_step = GenericStep{parent_id,
                id_as_child:*id_as_child,
                kind:AnalysisStepKind::Simulate(frt_elt,canal_ids_of_targets,hashmap!{})};
            to_enqueue.push( generic_step );
        }
    }

}





