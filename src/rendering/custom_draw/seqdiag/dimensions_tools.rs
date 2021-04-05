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
use std::cmp;


use crate::core::syntax::action::*;
use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::util::get_recursive_frag::*;

use crate::rendering::sd_drawing_conf::*;



pub fn get_interaction_max_yshift(interaction : &Interaction) -> usize {
    let mut cpt = 4;
    cpt += get_interaction_depth(interaction);
    return cpt;
}


fn get_interaction_depth(interaction : &Interaction ) -> usize {
    match interaction {
        &Interaction::Empty => {return  0},
        &Interaction::Action(ref act) => {
            return 3;
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let mut frags = get_recursive_strict_frags(i1);
            frags.extend( get_recursive_strict_frags(i2) );
            let mut sum : usize = 2;
            for frag in frags {
                sum = sum + get_interaction_depth(frag) + 2;
            }
            return sum;
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let mut frags = get_recursive_coreg_frags(cr, i1);
            frags.extend( get_recursive_coreg_frags(cr, i2) );
            let mut sum : usize = 0;
            for frag in frags {
                sum = sum + get_interaction_depth(frag) + 2;
            }
            return sum;
        },
        &Interaction::Seq(ref i1, ref i2) => {
            return get_interaction_depth(i1) + get_interaction_depth(i2) + 1;
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut frags = get_recursive_alt_frags(i1);
            frags.extend( get_recursive_alt_frags(i2) );
            let mut sum : usize = 2;
            for frag in frags {
                sum = sum + get_interaction_depth(frag) + 2;
            }
            return sum;
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut frags = get_recursive_par_frags(i1);
            frags.extend( get_recursive_par_frags(i2) );
            let mut sum : usize = 2;
            for frag in frags {
                sum = sum + get_interaction_depth(frag) + 2;
            }
            return sum;
        },
        &Interaction::Loop(_, ref i1) => {
            return get_interaction_depth(i1) + 4;
        }
    }
}

pub fn get_y_pos_from_yshift(yshift : u32) -> f32 {
    return MARGIN + VERTICAL_SIZE*(yshift as f32);
}

