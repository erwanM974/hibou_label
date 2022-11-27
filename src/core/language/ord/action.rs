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




use std::cmp::Ordering;
use crate::core::language::syntax::action::{EmissionAction, EmissionTargetRef, ReceptionAction};



impl Ord for EmissionTargetRef {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (EmissionTargetRef::Lifeline(self_lf),EmissionTargetRef::Lifeline(other_lf)) => {
                return self_lf.cmp(other_lf);
            },
            (EmissionTargetRef::Lifeline(_),EmissionTargetRef::Gate(_)) => {
                return Ordering::Less;
            },
            (EmissionTargetRef::Gate(_),EmissionTargetRef::Lifeline(_)) => {
                return Ordering::Greater;
            },
            (EmissionTargetRef::Gate(self_gt),EmissionTargetRef::Gate(other_gt)) => {
                return self_gt.cmp(other_gt);
            }
        }
    }
}



impl Ord for EmissionAction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ms_id < other.ms_id {
            return Ordering::Less;
        }
        if self.ms_id > other.ms_id {
            return Ordering::Greater;
        }
        // ***
        if self.origin_lf_id < other.origin_lf_id {
            return Ordering::Less;
        }
        if self.origin_lf_id > other.origin_lf_id {
            return Ordering::Greater;
        }
        // ***
        let max_tar_len = self.targets.len().max(other.targets.len());
        for i in 0..max_tar_len {
            match (self.targets.get(i) , other.targets.get(i) ) {
                ( Some( tar_ref1 ), Some(tar_ref2) ) => {
                    if tar_ref1 < tar_ref2 {
                        return Ordering::Less;
                    }
                    if tar_ref1 > tar_ref2 {
                        return Ordering::Greater;
                    }
                },
                (None,Some(_)) => {
                    return Ordering::Less;
                },
                (Some(_),None) => {
                    return Ordering::Greater;
                },
                (None,None) => {}
            }
        }
        // ***
        if self.synchronicity < other.synchronicity {
            return Ordering::Less;
        }
        if self.synchronicity > other.synchronicity {
            return Ordering::Greater;
        }
        // ***
        return Ordering::Equal;
    }
}



impl Ord for ReceptionAction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ms_id < other.ms_id {
            return Ordering::Less;
        }
        if self.ms_id > other.ms_id {
            return Ordering::Greater;
        }
        // ***
        match (self.origin_gt_id,other.origin_gt_id) {
            (None,Some(_)) => {
                return Ordering::Less;
            },
            (Some(_),None) => {
                return Ordering::Greater;
            },
            (None,None) => {},
            (Some( gt_id1),Some(gt_id2)) => {
                if gt_id1 < gt_id2 {
                    return Ordering::Less;
                }
                if gt_id1 > gt_id2 {
                    return Ordering::Greater;
                }
            }
        }
        // ***
        let max_tar_len = self.recipients.len().max(other.recipients.len());
        for i in 0..max_tar_len {
            match (self.recipients.get(i) , other.recipients.get(i) ) {
                ( Some( tar_lf_id1 ), Some(tar_lf_id2) ) => {
                    if tar_lf_id1 < tar_lf_id2 {
                        return Ordering::Less;
                    }
                    if tar_lf_id1 > tar_lf_id2 {
                        return Ordering::Greater;
                    }
                },
                (None,Some(_)) => {
                    return Ordering::Less;
                },
                (Some(_),None) => {
                    return Ordering::Greater;
                },
                (None,None) => {}
            }
        }
        // ***
        if self.synchronicity < other.synchronicity {
            return Ordering::Less;
        }
        if self.synchronicity > other.synchronicity {
            return Ordering::Greater;
        }
        // ***
        return Ordering::Equal;
    }
}






