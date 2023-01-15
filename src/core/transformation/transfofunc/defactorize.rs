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

pub fn transfo_defactorize_left(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::Strict( i1.clone(), i21.clone() );
                    let new_iB = Interaction::Strict( i1.clone(), i22.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::Seq( i1.clone(), i21.clone() );
                    let new_iB = Interaction::Seq( i1.clone(), i22.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::Par( i1.clone(), i21.clone() );
                    let new_iB = Interaction::Par( i1.clone(), i22.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::CoReg( cr.clone(), i1.clone(), i21.clone() );
                    let new_iB = Interaction::CoReg( cr.clone(), i1.clone(), i22.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}



pub fn transfo_defactorize_right(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::Strict( i11.clone(), i2.clone() );
                    let new_iB = Interaction::Strict( i12.clone(), i2.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::Seq( i11.clone(), i2.clone() );
                    let new_iB = Interaction::Seq( i12.clone(), i2.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::Par( i11.clone(), i2.clone() );
                    let new_iB = Interaction::Par( i12.clone(), i2.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::CoReg( cr.clone(), i11.clone(), i2.clone() );
                    let new_iB = Interaction::CoReg( cr.clone(), i12.clone(), i2.clone() );
                    return vec![ Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) ];
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}

