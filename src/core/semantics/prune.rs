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
use std::collections::HashSet;

use crate::core::syntax::position::*;
use crate::core::syntax::interaction::*;
use crate::core::semantics::frontier::*;


pub fn prune(my_int : Interaction, lf_id : usize) -> Interaction {
    match my_int {
        Interaction::Empty => {
            return my_int;
        },
        Interaction::Action(_) => {
            return my_int;
        },
        Interaction::Seq(i1, i2) => {
            return Interaction::Seq( Box::new( prune(*i1,lf_id)), Box::new( prune(*i2,lf_id)) );
        },
        Interaction::Strict(i1, i2) => {
            return Interaction::Strict( Box::new( prune(*i1,lf_id)), Box::new( prune(*i2,lf_id)) );
        },
        Interaction::Par(i1, i2) => {
            return Interaction::Par( Box::new( prune(*i1,lf_id)), Box::new( prune(*i2,lf_id)) );
        },
        Interaction::Alt(i1, i2) => {
            if i1.avoids(lf_id) {
                if i2.avoids(lf_id) {
                    return Interaction::Alt( Box::new( prune(*i1,lf_id)), Box::new( prune(*i2,lf_id)) );
                } else {
                    return prune(*i1,lf_id);
                }
            } else {
                return prune(*i2,lf_id);
            }
        },
        Interaction::Loop(lkind, i1) => {
            if i1.avoids(lf_id) {
                return Interaction::Loop(lkind, Box::new(prune(*i1,lf_id)));
            } else {
                return Interaction::Empty;
            }
        }
    }
}


