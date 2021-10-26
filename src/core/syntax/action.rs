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

use std::fmt::Debug;
use std::collections::HashSet;

use crate::core::trace::{TraceActionKind,TraceAction};

#[derive(PartialOrd, Ord, Clone, PartialEq, Debug, Eq, Hash)]
pub enum EmissionTargetRef {
    Lifeline(usize),
    Gate(usize)
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum ObservableActionKind {
    Reception(Option<usize>),
    Emission(Vec<EmissionTargetRef>)
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct ObservableAction {
    pub lf_id : usize,
    pub act_kind : ObservableActionKind,
    pub ms_id : usize
}


impl ObservableAction {

    pub fn get_all_atomic_actions(&self) -> HashSet<TraceAction> {
        let mut contents : HashSet<TraceAction> = HashSet::new();
        match &self.act_kind {
            ObservableActionKind::Reception(_) => {
                contents.insert( TraceAction::new(self.lf_id,TraceActionKind::Reception,self.ms_id) );
            },
            ObservableActionKind::Emission( ref targets ) => {
                contents.insert( TraceAction::new(self.lf_id,TraceActionKind::Emission,self.ms_id) );
                for target_ref in targets {
                    match target_ref {
                        &EmissionTargetRef::Lifeline(tar_lf_id) => {
                            contents.insert( TraceAction::new(tar_lf_id,TraceActionKind::Reception, self.ms_id) );
                        },
                        _ => {}
                    }
                }
            }
        }
        return contents;
    }

    pub fn to_trace_action(&self) -> TraceAction {
        match self.act_kind {
            ObservableActionKind::Reception(_) => {
                return TraceAction::new(self.lf_id,TraceActionKind::Reception,self.ms_id);
            },
            ObservableActionKind::Emission(_) => {
                return TraceAction::new(self.lf_id,TraceActionKind::Emission,self.ms_id);
            }
        }
    }

    pub fn get_action_kind(&self) -> TraceActionKind {
        match self.act_kind {
            ObservableActionKind::Reception(_) => {
                return TraceActionKind::Reception;
            },
            ObservableActionKind::Emission(_) => {
                return TraceActionKind::Emission;
            }
        }
    }

    pub fn occupation_after(&self) -> HashSet<usize> {
        match self.act_kind {
            ObservableActionKind::Emission(ref targets) => {
                let mut occ : HashSet<usize> = HashSet::new();
                for target_ref in targets {
                    match target_ref {
                        &EmissionTargetRef::Lifeline(tar_lf_id) => {
                            occ.insert( tar_lf_id );
                        },
                        _ => {}
                    }
                }
                occ.insert( self.lf_id );
                return occ;
            },
            _ => {
                let mut occ : HashSet<usize> = HashSet::new();
                occ.insert( self.lf_id );
                return occ;
            }
        }
    }
    
}

