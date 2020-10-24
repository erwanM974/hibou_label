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


use std::collections::HashSet;
use crate::core::syntax::action::*;


#[derive(Clone, PartialEq, Debug)]
pub enum TraceActionKind {
    Reception,
    Emission
}

#[derive(Clone, PartialEq, Debug)] //Eq, Hash,
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

    pub fn is_match(&self,model_action: &ObservableAction) -> bool {
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
    }
}


#[derive(Clone, PartialEq, Debug)]
pub struct MultiTraceCanal {
    pub lifelines : HashSet<usize>,
    pub trace : Vec<TraceAction>,
    pub flag_hidden : bool,
    pub consumed : u32,
    pub simulated_before : u32,
    pub simulated_after : u32
}

impl MultiTraceCanal {
    pub fn new(lifelines : HashSet<usize>,
               trace : Vec<TraceAction>,
               flag_hidden : bool,
               consumed : u32,
               simulated_before : u32,
               simulated_after : u32) -> MultiTraceCanal {
        return MultiTraceCanal{lifelines,trace,flag_hidden,consumed,simulated_before,simulated_after};
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
}

pub enum WasMultiTraceConsumedWithSimulation {
    No,
    OnlyAfterEnd,
    AsSlice
}