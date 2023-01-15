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
use crate::core::language::include::includes::PartialSemanticInclusion;
use crate::core::language::syntax::action::{EmissionAction, EmissionTargetRef};


impl PartialSemanticInclusion for EmissionAction {

    fn is_included(&self, other: &Self) -> bool {
        if self.ms_id != other.ms_id {
            return false;
        }
        if self.origin_lf_id != other.origin_lf_id {
            return false;
        }
        // ***
        let self_target_lfs : Vec<usize> = self.targets.iter().filter_map(|x| match x {
            EmissionTargetRef::Lifeline(x_lf) => {Some(x_lf)},
            _ => {None}
        }).collect();
        let other_target_lfs : Vec<usize> = other.targets.iter().filter_map(|x| match x {
            EmissionTargetRef::Lifeline(x_lf) => {Some(x_lf)},
            _ => {None}
        }).collect();
        // ***
        let self_target_lfs_as_set : HashSet<usize> = HashSet::from_iter(self_target_lfs.into_iter());
        let other_target_lfs_as_set : HashSet<usize> = HashSet::from_iter(other_target_lfs.into_iter());
        if !other_target_lfs_as_set.is_superset(&self_target_lfs_as_set) {
            return false;
        }
        // ***
        if self_target_lfs_as_set.len() > 0 {
            if self.synchronicity != other.synchronicity {
                return false;
            }
        }
        // ***
        return true;
    }

}

