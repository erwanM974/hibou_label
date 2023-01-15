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





use itertools::Itertools;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::util::fold_recursive_frags::{fold_recursive_alt_frags, fold_recursive_par_frags};
use crate::core::language::syntax::util::get_recursive_frag::{get_recursive_alt_frags, get_recursive_par_frags};

pub fn transfo_invert_alt_sorted(interaction : &Interaction) -> Vec<Interaction> {
    let orig_alt_frags = get_recursive_alt_frags(interaction);
    let mut sorted_alt_frags : Vec<&Interaction> = orig_alt_frags.iter().map(|x| *x).sorted().collect();
    if sorted_alt_frags != orig_alt_frags {
        return vec![fold_recursive_alt_frags(&mut sorted_alt_frags)];
    } else {
        return vec![];
    }
}

pub fn transfo_invert_par_sorted(interaction : &Interaction) -> Vec<Interaction> {
    let orig_par_frags = get_recursive_par_frags(interaction);
    let mut sorted_par_frags : Vec<&Interaction> = orig_par_frags.iter().map(|x| *x).sorted().collect();
    if sorted_par_frags != orig_par_frags {
        return vec![fold_recursive_par_frags(&mut sorted_par_frags)];
    } else {
        return vec![];
    }
}

/*
pub fn tri_invert_alt_conditional_right_flushed(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i_right) => {
            match **i_right {
                Interaction::Alt(ref i2,ref i3) => {
                    if i2 < i1 {
                        return vec![ Interaction::Alt( i2.clone(), Box::new(Interaction::Alt(i1.clone(), i3.clone())) ) ];
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}

pub fn tri_invert_par_conditional_right_flushed(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Par(ref i1, ref i_right) => {
            match **i_right {
                Interaction::Par(ref i2,ref i3) => {
                    if i2 < i1 {
                        return vec![Interaction::Par(i2.clone(), Box::new(Interaction::Par(i1.clone(), i3.clone())))];
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}*/