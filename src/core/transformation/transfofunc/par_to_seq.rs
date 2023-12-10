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
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::syntax::interaction::Interaction;

pub fn transfo_par_to_seq(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Par(ref i1, ref i2) => {
            let i1_lfs = i1.involved_lifelines();
            let i2_lfs = i2.involved_lifelines();
            let intersect_lfs : HashSet<usize> = i1_lfs.intersection(&i2_lfs).into_iter().cloned().collect();
            if intersect_lfs.is_empty() {
                return vec![Interaction::Seq(i1.clone(),i2.clone())];
            }
        },
        _ => {}
    }
    return vec![];
}