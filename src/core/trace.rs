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
    pub trace : Vec<TraceAction>
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnalysableMultiTrace {
    pub canals : Vec<MultiTraceCanal>
}

impl AnalysableMultiTrace {
    pub fn new(canals:Vec<MultiTraceCanal>) -> AnalysableMultiTrace {
        return AnalysableMultiTrace{canals};
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

}

