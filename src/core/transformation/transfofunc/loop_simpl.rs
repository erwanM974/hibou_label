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



use crate::core::language::syntax::interaction::{Interaction};

pub fn transfo_loop_empty_simpl(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Loop(ref sk, ref i1) => {
            match **i1 {
                Interaction::Empty => {
                    return vec![Interaction::Empty];
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}


pub fn transfo_loop_unnest(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Loop(ref lkA, ref i1) => {
            match **i1 {
                Interaction::Loop(ref lkB, ref i11) => {
                    return vec![Interaction::Loop((lkA.min(lkB)).clone(), i11.clone())];
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}






