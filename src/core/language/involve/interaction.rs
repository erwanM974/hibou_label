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
use crate::core::language::syntax::interaction::Interaction;





impl InvolvesLifelines for Interaction {
    fn involved_lifelines(&self) -> HashSet<usize> {
        match &self {
            &Interaction::Empty => {
                return HashSet::new();
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.involved_lifelines();
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.involved_lifelines();
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Seq(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Par(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Loop(_, i1) => {
                return i1.involved_lifelines();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    fn involves_any_of(&self, lf_ids : &HashSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                return false;
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.involves_any_of(lf_ids);
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.involves_any_of(lf_ids);
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Par(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Loop(_, ref i1) => {
                return i1.involves_any_of(lf_ids);
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }
}