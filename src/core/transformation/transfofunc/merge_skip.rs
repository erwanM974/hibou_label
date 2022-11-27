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

pub fn merge_skip(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::And(ref i1, ref i2) => {
            match (&**i1,&**i2) {
                (Interaction::Strict(ref i11, ref i12),Interaction::Strict(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    return Some(Interaction::Strict(Box::new(new_left), Box::new(new_right) ));
                },
                (Interaction::Seq(ref i11, ref i12),Interaction::Seq(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    return Some(Interaction::Seq(Box::new(new_left), Box::new(new_right) ));
                },
                (Interaction::CoReg(ref cr1, ref i11, ref i12),Interaction::CoReg(ref cr2,ref i21, ref i22)) => {
                    if cr1 == cr2 {
                        let new_left = Interaction::And( i11.clone(), i21.clone() );
                        let new_right = Interaction::And( i12.clone(), i22.clone() );
                        return Some(Interaction::CoReg(cr1.clone(), Box::new(new_left), Box::new(new_right) ));
                    }
                },
                (Interaction::Par(ref i11, ref i12),Interaction::Par(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    return Some(Interaction::Par(Box::new(new_left), Box::new(new_right) ));
                },
                (Interaction::Alt(ref i11, ref i12),Interaction::Alt(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    return Some(Interaction::Alt(Box::new(new_left), Box::new(new_right) ));
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}