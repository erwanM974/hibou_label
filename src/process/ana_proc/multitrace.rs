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
use crate::core::general_context::GeneralContext;
use crate::core::syntax::interaction::Interaction;
use crate::core::trace::TraceAction;
use crate::process::ana_proc::anakind::{SimulationActionCriterion, SimulationConfiguration, SimulationLoopCriterion};
use crate::process::ana_proc::interface::step::SimulationStepKind;


#[derive(Clone, PartialEq, Debug)]
pub struct AnalysableMultiTraceCanal {
    pub trace : Vec<HashSet<TraceAction>>,
    pub flag_hidden : bool,
    pub flag_dirty4local : bool,
    pub consumed : u32,
    pub simulated_before : u32,
    pub simulated_after : u32
}

impl AnalysableMultiTraceCanal {
    pub fn new(trace : Vec<HashSet<TraceAction>>,
               flag_hidden : bool,
               flag_dirty4local : bool,
               consumed : u32,
               simulated_before : u32,
               simulated_after : u32) -> AnalysableMultiTraceCanal {
        return AnalysableMultiTraceCanal{trace,flag_hidden,flag_dirty4local,consumed,simulated_before,simulated_after};
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnalysableMultiTrace {
    pub canals : Vec<AnalysableMultiTraceCanal>,
    pub rem_loop_in_sim : u32,
    pub rem_act_in_sim : u32
}

pub enum WasMultiTraceConsumedWithSimulation {
    No,
    OnlyAfterEnd,
    AsSlice
}

impl AnalysableMultiTrace {

    pub fn new(canals:Vec<AnalysableMultiTraceCanal>,
               rem_loop_in_sim : u32,
               rem_act_in_sim : u32) -> AnalysableMultiTrace {
        return AnalysableMultiTrace{canals,rem_loop_in_sim,rem_act_in_sim};
    }

    pub fn head_actions(&self) -> HashSet<&TraceAction> {
        let mut heads : HashSet<&TraceAction> = HashSet::new();
        for canal in &self.canals {
            if canal.trace.len() > 0 {
                let canal_head = canal.trace.get(0).unwrap();
                heads.extend(canal_head);
            }
        }
        return heads;
    }

    pub fn length(&self) -> usize {
        let mut length = 0;
        for canal in &self.canals {
            length = length + (canal.trace.len());
        }
        return length;
    }

    pub fn is_any_component_empty(&self) -> bool {
        for canal in &self.canals {
            if canal.trace.len() == 0 {
                return true;
            }
        }
        return false;
    }

    pub fn is_any_component_hidden(&self) -> bool {
        for canal in &self.canals {
            if canal.flag_hidden {
                return true;
            }
        }
        return false;
    }

    pub fn is_simulated(&self) -> WasMultiTraceConsumedWithSimulation {
        let mut got_sim_after = false;
        for canal in &self.canals {
            if canal.simulated_before > 0 {
                return WasMultiTraceConsumedWithSimulation::AsSlice;
            }
            if canal.simulated_after > 0 {
                got_sim_after = true;
            }
        }
        if got_sim_after {
            return WasMultiTraceConsumedWithSimulation::OnlyAfterEnd;
        } else {
            return WasMultiTraceConsumedWithSimulation::No;
        }
    }

    pub fn update_on_execution(&self,
                               sim_config : &SimulationConfiguration,
                               gen_ctx : &GeneralContext,
                               target_lf_ids : &HashSet<usize>,
                               affected_lfs : &HashSet<usize>,
                               new_interaction : &Interaction) -> AnalysableMultiTrace {
        let mut new_canals : Vec<AnalysableMultiTraceCanal> = Vec::new();
        // ***
        for coloc_id in 0..self.canals.len() {
            let lifelines = gen_ctx.co_localizations.get(coloc_id).unwrap();
            let mut updated_canal = self.canals.get(coloc_id).unwrap().clone();
            // ***
            if !lifelines.is_disjoint(&target_lf_ids) {
                updated_canal.trace.remove(0);
                updated_canal.consumed = updated_canal.consumed + 1;
            }
            // ***
            if !lifelines.is_disjoint(affected_lfs) {
                updated_canal.flag_dirty4local = true;
            }
            // ***
            new_canals.push( updated_canal );
        }
        // ***
        let rem_loop_in_sim= sim_config.get_reset_rem_loop(new_interaction);
        let rem_act_in_sim = sim_config.get_reset_rem_act(new_interaction);
        // ***
        return AnalysableMultiTrace::new(new_canals,rem_loop_in_sim,rem_act_in_sim);
    }



    pub fn update_on_hide(&self, gen_ctx : &GeneralContext, lfs_to_hide : &HashSet<usize>) -> AnalysableMultiTrace {
        let mut new_canals : Vec<AnalysableMultiTraceCanal> = Vec::new();
        // ***
        for coloc_id in 0..self.canals.len() {
            let lifelines = gen_ctx.co_localizations.get(coloc_id).unwrap();
            let mut updated_canal = self.canals.get(coloc_id).unwrap().clone();
            if lifelines.is_subset( lfs_to_hide ) {
                updated_canal.flag_hidden = true;
            }
            new_canals.push( updated_canal );
        }
        // ***
        return AnalysableMultiTrace::new(new_canals,0,0);
    }

    pub fn update_on_simulation(&self,
                                sim_config : &SimulationConfiguration,
                                consu_set : &HashSet<usize>,
                                sim_map : &HashMap<usize,SimulationStepKind>, // id of canal on which simulation step is done, kind of simulation step
                                gen_ctx : &GeneralContext,
                                target_lf_ids : &HashSet<usize>,
                                affected_lfs : &HashSet<usize>,
                                loop_depth : u32,
                                new_interaction : &Interaction) -> AnalysableMultiTrace {
        // ***
        let mut new_canals : Vec<AnalysableMultiTraceCanal> = Vec::new();
        // ***
        for coloc_id in 0..self.canals.len() {
            let lifelines = gen_ctx.co_localizations.get(coloc_id).unwrap();
            let mut updated_canal = self.canals.get(coloc_id).unwrap().clone();
            // ***
            if !lifelines.is_disjoint(affected_lfs) {
                updated_canal.flag_dirty4local = true;
            }
            // ***
            match sim_map.get(&coloc_id) {
                None => {
                    if !lifelines.is_disjoint(target_lf_ids) {
                        updated_canal.trace.remove(0);
                        updated_canal.consumed += 1;
                    }
                },
                Some( sim_kind ) => {
                    match sim_kind {
                        SimulationStepKind::BeforeStart => {
                            updated_canal.simulated_before += 1;
                        },
                        SimulationStepKind::AfterEnd => {
                            updated_canal.simulated_after += 1;
                        }
                    }
                }
            }
            // ***
            new_canals.push( updated_canal );
        }
        // ***
        let rem_loop_in_sim : u32;
        match sim_config.loop_crit {
            SimulationLoopCriterion::MaxDepth => {
                if consu_set.len() > 0 {
                    rem_loop_in_sim = new_interaction.max_nested_loop_depth();
                } else {
                    let on_crit = new_interaction.max_nested_loop_depth();
                    let removed = self.rem_loop_in_sim - loop_depth;
                    rem_loop_in_sim = on_crit.min(removed);
                }
            },
            SimulationLoopCriterion::MaxNum => {
                if consu_set.len() > 0 {
                    rem_loop_in_sim = new_interaction.total_loop_num();
                } else {
                    let on_crit = new_interaction.total_loop_num();
                    let removed = self.rem_loop_in_sim - loop_depth;
                    rem_loop_in_sim = on_crit.min(removed);
                }
            },
            SimulationLoopCriterion::SpecificNum( sn ) => {
                if consu_set.len() > 0 {
                    rem_loop_in_sim = sn;
                } else {
                    let on_crit = sn;
                    let removed = self.rem_loop_in_sim - loop_depth;
                    rem_loop_in_sim = on_crit.min(removed);
                }
            },
            SimulationLoopCriterion::None => {
                rem_loop_in_sim = 0;
            }
        }
        // ***
        let rem_act_in_sim : u32;
        match sim_config.act_crit {
            SimulationActionCriterion::SpecificNum( sn ) => {
                if consu_set.len() > 0 {
                    rem_act_in_sim = sn;
                } else {
                    let on_crit = sn;
                    let removed = self.rem_act_in_sim - 1;
                    rem_act_in_sim = on_crit.min(removed);
                }
            },
            SimulationActionCriterion::None => {
                rem_act_in_sim = 0;
            }
        }
        // ***
        return AnalysableMultiTrace::new(new_canals,rem_loop_in_sim,rem_act_in_sim);
    }
}


