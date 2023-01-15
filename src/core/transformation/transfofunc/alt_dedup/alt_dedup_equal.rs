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
use itertools::Itertools;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::util::fold_recursive_frags::fold_recursive_alt_frags;
use crate::core::language::syntax::util::get_recursive_frag::get_recursive_alt_frags;

pub fn transfo_deduplicate(interaction : &Interaction) -> Vec<Interaction> {
    let orig_alt_frags = get_recursive_alt_frags(interaction);
    let as_set : HashSet<&Interaction> = HashSet::from_iter(orig_alt_frags.iter().cloned());
    if as_set.len() < orig_alt_frags.len() {
        let mut new_alt_frags : Vec<&Interaction> = as_set.into_iter().sorted().collect(); //
        return vec![fold_recursive_alt_frags(&mut new_alt_frags)];
    } else {
        return vec![];
    }
}
