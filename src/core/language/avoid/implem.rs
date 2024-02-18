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




use std::collections::{BTreeSet, HashSet};
use crate::core::language::avoid::avoids::AvoidsLifelines;
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::syntax::action::{EmissionAction, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;


impl AvoidsLifelines for EmissionAction {
    fn avoids_all_of(&self, lf_ids: &BTreeSet<usize>) -> bool {
        return self.involved_lifelines().is_disjoint(lf_ids);
    }
}


impl AvoidsLifelines for ReceptionAction {
    fn avoids_all_of(&self, lf_ids: &BTreeSet<usize>) -> bool {
        return self.involved_lifelines().is_disjoint(lf_ids);
    }
}


impl AvoidsLifelines for Interaction {
    fn avoids_all_of(&self, lf_ids: &BTreeSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.avoids_all_of(lf_ids);
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.avoids_all_of(lf_ids);
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::Par(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) || i2.avoids_all_of(lf_ids);
            },
            &Interaction::Loop(_, _) => {
                return true;
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }
}