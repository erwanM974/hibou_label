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





use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::util::fold_recursive_frags::{fold_recursive_coreg_frags, fold_recursive_par_frags, fold_recursive_seq_frags, fold_recursive_strict_frags};
use crate::core::language::syntax::util::get_recursive_frag::{get_recursive_coreg_frags, get_recursive_par_frags, get_recursive_seq_frags, get_recursive_strict_frags};


pub fn transfo_simpl(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            let mut old_frags = get_recursive_strict_frags(i1);
            old_frags.extend(get_recursive_strict_frags(i2));
            // ***
            let old_len = old_frags.len();
            // ***
            let mut new_frags : Vec<&Interaction> = old_frags.into_iter().filter(|x| *x != &Interaction::Empty).collect();
            if new_frags.len() < old_len {
                return vec![fold_recursive_strict_frags(&mut new_frags)];
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            let mut old_frags = get_recursive_seq_frags(i1);
            old_frags.extend(get_recursive_seq_frags(i2));
            // ***
            let old_len = old_frags.len();
            // ***
            let mut new_frags : Vec<&Interaction> = old_frags.into_iter().filter(|x| *x != &Interaction::Empty).collect();
            if new_frags.len() < old_len {
                return vec![fold_recursive_seq_frags(&mut new_frags)];
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut old_frags = get_recursive_par_frags(i1);
            old_frags.extend(get_recursive_par_frags(i2));
            // ***
            let old_len = old_frags.len();
            // ***
            let mut new_frags : Vec<&Interaction> = old_frags.into_iter().filter(|x| *x != &Interaction::Empty).collect();
            if new_frags.len() < old_len {
                return vec![fold_recursive_par_frags(&mut new_frags)];
            }
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let mut old_frags = get_recursive_coreg_frags(cr,i1);
            old_frags.extend(get_recursive_coreg_frags(cr,i2));
            // ***
            let old_len = old_frags.len();
            // ***
            let mut new_frags : Vec<&Interaction> = old_frags.into_iter().filter(|x| *x != &Interaction::Empty).collect();
            if new_frags.len() < old_len {
                return vec![fold_recursive_coreg_frags(cr,&mut new_frags)];
            }
        },
        _ => {}
    }
    return vec![];
}


