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
use crate::core::language::syntax::util::fold_recursive_frags::*;
use crate::core::language::syntax::util::get_recursive_frag::*;



pub fn transfo_factorize_suffix_strict(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_strict_frags = get_recursive_strict_frags(i1);
            let mut right_strict_frags = get_recursive_strict_frags(i2);
            if left_strict_frags.last() == right_strict_frags.last() {
                let last_frag : &Interaction = left_strict_frags.pop().unwrap();
                right_strict_frags.pop();
                if last_frag != &Interaction::Empty {
                    let new_alt = Interaction::Alt(Box::new(fold_recursive_strict_frags(&mut left_strict_frags)),
                                                   Box::new(fold_recursive_strict_frags(&mut right_strict_frags))
                    );
                    return vec![Interaction::Strict( Box::new(new_alt), Box::new(last_frag.clone()) )];
                }
            }
        },
        _ => {}
    }
    return vec![];
}

pub fn transfo_factorize_suffix_seq(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_seq_frags = get_recursive_seq_frags(i1);
            let mut right_seq_frags = get_recursive_seq_frags(i2);
            if left_seq_frags.last() == right_seq_frags.last() {
                let last_frag : &Interaction = left_seq_frags.pop().unwrap();
                right_seq_frags.pop();
                if last_frag != &Interaction::Empty {
                    let new_alt = Interaction::Alt(Box::new(fold_recursive_seq_frags(&mut left_seq_frags)),
                                                   Box::new(fold_recursive_seq_frags(&mut right_seq_frags))
                    );
                    return vec![Interaction::Seq( Box::new(new_alt), Box::new(last_frag.clone()) )];
                }
            }
        },
        _ => {}
    }
    return vec![];
}

