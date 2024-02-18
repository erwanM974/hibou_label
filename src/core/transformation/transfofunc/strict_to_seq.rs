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
use crate::core::execution::semantics::frontier::global_frontier;
use crate::core::language::syntax::interaction::Interaction;

pub fn transfo_strict_to_seq(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            let last_actions_of_i1 = global_frontier(&i1.reverse());
            let last_locations_on_i1 : HashSet<usize> = last_actions_of_i1.iter()
                .fold( HashSet::new(),|mut p, x| {p.extend(x.target_lf_ids.clone()); p});
            if last_locations_on_i1.len() == 1 {
                let first_actions_of_i2 = global_frontier(i2);
                let first_locatins_on_i2 = first_actions_of_i2.iter()
                    .fold( HashSet::new(),|mut p, x| {p.extend(x.target_lf_ids.clone()); p});
                if last_locations_on_i1 == first_locatins_on_i2 {
                    return vec![Interaction::Seq(i1.clone(),i2.clone())];
                }
            }
        },
        _ => {}
    }
    return vec![];
}