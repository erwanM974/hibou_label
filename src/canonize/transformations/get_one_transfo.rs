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




use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::general_context::GeneralContext;

use crate::rendering::textual::monochrome::position::position_to_text;

use crate::canonize::transformations::transfokind::*;
use crate::canonize::transformations::transfodef::*;

use crate::canonize::transformations::phases::*;


pub fn phase_1_one_transfo(interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    match get_one_transformation_rec(&transfos_phase1(),interaction) {
        Some( got_transfo ) => {
            return vec![got_transfo];
        },
        None => {
            return Vec::new();
        }
    }
}

pub fn phase_2_one_transfo(interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    match get_one_transformation_rec(&transfos_phase2(),interaction) {
        Some( got_transfo ) => {
            return vec![got_transfo];
        },
        None => {
            return Vec::new();
        }
    }
}

fn get_one_transformation_inner(transfos : &Vec<(TransformationKind, &dyn Fn(&Interaction) -> Option<Interaction>)>,
                                 interaction : &Interaction) -> Option<InteractionTermTransformation> {
    for (transfo_kind, transfo_func) in transfos {
        match transfo_func(interaction) {
            None => {},
            Some(new_int) => {
                return Some( InteractionTermTransformation::new((*transfo_kind).clone(),Position::Epsilon,new_int) );
            }
        }
    }
    return None;
}

fn get_one_transformation_rec(transfos : &Vec<(TransformationKind, &dyn Fn(&Interaction) -> Option<Interaction>)>,
                               interaction : &Interaction) -> Option<InteractionTermTransformation> {
    match get_one_transformation_inner(transfos,interaction) {
        Some( got_transfo ) => {
            return Some(got_transfo);
        },
        None => {
            match interaction {
                &Interaction::Empty => {
                    // ***
                }, &Interaction::Action(_) => {
                    // ***
                }, &Interaction::Strict(ref i1, ref i2) => {
                    match get_one_transformation_rec(transfos,i1) {
                        Some(left_transfo) => {
                            return Some( InteractionTermTransformation::new(left_transfo.kind,
                                                                            Position::Left(Box::new(left_transfo.position)),
                                                                            Interaction::Strict(Box::new(left_transfo.result),i2.clone())) );
                        },
                        None => {}
                    }
                    match get_one_transformation_rec(transfos,i2) {
                        Some(right_transfo) => {
                            return Some( InteractionTermTransformation::new(right_transfo.kind,
                                                                            Position::Right(Box::new(right_transfo.position)),
                                                                            Interaction::Strict(i1.clone(), Box::new(right_transfo.result))) );
                        },
                        None => {}
                    }
                }, &Interaction::Seq(ref i1, ref i2) => {
                    match get_one_transformation_rec(transfos,i1) {
                        Some(left_transfo) => {
                            return Some( InteractionTermTransformation::new(left_transfo.kind,
                                                                            Position::Left(Box::new(left_transfo.position)),
                                                                            Interaction::Seq(Box::new(left_transfo.result),i2.clone())) );
                        },
                        None => {}
                    }
                    match get_one_transformation_rec(transfos,i2) {
                        Some(right_transfo) => {
                            return Some( InteractionTermTransformation::new(right_transfo.kind,
                                                                            Position::Right(Box::new(right_transfo.position)),
                                                                            Interaction::Seq(i1.clone(), Box::new(right_transfo.result))) );
                        },
                        None => {}
                    }
                }, &Interaction::CoReg(ref cr, ref i1, ref i2) => {
                    match get_one_transformation_rec(transfos,i1) {
                        Some(left_transfo) => {
                            return Some( InteractionTermTransformation::new(left_transfo.kind,
                                                                            Position::Left(Box::new(left_transfo.position)),
                                                                            Interaction::CoReg(cr.clone(), Box::new(left_transfo.result),i2.clone())) );
                        },
                        None => {}
                    }
                    match get_one_transformation_rec(transfos,i2) {
                        Some(right_transfo) => {
                            return Some( InteractionTermTransformation::new(right_transfo.kind,
                                                                            Position::Right(Box::new(right_transfo.position)),
                                                                            Interaction::CoReg(cr.clone(), i1.clone(), Box::new(right_transfo.result))) );
                        },
                        None => {}
                    }
                }, &Interaction::Par(ref i1, ref i2) => {
                    match get_one_transformation_rec(transfos,i1) {
                        Some(left_transfo) => {
                            return Some( InteractionTermTransformation::new(left_transfo.kind,
                                                                            Position::Left(Box::new(left_transfo.position)),
                                                                            Interaction::Par(Box::new(left_transfo.result),i2.clone())) );
                        },
                        None => {}
                    }
                    match get_one_transformation_rec(transfos,i2) {
                        Some(right_transfo) => {
                            return Some( InteractionTermTransformation::new(right_transfo.kind,
                                                                            Position::Right(Box::new(right_transfo.position)),
                                                                            Interaction::Par(i1.clone(), Box::new(right_transfo.result))) );
                        },
                        None => {}
                    }
                }, &Interaction::Alt(ref i1, ref i2) => {
                    match get_one_transformation_rec(transfos,i1) {
                        Some(left_transfo) => {
                            return Some( InteractionTermTransformation::new(left_transfo.kind,
                                                                            Position::Left(Box::new(left_transfo.position)),
                                                                            Interaction::Alt(Box::new(left_transfo.result),i2.clone())) );
                        },
                        None => {}
                    }
                    match get_one_transformation_rec(transfos,i2) {
                        Some(right_transfo) => {
                            return Some( InteractionTermTransformation::new(right_transfo.kind,
                                                                            Position::Right(Box::new(right_transfo.position)),
                                                                            Interaction::Alt(i1.clone(), Box::new(right_transfo.result))) );
                        },
                        None => {}
                    }
                }, &Interaction::Loop(ref lk, ref i1) => {
                    match get_one_transformation_rec(transfos,i1) {
                        Some(sub_transfo) => {
                            return Some( InteractionTermTransformation::new(sub_transfo.kind,
                                                                            Position::Left(Box::new(sub_transfo.position)),
                                                                            Interaction::Loop(lk.clone(), Box::new(sub_transfo.result))) );
                        },
                        None => {}
                    }
                }, &Interaction::And(ref i1, ref i2) => {
                    match get_one_transformation_rec(transfos,i1) {
                        Some(left_transfo) => {
                            return Some( InteractionTermTransformation::new(left_transfo.kind,
                                                                            Position::Left(Box::new(left_transfo.position)),
                                                                            Interaction::Strict(Box::new(left_transfo.result),i2.clone())) );
                        },
                        None => {}
                    }
                    match get_one_transformation_rec(transfos,i2) {
                        Some(right_transfo) => {
                            return Some( InteractionTermTransformation::new(right_transfo.kind,
                                                                            Position::Right(Box::new(right_transfo.position)),
                                                                            Interaction::Strict(i1.clone(), Box::new(right_transfo.result))) );
                        },
                        None => {}
                    }
                }
            }
        }
    }
    return None;
}


