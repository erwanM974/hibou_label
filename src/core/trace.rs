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
use crate::core::syntax::action::*;
use crate::core::syntax::interaction::Interaction;
use crate::process::hibou_process::SimulationStepKind;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TraceActionKind {
    Reception,
    Emission
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TraceAction {
    pub lf_id : usize,
    pub act_kind : TraceActionKind,
    pub ms_id : usize
}

impl TraceAction {

    pub fn new(lf_id : usize,
               act_kind : TraceActionKind,
               ms_id : usize) -> TraceAction {
        return TraceAction{lf_id,act_kind,ms_id};
    }

    /*pub fn is_match(&self,model_action: &ObservableAction) -> bool {
        if model_action.lf_id != self.lf_id {
            return false;
        }
        if model_action.ms_id != self.ms_id {
            return false;
        }
        if self.act_kind != model_action.get_action_kind() {
            return false;
        }
        return true;
    }*/
}


#[derive(Clone, PartialEq, Debug)]
pub struct MultiTraceCanal {
    pub lifelines : HashSet<usize>,
    pub trace : Vec<TraceAction>,
    pub flag_hidden : bool,
    pub flag_dirty4local : bool,
    pub consumed : u32,
    pub simulated_before : u32,
    pub simulated_after : u32
}

impl MultiTraceCanal {
    pub fn new(lifelines : HashSet<usize>,
               trace : Vec<TraceAction>,
               flag_hidden : bool,
               flag_dirty4local : bool,
               consumed : u32,
               simulated_before : u32,
               simulated_after : u32) -> MultiTraceCanal {
        return MultiTraceCanal{lifelines,trace,flag_hidden,flag_dirty4local,consumed,simulated_before,simulated_after};
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnalysableMultiTrace {
    pub canals : Vec<MultiTraceCanal>,
    pub remaining_loop_instantiations_in_simulation : u32
}

impl AnalysableMultiTrace {
    pub fn new(canals:Vec<MultiTraceCanal>,remaining_loop_instantiations_in_simulation : u32) -> AnalysableMultiTrace {
        return AnalysableMultiTrace{canals,remaining_loop_instantiations_in_simulation};
    }

    pub fn head_actions(&self) -> HashSet<&TraceAction> {
        let mut heads : HashSet<&TraceAction> = HashSet::new();
        for canal in &self.canals {
            if canal.trace.len() > 0 {
                heads.insert( canal.trace.get(0).unwrap() );
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

    pub fn are_colocalizations_singletons(&self) -> bool {
        for canal in &self.canals {
            if canal.lifelines.len() > 1 {
                return false;
            }
        }
        return true;
    }

    pub fn update_on_execution(&self,
                               target_lf_ids : &HashSet<usize>,
                               affected_lfs : &HashSet<usize>,
                               new_interaction : &Interaction) -> AnalysableMultiTrace {
        let remaining_loop_instantiations_in_simulation = new_interaction.max_nested_loop_depth();
        let mut new_canals : Vec<MultiTraceCanal> = Vec::new();
        for canal in &self.canals {
            let canal_lfs = canal.lifelines.clone();
            let mut new_trace = canal.trace.clone();
            // ***
            let new_simu_before = canal.simulated_before;
            let new_simu_after = canal.simulated_after;
            let new_flag_hidden = canal.flag_hidden;
            // ***
            let mut new_consumed = canal.consumed;
            if !canal_lfs.is_disjoint(&target_lf_ids) {
                new_trace.remove(0);
                new_consumed = new_consumed + 1;
            }
            // ***
            let new_flag_dirty4local : bool;
            if new_trace.len() > 0 {
                if canal_lfs.is_disjoint(affected_lfs) {
                    new_flag_dirty4local = canal.flag_dirty4local;
                } else {
                    new_flag_dirty4local = true;
                }
            } else {
                new_flag_dirty4local = false;
            }
            // ***
            new_canals.push( MultiTraceCanal::new(canal_lfs,
                                                  new_trace,
                                                  new_flag_hidden,
                                                  new_flag_dirty4local,
                                                  new_consumed,
                                                  new_simu_before,
                                                  new_simu_after) );
        }
        return AnalysableMultiTrace::new(new_canals,remaining_loop_instantiations_in_simulation);
    }

    pub fn update_on_hide(&self, lfs_to_hide : &HashSet<usize>) -> AnalysableMultiTrace {
        let mut new_canals : Vec<MultiTraceCanal> = Vec::new();
        for canal in &self.canals {
            if canal.lifelines.is_subset( lfs_to_hide ) {
                new_canals.push(MultiTraceCanal::new(canal.lifelines.clone(),
                                                     canal.trace.clone(),
                                                     true,
                                                     canal.flag_dirty4local,
                                                     canal.consumed,
                                                     canal.simulated_before,
                                                     canal.simulated_after));
            } else {
                new_canals.push(MultiTraceCanal::new(canal.lifelines.clone(),
                                                     canal.trace.clone(),
                                                     canal.flag_hidden,
                                                     canal.flag_dirty4local,
                                                     canal.consumed,
                                                     canal.simulated_before,
                                                     canal.simulated_after));
            }
        }
        return AnalysableMultiTrace::new(new_canals,0);
    }

    pub fn update_on_simulation(&self,
                                sim_map : &HashMap<usize,SimulationStepKind>,
                                target_lf_ids : &HashSet<usize>,
                                affected_lfs : &HashSet<usize>,
                                rem_sim_depth : u32) -> AnalysableMultiTrace {
        let mut new_canals : Vec<MultiTraceCanal> = Vec::new();
        let simulated_lfs : HashSet<usize> = sim_map.keys().cloned().collect();
        for canal in &self.canals {
            let canal_lfs = canal.lifelines.clone();
            let mut new_trace = canal.trace.clone();
            // ***
            let new_flag_dirty4local : bool;
            if canal_lfs.is_disjoint(affected_lfs) {
                new_flag_dirty4local = canal.flag_dirty4local;
            } else {
                new_flag_dirty4local = true;
            }
            // ***
            let new_flag_hidden = canal.flag_hidden;
            // ***
            // ***
            let mut new_consumed = canal.consumed;
            let mut new_simu_before = canal.simulated_before;
            let mut new_simu_after = canal.simulated_after;
            if !canal_lfs.is_disjoint(target_lf_ids) {
                if canal_lfs.is_disjoint(&simulated_lfs) {
                    new_trace.remove(0);
                    new_consumed = new_consumed + 1;
                } else {
                    for lf_id in &canal_lfs {
                        match sim_map.get(lf_id) {
                            None => {},
                            Some( sim_kind ) => {
                                match sim_kind {
                                    SimulationStepKind::BeforeStart => {
                                        new_simu_before += 1;
                                    },
                                    SimulationStepKind::AfterEnd => {
                                        new_simu_after += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // ***
            new_canals.push( MultiTraceCanal::new(canal_lfs,
                                                  new_trace,
                                                  new_flag_hidden,
                                                  new_flag_dirty4local,
                                                  new_consumed,
                                                  new_simu_before,
                                                  new_simu_after) );
        }
        return AnalysableMultiTrace::new(new_canals,rem_sim_depth);
    }
}

pub enum WasMultiTraceConsumedWithSimulation {
    No,
    OnlyAfterEnd,
    AsSlice
}