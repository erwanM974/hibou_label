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
    pub fn is_match(&self,model_action: &ObservableAction) -> bool {
        if model_action.lf_id != self.lf_id {
            return false;
        }
        if model_action.ms_id != self.ms_id {
            return false;
        }
        match &self.act_kind {
            &TraceActionKind::Emission => {
                match model_action.act_kind {
                    ObservableActionKind::Emission(_) => {
                        return true;
                    },
                    _ => {
                        return false;
                    }
                }
            },
            &TraceActionKind::Reception => {
                match model_action.act_kind {
                    ObservableActionKind::Reception => {
                        return true;
                    },
                    _ => {
                        return false;
                    }
                }
            }
        }
    }
}


#[derive(Clone, PartialEq, Debug)] //Eq, Hash,
pub struct MultiTraceCanal {
    pub lifelines : HashSet<usize>,
    pub trace : Vec<TraceAction>
}

pub type AnalysableMultiTrace = Vec<MultiTraceCanal>;

pub fn multitrace_length(multitrace:&AnalysableMultiTrace) -> usize {
    let mut length = 0;
    for canal in multitrace {
        length = length + (canal.trace.len());
    }
    return length;
}

