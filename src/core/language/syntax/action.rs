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



#[derive(PartialOrd, Ord, Clone, PartialEq, Debug, Eq, Hash)]
pub enum EmissionTargetRef {
    Lifeline(usize),
    Gate(usize)
}

#[derive(PartialOrd, Ord, Clone, PartialEq, Debug, Eq, Hash)]
pub enum CommunicationSynchronicity {
    Asynchronous,
    Synchronous
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct EmissionAction {
    pub origin_lf_id : usize,
    pub ms_id : usize,
    pub synchronicity : CommunicationSynchronicity,
    pub targets : Vec<EmissionTargetRef> // both lf_ids and gt_ids possible
}

impl EmissionAction {
    pub fn new(origin_lf_id : usize,
               ms_id : usize,
               synchronicity : CommunicationSynchronicity,
               targets : Vec<EmissionTargetRef>) -> EmissionAction {
        return EmissionAction{origin_lf_id,ms_id,synchronicity,targets}
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct ReceptionAction {
    pub origin_gt_id : Option<usize>,
    pub ms_id : usize,
    pub synchronicity : CommunicationSynchronicity,
    pub recipients : Vec<usize> // only lf_ids here
}

impl ReceptionAction {
    pub fn new(origin_gt_id : Option<usize>,
               ms_id : usize,
               synchronicity : CommunicationSynchronicity,
               recipients : Vec<usize>) -> ReceptionAction {
        return ReceptionAction{origin_gt_id,ms_id,synchronicity,recipients}
    }
}

