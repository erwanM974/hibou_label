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
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::syntax::util::fold_recursive_frags::{fold_recursive_par_frags, fold_recursive_seq_frags, fold_recursive_strict_frags};
use crate::core::language::syntax::util::get_recursive_frag::{get_recursive_par_frags, get_recursive_seq_frags, get_recursive_strict_frags};




fn simpl_alt_with_loop(looped_frag : &Interaction,
                       sched_frags : Vec<&Interaction>) -> bool {
    let as_set : HashSet<&Interaction> = HashSet::from_iter(sched_frags.iter().cloned());
    if as_set.len() == 1 {
        let got_int : &Interaction = as_set.into_iter().next().unwrap();
        if got_int == looped_frag {
            return true;
        }
        if got_int == &Interaction::Empty {
            return true;
        }
    }
    return false;
}


pub fn loop_alt_simpl(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            match **i2 {
                Interaction::Loop(ref lk, ref i21) => {
                    match lk {
                        LoopKind::PInterleaving => {
                            let frags = get_recursive_par_frags(i1);
                            if simpl_alt_with_loop(i21,frags) {
                                return vec![*i2.clone()];
                            }
                        },
                        LoopKind::WWeakSeq => {
                            let frags = get_recursive_seq_frags(i1);
                            if simpl_alt_with_loop(i21,frags) {
                                return vec![*i2.clone()];
                            }
                        },
                        LoopKind::SStrictSeq => {
                            let frags = get_recursive_strict_frags(i1);
                            if simpl_alt_with_loop(i21,frags) {
                                return vec![*i2.clone()];
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}
