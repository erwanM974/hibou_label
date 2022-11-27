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

pub fn merge_shift_left_1(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::And(ref i1, ref i2) => {
            match **i1 {
                Interaction::Strict(ref i11, ref i12) => {
                    let new_left = Interaction::And( i11.clone(), i2.clone() );
                    return Some(Interaction::Strict(Box::new(new_left), i12.clone() ));
                },
                Interaction::Seq(ref i11, ref i12) => {
                    let new_left = Interaction::And( i11.clone(), i2.clone() );
                    return Some(Interaction::Seq(Box::new(new_left), i12.clone() ));
                },
                Interaction::CoReg(ref cr, ref i11, ref i12) => {
                    let new_left = Interaction::And( i11.clone(), i2.clone() );
                    return Some(Interaction::CoReg(cr.clone(),Box::new(new_left), i12.clone() ));
                },
                Interaction::Par(ref i11, ref i12) => {
                    let new_left = Interaction::And( i11.clone(), i2.clone() );
                    return Some(Interaction::Par(Box::new(new_left), i12.clone() ));
                },
                Interaction::Alt(ref i11, ref i12) => {
                    let new_left = Interaction::And( i11.clone(), i2.clone() );
                    return Some(Interaction::Alt(Box::new(new_left), i12.clone() ));
                },
                Interaction::Loop(ref lk, ref i11) => {
                    let new_sub = Interaction::And( i11.clone(), i2.clone() );
                    return Some(Interaction::Loop(lk.clone(), Box::new(new_sub) ));
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}



pub fn merge_shift_left_2(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::And(ref i1, ref i2) => {
            match **i1 {
                Interaction::Strict(ref i11, ref i12) => {
                    let new_right = Interaction::And( i12.clone(), i2.clone() );
                    return Some(Interaction::Strict( i11.clone(), Box::new(new_right) ));
                },
                Interaction::Seq(ref i11, ref i12) => {
                    let new_right = Interaction::And( i12.clone(), i2.clone() );
                    return Some(Interaction::Seq( i11.clone(), Box::new(new_right) ));
                },
                Interaction::CoReg(ref cr, ref i11, ref i12) => {
                    let new_right = Interaction::And( i12.clone(), i2.clone() );
                    return Some(Interaction::CoReg( cr.clone(),i11.clone(), Box::new(new_right) ));
                },
                Interaction::Par(ref i11, ref i12) => {
                    let new_right = Interaction::And( i12.clone(), i2.clone() );
                    return Some(Interaction::Par( i11.clone(), Box::new(new_right) ));
                },
                Interaction::Alt(ref i11, ref i12) => {
                    let new_right = Interaction::And( i12.clone(), i2.clone() );
                    return Some(Interaction::Alt( i11.clone(), Box::new(new_right) ));
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}



