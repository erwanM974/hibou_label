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
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::syntax::action::{EmissionAction, EmissionTargetRef, ReceptionAction};

impl InvolvesLifelines for EmissionAction {

    fn involved_lifelines(&self) -> HashSet<usize> {
        let mut involved = HashSet::new();
        involved.insert( self.origin_lf_id );
        for target in &self.targets {
            match target {
                EmissionTargetRef::Lifeline(lf_id) => {
                    involved.insert( *lf_id );
                },
                _ => {}
            }
        }
        return involved;
    }
    fn involves_any_of(&self, lf_ids : &HashSet<usize>) -> bool {
        let occ = self.involved_lifelines();
        if occ.is_disjoint(lf_ids) {
            return false;
        } else {
            return true;
        }
    }
}


impl InvolvesLifelines for ReceptionAction {

    fn involved_lifelines(&self) -> HashSet<usize> {
        let mut involved = HashSet::new();
        for rcp_lf_id in &self.recipients {
            involved.insert( *rcp_lf_id );
        }
        return involved;
    }
    fn involves_any_of(&self, lf_ids : &HashSet<usize>) -> bool {
        let occ = self.involved_lifelines();
        if occ.is_disjoint(lf_ids) {
            return false;
        } else {
            return true;
        }
    }
}