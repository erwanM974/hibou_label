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

pub fn transfo_merge_shift_right_1(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::And(ref i1, ref i2) => {
            match **i2 {
                Interaction::Strict(ref i21, ref i22) => {
                    let new_left = Interaction::And( i1.clone(), i21.clone() );
                    return Some(Interaction::Strict(Box::new(new_left), i22.clone() ));
                },
                Interaction::Seq(ref i21, ref i22) => {
                    let new_left = Interaction::And( i1.clone(), i21.clone() );
                    return Some(Interaction::Seq(Box::new(new_left), i22.clone() ));
                },
                Interaction::CoReg(ref cr, ref i21, ref i22) => {
                    let new_left = Interaction::And( i1.clone(), i21.clone() );
                    return Some(Interaction::CoReg(cr.clone(),Box::new(new_left), i22.clone() ));
                },
                Interaction::Par(ref i21, ref i22) => {
                    let new_left = Interaction::And( i1.clone(), i21.clone() );
                    return Some(Interaction::Par(Box::new(new_left), i22.clone() ));
                },
                Interaction::Alt(ref i21, ref i22) => {
                    let new_left = Interaction::And( i1.clone(), i21.clone() );
                    return Some(Interaction::Alt(Box::new(new_left), i22.clone() ));
                },
                Interaction::Loop(ref lk, ref i21) => {
                    let new_sub = Interaction::And( i1.clone(), i21.clone() );
                    return Some(Interaction::Loop(lk.clone(), Box::new(new_sub) ));
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}



pub fn transfo_merge_shift_right_2(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::And(ref i1, ref i2) => {
            match **i2 {
                Interaction::Strict(ref i21, ref i22) => {
                    let new_right = Interaction::And( i1.clone(), i22.clone() );
                    return Some(Interaction::Strict( i21.clone(), Box::new(new_right) ));
                },
                Interaction::Seq(ref i21, ref i22) => {
                    let new_right = Interaction::And( i1.clone(), i22.clone() );
                    return Some(Interaction::Seq( i21.clone(), Box::new(new_right) ));
                },
                Interaction::CoReg(ref cr, ref i21, ref i22) => {
                    let new_right = Interaction::And( i1.clone(), i22.clone() );
                    return Some(Interaction::CoReg( cr.clone(),i21.clone(), Box::new(new_right) ));
                },
                Interaction::Par(ref i21, ref i22) => {
                    let new_right = Interaction::And( i1.clone(), i22.clone() );
                    return Some(Interaction::Par( i21.clone(), Box::new(new_right) ));
                },
                Interaction::Alt(ref i21, ref i22) => {
                    let new_right = Interaction::And( i1.clone(), i22.clone() );
                    return Some(Interaction::Alt( i21.clone(), Box::new(new_right) ));
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}



