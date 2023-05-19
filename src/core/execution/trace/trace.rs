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





use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, PartialOrd, Copy, Ord, Hash, Debug)]
pub enum TraceActionKind {
    Reception,
    Emission
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Copy, Ord, Hash, Debug)]
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

    pub fn get_actions_kinds(set_of_actions : &BTreeSet<TraceAction>) -> (i32,i32) {
        let mut num_emissions = 0;
        let mut num_receptions = 0;
        for tract in set_of_actions {
            match tract.act_kind {
                TraceActionKind::Emission => {
                    num_emissions += 1;
                },
                TraceActionKind::Reception => {
                    num_receptions += 1;
                }
            }
        }
        return (num_emissions,num_receptions);
    }
}


