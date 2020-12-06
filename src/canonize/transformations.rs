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


use crate::core::syntax::interaction::{Interaction,ScheduleOperatorKind};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::general_context::GeneralContext;

use crate::rendering::textual::monochrome::position::position_to_text;

pub enum TransformationKind {
    SimplLeft,
    SimplRight,
    FlushLeft,
    FlushRight,
    InvertAlt,
    InvertPar,
    TriInvertAlt,
    TriInvertPar,
    Deduplicate,
    Factorize
}

impl TransformationKind {
    pub fn to_string(&self) -> String {
        match self {
            &TransformationKind::SimplLeft => {
                return "SimplLeft".to_string();
            },
            &TransformationKind::SimplRight => {
                return "SimplRight".to_string();
            },
            &TransformationKind::FlushLeft => {
                return "FlushLeft".to_string();
            },
            &TransformationKind::FlushRight => {
                return "FlushRight".to_string();
            },
            &TransformationKind::InvertAlt => {
                return "InvertAlt".to_string();
            },
            &TransformationKind::InvertPar => {
                return "InvertPar".to_string();
            },
            &TransformationKind::TriInvertAlt => {
                return "TriInvertAlt".to_string();
            },
            &TransformationKind::TriInvertPar => {
                return "TriInvertPar".to_string();
            },
            &TransformationKind::Deduplicate => {
                return "Deduplicate".to_string();
            },
            &TransformationKind::Factorize => {
                return "Factorize".to_string();
            }
        }
    }
}

fn simpl_left(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            if **i1 == Interaction::Empty {
                return Some( *(i2.clone()) );
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            if **i1 == Interaction::Empty {
                return Some( *(i2.clone()) );
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            if **i1 == Interaction::Empty {
                return Some( *(i2.clone()) );
            }
        },
        &Interaction::CoReg(_, ref i1, ref i2) => {
            if **i1 == Interaction::Empty {
                return Some( *(i2.clone()) );
            }
        },
        _ => {}
    }
    return None;
}

fn simpl_right(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            if **i2 == Interaction::Empty {
                return Some( *(i1.clone()) );
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            if **i2 == Interaction::Empty {
                return Some( *(i1.clone()) );
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            if **i2 == Interaction::Empty {
                return Some( *(i1.clone()) );
            }
        },
        &Interaction::CoReg(_, ref i1, ref i2) => {
            if **i2 == Interaction::Empty {
                return Some( *(i1.clone()) );
            }
        },
        _ => {}
    }
    return None;
}

fn flush_right(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11,ref i12) => {
                    return Some( Interaction::Alt( i11.clone(), Box::new(Interaction::Alt(i12.clone(), i2.clone())) ) );
                },
                _ => {}
            }
        },
        &Interaction::Strict(ref i1, ref i2) => {
            match **i1 {
                Interaction::Strict(ref i11,ref i12) => {
                    return Some( Interaction::Strict( i11.clone(), Box::new(Interaction::Strict(i12.clone(), i2.clone())) ) );
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i1 {
                Interaction::Seq(ref i11,ref i12) => {
                    return Some( Interaction::Seq( i11.clone(), Box::new(Interaction::Seq(i12.clone(), i2.clone())) ) );
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i1 {
                Interaction::Par(ref i11,ref i12) => {
                    return Some( Interaction::Par( i11.clone(), Box::new(Interaction::Par(i12.clone(), i2.clone())) ) );
                },
                _ => {}
            }
        },
        &Interaction::CoReg(ref cr1, ref i1, ref i2) => {
            match **i1 {
                Interaction::CoReg(ref cr2, ref i11,ref i12) => {
                    if cr1 == cr2 {
                        return Some( Interaction::CoReg( cr1.clone(), i11.clone(), Box::new(Interaction::CoReg(cr1.clone(), i12.clone(), i2.clone())) ) );
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}

fn flush_left(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21,ref i22) => {
                    return Some( Interaction::Alt( Box::new(Interaction::Alt(i1.clone(), i21.clone())), i22.clone() ) );
                },
                _ => {}
            }
        },
        &Interaction::Strict(ref i1, ref i2) => {
            match **i2 {
                Interaction::Strict(ref i21,ref i22) => {
                    return Some( Interaction::Strict( Box::new(Interaction::Strict(i1.clone(), i21.clone())), i22.clone() ) );
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i2 {
                Interaction::Seq(ref i21,ref i22) => {
                    return Some( Interaction::Seq( Box::new(Interaction::Seq(i1.clone(), i21.clone())), i22.clone() ) );
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i2 {
                Interaction::Par(ref i21,ref i22) => {
                    return Some( Interaction::Par( Box::new(Interaction::Par(i1.clone(), i21.clone())), i22.clone() ) );
                },
                _ => {}
            }
        },
        &Interaction::CoReg(ref cr1, ref i1, ref i2) => {
            match **i2 {
                Interaction::CoReg(ref cr2, ref i21,ref i22) => {
                    if cr1 == cr2 {
                        return Some( Interaction::CoReg( cr1.clone(), Box::new(Interaction::CoReg(cr1.clone(), i1.clone(), i21.clone())), i22.clone() ) );
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}


fn invert_alt(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            return Some( Interaction::Alt( i2.clone(), i1.clone() ) );
        },
        _ => {}
    }
    return None;
}

fn invert_par(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Par(ref i1, ref i2) => {
            return Some( Interaction::Par( i2.clone(), i1.clone() ) );
        },
        _ => {}
    }
    return None;
}

fn tri_invert_alt(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i_right) => {
            match **i_right {
                Interaction::Alt(ref i2,ref i3) => {
                    return Some( Interaction::Alt( i2.clone(), Box::new(Interaction::Alt(i1.clone(), i3.clone())) ) );
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}

fn tri_invert_par(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Par(ref i1, ref i_right) => {
            match **i_right {
                Interaction::Par(ref i2,ref i3) => {
                    return Some( Interaction::Par( i2.clone(), Box::new(Interaction::Par(i1.clone(), i3.clone())) ) );
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}

fn deduplicate(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            if i1 == i2 {
                return Some( *i1.clone() );
            }
        },
        _ => {}
    }
    return None;
}

fn factorize(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            match **i1 {
                Interaction::Strict(ref i11, ref i12) => {
                    match **i2 {
                        Interaction::Strict(ref i21, ref i22) => {
                            if i11 == i21 {
                                return Some( Interaction::Strict( i11.clone(), Box::new(Interaction::Alt(i12.clone(), i22.clone()))));
                            }
                        },
                        _ => {}
                    }
                },
                Interaction::Seq(ref i11, ref i12) => {
                    match **i2 {
                        Interaction::Seq(ref i21, ref i22) => {
                            if i11 == i12 {
                                return Some( Interaction::Seq( i11.clone(), Box::new(Interaction::Alt(i12.clone(), i22.clone()))));
                            }
                        },
                        _ => {}
                    }
                },
                Interaction::Par(ref i11, ref i12) => {
                    match **i2 {
                        Interaction::Par(ref i21, ref i22) => {
                            if i11 == i12 {
                                return Some( Interaction::Par( i11.clone(), Box::new(Interaction::Alt(i12.clone(), i22.clone()))));
                            }
                        },
                        _ => {}
                    }
                },
                Interaction::CoReg(ref cra, ref i11, ref i12) => {
                    match **i2 {
                        Interaction::CoReg(ref crb, ref i21, ref i22) => {
                            if i11 == i12 && cra == crb {
                                return Some( Interaction::CoReg( cra.clone(), i11.clone(), Box::new(Interaction::Alt(i12.clone(), i22.clone()))));
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}


pub struct InteractionTermTransformation {
    pub kind : TransformationKind,
    pub position : Position,
    pub result : Interaction
}

impl InteractionTermTransformation {
    pub fn new(kind : TransformationKind,
               position : Position,
               result : Interaction) -> InteractionTermTransformation {
        return InteractionTermTransformation{kind,position,result};
    }

    pub fn transformation_str_description(&self) -> String {
        return format!("{}@{}", self.kind.to_string(), position_to_text(&self.position))
    }
}

pub fn get_all_transformations_rec(interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    let mut transformations = get_all_transformations_inner(interaction);
    match interaction {
        &Interaction::Empty => {
            // ***
        }, &Interaction::Action(_) => {
            // ***
        }, &Interaction::Strict(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(i1) {
                transformations.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Strict(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(i2) {
                transformations.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Strict(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Seq(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(i1) {
                transformations.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Seq(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(i2) {
                transformations.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Seq(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(i1) {
                transformations.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::CoReg(cr.clone(), Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(i2) {
                transformations.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::CoReg(cr.clone(), i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Par(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(i1) {
                transformations.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Par(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(i2) {
                transformations.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Par(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Alt(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(i1) {
                transformations.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Alt(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(i2) {
                transformations.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Alt(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Loop(ref lk, ref i1) => {
            for left_transfo in get_all_transformations_rec(i1) {
                transformations.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Loop(lk.clone(), Box::new(left_transfo.result))
                ) );
            }
        }
    }
    return transformations;
}

fn get_all_transformations_inner(interaction : &Interaction) -> Vec<InteractionTermTransformation> {

    let transfos_funcs : Vec<(TransformationKind, &dyn Fn(&Interaction) -> Option<Interaction>)> = vec![
        (TransformationKind::SimplLeft,&simpl_left),
        (TransformationKind::SimplRight,&simpl_right),
        (TransformationKind::Deduplicate,&deduplicate),
        (TransformationKind::Factorize,&factorize),
        (TransformationKind::FlushLeft,&flush_left),
        (TransformationKind::FlushRight,&flush_right),
        (TransformationKind::InvertAlt,&invert_alt),
        (TransformationKind::InvertPar,&invert_par),
        (TransformationKind::TriInvertAlt,&tri_invert_alt),
        (TransformationKind::TriInvertPar,&tri_invert_par)
    ];

    let mut transformations : Vec<InteractionTermTransformation> = Vec::new();
    for (transfo_kind, transfo_func) in transfos_funcs {
        match transfo_func(interaction) {
            None => {},
            Some(new_int) => {
                transformations.push( InteractionTermTransformation::new(transfo_kind,
                                                                         Position::Epsilon,
                                                                         new_int) );
            }
        }
    }

    /*
    match simpl_left(interaction) {
        None => {},
        Some(new_int) => {
            transformations.push( InteractionTermTransformation::new(TransformationKind::SimplLeft,
                                                                     position.clone(), new_int) );
        }
    }
    match simpl_right(interaction) {
        None => {},
        Some(new_int) => {
            transformations.push( InteractionTermTransformation::new(TransformationKind::SimplRight,
                                                                     position.clone(), new_int) );
        }
    }
    match simpl_right(interaction) {
        None => {},
        Some(new_int) => {
            transformations.push( InteractionTermTransformation::new(TransformationKind::SimplRight,
                                                                     position.clone(), new_int) );
        }
    }*/

    return transformations;
}


