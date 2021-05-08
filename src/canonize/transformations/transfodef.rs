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
use crate::core::syntax::util::get_recursive_frag::*;
use crate::core::syntax::util::fold_recursive_frags::*;
use crate::core::general_context::GeneralContext;

use crate::canonize::total_order::interaction_lower_than;


pub(in crate::canonize) fn simpl_left(interaction : &Interaction) -> Option<Interaction> {
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

pub(in crate::canonize) fn simpl_right(interaction : &Interaction) -> Option<Interaction> {
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

pub(in crate::canonize) fn flush_right(interaction : &Interaction) -> Option<Interaction> {
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

pub(in crate::canonize) fn flush_left(interaction : &Interaction) -> Option<Interaction> {
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


pub(in crate::canonize) fn invert_alt_conditional(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            if interaction_lower_than(i2,i1) {
                return Some( Interaction::Alt( i2.clone(), i1.clone() ) );
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn invert_par_conditional(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Par(ref i1, ref i2) => {
            if interaction_lower_than(i2,i1) {
                return Some(Interaction::Par(i2.clone(), i1.clone()));
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn tri_invert_alt_conditional_right_flushed(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i_right) => {
            match **i_right {
                Interaction::Alt(ref i2,ref i3) => {
                    if interaction_lower_than(i2,i1) {
                        return Some( Interaction::Alt( i2.clone(), Box::new(Interaction::Alt(i1.clone(), i3.clone())) ) );
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn tri_invert_par_conditional_right_flushed(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Par(ref i1, ref i_right) => {
            match **i_right {
                Interaction::Par(ref i2,ref i3) => {
                    if interaction_lower_than(i2,i1) {
                        return Some(Interaction::Par(i2.clone(), Box::new(Interaction::Par(i1.clone(), i3.clone()))));
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn deduplicate(interaction : &Interaction) -> Option<Interaction> {
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

pub(in crate::canonize) fn tri_deduplicate_right_flushed(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i_right) => {
            match **i_right {
                Interaction::Alt(ref i2,ref i3) => {
                    if i1 == i2 {
                        return Some( Interaction::Alt(i1.clone(),i3.clone()) );
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn factorize_prefix_strict(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_strict_frags = get_recursive_strict_frags(i1);
            let mut right_strict_frags = get_recursive_strict_frags(i2);
            if left_strict_frags[0] == right_strict_frags[0] {
                let first_frag = left_strict_frags.remove(0);
                right_strict_frags.remove(0);
                if first_frag != &Interaction::Empty {
                    let new_alt = Interaction::Alt(Box::new(fold_recursive_strict_frags(&mut left_strict_frags)),
                                                   Box::new(fold_recursive_strict_frags(&mut right_strict_frags))
                    );
                    return Some( Interaction::Strict( Box::new(first_frag.clone()), Box::new(new_alt)) );
                }
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn factorize_prefix_seq(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_seq_frags = get_recursive_seq_frags(i1);
            let mut right_seq_frags = get_recursive_seq_frags(i2);
            if left_seq_frags[0] == right_seq_frags[0] {
                let first_frag = left_seq_frags.remove(0);
                right_seq_frags.remove(0);
                if first_frag != &Interaction::Empty {
                    let new_alt = Interaction::Alt(Box::new(fold_recursive_seq_frags(&mut left_seq_frags)),
                                                   Box::new(fold_recursive_seq_frags(&mut right_seq_frags))
                    );
                    return Some( Interaction::Seq( Box::new(first_frag.clone()), Box::new(new_alt)) );
                }
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn factorize_prefix_par(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_par_frags = get_recursive_par_frags(i1);
            let mut right_par_frags = get_recursive_par_frags(i2);
            if left_par_frags[0] == right_par_frags[0] {
                let first_frag = left_par_frags.remove(0);
                right_par_frags.remove(0);
                if first_frag != &Interaction::Empty {
                    let new_alt = Interaction::Alt(Box::new(fold_recursive_par_frags(&mut left_par_frags)),
                                                   Box::new(fold_recursive_par_frags(&mut right_par_frags))
                    );
                    return Some( Interaction::Par( Box::new(first_frag.clone()), Box::new(new_alt)) );
                }
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn factorize_suffix_strict(interaction : &Interaction) -> Option<Interaction> {
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
                    return Some( Interaction::Strict( Box::new(new_alt), Box::new(last_frag.clone()) ) );
                }
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn factorize_suffix_seq(interaction : &Interaction) -> Option<Interaction> {
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
                    return Some( Interaction::Seq( Box::new(new_alt), Box::new(last_frag.clone()) ) );
                }
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn factorize_suffix_par(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            let mut left_par_frags = get_recursive_par_frags(i1);
            let mut right_par_frags = get_recursive_par_frags(i2);
            if left_par_frags.last() == right_par_frags.last() {
                let last_frag : &Interaction = left_par_frags.pop().unwrap();
                right_par_frags.pop();
                if last_frag != &Interaction::Empty {
                    let new_alt = Interaction::Alt(Box::new(fold_recursive_par_frags(&mut left_par_frags)),
                                                   Box::new(fold_recursive_par_frags(&mut right_par_frags))
                    );
                    return Some( Interaction::Par( Box::new(new_alt), Box::new(last_frag.clone()) ) );
                }
            }
        },
        _ => {}
    }
    return None;
}

pub(in crate::canonize) fn defactorize_left(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::Strict( i1.clone(), i21.clone() );
                    let new_iB = Interaction::Strict( i1.clone(), i22.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::Seq( i1.clone(), i21.clone() );
                    let new_iB = Interaction::Seq( i1.clone(), i22.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::Par( i1.clone(), i21.clone() );
                    let new_iB = Interaction::Par( i1.clone(), i22.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            match **i2 {
                Interaction::Alt(ref i21, ref i22) => {
                    let new_iA = Interaction::CoReg( cr.clone(), i1.clone(), i21.clone() );
                    let new_iB = Interaction::CoReg( cr.clone(), i1.clone(), i22.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}



pub(in crate::canonize) fn defactorize_right(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::Strict( i11.clone(), i2.clone() );
                    let new_iB = Interaction::Strict( i12.clone(), i2.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::Seq( i11.clone(), i2.clone() );
                    let new_iB = Interaction::Seq( i12.clone(), i2.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::Par( i11.clone(), i2.clone() );
                    let new_iB = Interaction::Par( i12.clone(), i2.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            match **i1 {
                Interaction::Alt(ref i11, ref i12) => {
                    let new_iA = Interaction::CoReg( cr.clone(), i11.clone(), i2.clone() );
                    let new_iB = Interaction::CoReg( cr.clone(), i12.clone(), i2.clone() );
                    return Some( Interaction::Alt(Box::new(new_iA), Box::new(new_iB) ) );
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}



pub(in crate::canonize) fn loop_simpl(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Loop(ref sk, ref i1) => {
            match **i1 {
                Interaction::Empty => {
                    return Some( Interaction::Empty );
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}


pub(in crate::canonize) fn loop_unnest(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        &Interaction::Loop(ref lkA, ref i1) => {
            match **i1 {
                Interaction::Loop(ref lkB, ref i11) => {
                    return Some( Interaction::Loop((lkA.min(lkB)).clone(), i11.clone()) );
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}

























