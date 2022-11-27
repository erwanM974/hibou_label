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



pub fn factorize_prefix_strict(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_strict_frags = get_recursive_strict_frags(i1);
            // ***
            match **i2 {
                Interaction::Alt(ref i21,ref i22) => {
                    let mut right_strict_frags = get_recursive_strict_frags(i21);
                    if left_strict_frags[0] == right_strict_frags[0] {
                        let first_frag = left_strict_frags.remove(0);
                        right_strict_frags.remove(0);
                        if first_frag != &Interaction::Empty {
                            let inner_alt = Interaction::Alt(Box::new(fold_recursive_strict_frags(&mut left_strict_frags)),
                                                           Box::new(fold_recursive_strict_frags(&mut right_strict_frags))
                            );
                            let inner_strict = Interaction::Strict(Box::new(first_frag.clone()), Box::new(inner_alt));
                            // ***
                            return vec![Interaction::Alt( Box::new(inner_strict), i22.clone())];
                        }
                    }
                },
                _ => {
                    let mut right_strict_frags = get_recursive_strict_frags(i2);
                    if left_strict_frags[0] == right_strict_frags[0] {
                        let first_frag = left_strict_frags.remove(0);
                        right_strict_frags.remove(0);
                        if first_frag != &Interaction::Empty {
                            let new_alt = Interaction::Alt(Box::new(fold_recursive_strict_frags(&mut left_strict_frags)),
                                                           Box::new(fold_recursive_strict_frags(&mut right_strict_frags))
                            );
                            return vec![Interaction::Strict( Box::new(first_frag.clone()), Box::new(new_alt))];
                        }
                    }
                }
            }
        },
        _ => {}
    }
    return vec![];
}

pub fn factorize_prefix_seq(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_seq_frags = get_recursive_seq_frags(i1);
            // ***
            match **i2 {
                Interaction::Alt(ref i21,ref i22) => {
                    let mut right_seq_frags = get_recursive_seq_frags(i21);
                    if left_seq_frags[0] == right_seq_frags[0] {
                        let first_frag = left_seq_frags.remove(0);
                        right_seq_frags.remove(0);
                        if first_frag != &Interaction::Empty {
                            let inner_alt = Interaction::Alt(Box::new(fold_recursive_seq_frags(&mut left_seq_frags)),
                                                             Box::new(fold_recursive_seq_frags(&mut right_seq_frags))
                            );
                            let inner_seq = Interaction::Seq(Box::new(first_frag.clone()), Box::new(inner_alt));
                            // ***
                            return vec![Interaction::Alt( Box::new(inner_seq), i22.clone())];
                        }
                    }
                },
                _ => {
                    let mut right_seq_frags = get_recursive_seq_frags(i2);
                    if left_seq_frags[0] == right_seq_frags[0] {
                        let first_frag = left_seq_frags.remove(0);
                        right_seq_frags.remove(0);
                        if first_frag != &Interaction::Empty {
                            let new_alt = Interaction::Alt(Box::new(fold_recursive_seq_frags(&mut left_seq_frags)),
                                                           Box::new(fold_recursive_seq_frags(&mut right_seq_frags))
                            );
                            return vec![Interaction::Seq( Box::new(first_frag.clone()), Box::new(new_alt))];
                        }
                    }
                }
            }
        },
        _ => {}
    }
    return vec![];
}



