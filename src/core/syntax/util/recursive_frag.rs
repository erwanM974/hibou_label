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


use crate::core::syntax::interaction::Interaction;


pub fn get_recursive_alt_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Alt(ref i1, ref i2) => {
            frags.extend( get_recursive_alt_frags(i1));
            frags.extend( get_recursive_alt_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

pub fn get_recursive_par_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Par(ref i1, ref i2) => {
            frags.extend( get_recursive_par_frags(i1));
            frags.extend( get_recursive_par_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

pub fn get_recursive_strict_frags(interaction : &Interaction) -> Vec<&Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            frags.extend( get_recursive_strict_frags(i1));
            frags.extend( get_recursive_strict_frags(i2));
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}

pub fn get_recursive_coreg_frags<'lifetime>(ref_cr : &Vec<usize>, interaction : &'lifetime Interaction) -> Vec<&'lifetime Interaction> {
    let mut frags : Vec<&Interaction> = Vec::new();
    match interaction {
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            if cr == ref_cr {
                frags.extend( get_recursive_coreg_frags(ref_cr,i1));
                frags.extend( get_recursive_coreg_frags(ref_cr,i2));
            } else {
                frags.push(interaction);
            }
        },
        _ => {
            frags.push(interaction);
        }
    }
    return frags;
}
