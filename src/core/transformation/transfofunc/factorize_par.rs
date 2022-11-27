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



pub fn factorize_par(interaction : &Interaction) -> Vec<Interaction> {
    let mut got_ints = vec![];
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let left_par_frags = get_recursive_par_frags(i1);
            let right_par_frags = get_recursive_par_frags(i2);
            // ***
            for (left_frag_id,left_frag) in left_par_frags.iter().enumerate() {
                for (right_frag_id,right_frag) in right_par_frags.iter().enumerate() {
                    if left_frag == right_frag {
                        let mut new_left_par_frags = left_par_frags.clone();
                        new_left_par_frags.remove(left_frag_id);
                        let mut new_right_par_frags = right_par_frags.clone();
                        new_right_par_frags.remove(right_frag_id);
                        // ***
                        let new_alt = Interaction::Alt(Box::new(fold_recursive_par_frags(&mut new_left_par_frags)),
                                                       Box::new(fold_recursive_par_frags(&mut new_right_par_frags))
                        );
                        if *left_frag != &Interaction::Empty {
                            got_ints.push( Interaction::Par( Box::new((*left_frag).clone()), Box::new(new_alt)) )
                        } else {
                            got_ints.push(new_alt);
                        }
                    }
                }
            }
        },
        _ => {}
    }
    return got_ints;
}
