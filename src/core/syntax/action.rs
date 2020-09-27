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

use crate::core::trace::TraceActionKind;

#[derive(Clone, PartialEq, Debug)]
pub enum ObservableActionKind {
    Reception,
    Emission(Vec<usize>)
}

#[derive(Clone, PartialEq, Debug)]
pub struct ObservableAction {
    pub lf_id : usize,
    pub act_kind : ObservableActionKind,
    pub ms_id : usize
}


impl ObservableAction {

    pub fn get_action_kind(&self) -> TraceActionKind {
        match self.act_kind {
            ObservableActionKind::Reception => {
                return TraceActionKind::Reception;
            },
            ObservableActionKind::Emission(_) => {
                return TraceActionKind::Emission;
            }
        }
    }

    pub fn occupation_before(&self) -> usize {
        return self.lf_id;
    }

    pub fn occupation_after(&self) -> HashSet<usize> {
        match self.act_kind {
            ObservableActionKind::Emission(ref targets) => {
                let mut occ : HashSet<usize> = HashSet::new();
                for lf_id in targets {
                    occ.insert( *lf_id );
                }
                occ.insert( self.occupation_before() );
                return occ;
            },
            _ => {
                let mut occ : HashSet<usize> = HashSet::new();
                occ.insert( self.occupation_before() );
                return occ;
            }
        }
    }
    
}

