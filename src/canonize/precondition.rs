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

use crate::rendering::graphviz::graph::*;
use crate::rendering::graphviz::node_style::*;
use crate::rendering::graphviz::edge_style::*;
use crate::rendering::graphviz::common::*;

use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;
use crate::core::syntax::position::Position;

use crate::core::semantics::execute::deploy_receptions;

use std::fmt;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum InteractionPreconditionCheckForCanonization {
    HasCoReg,
    HasTargets,
    IsOk
}

impl InteractionPreconditionCheckForCanonization {
    pub fn worst(&self, other:&InteractionPreconditionCheckForCanonization) -> InteractionPreconditionCheckForCanonization {
        match self {
            InteractionPreconditionCheckForCanonization::HasCoReg => {
                return InteractionPreconditionCheckForCanonization::HasCoReg;
            },
            InteractionPreconditionCheckForCanonization::HasTargets => {
                match other {
                    InteractionPreconditionCheckForCanonization::HasCoReg => {
                        return InteractionPreconditionCheckForCanonization::HasCoReg;
                    },
                    _ => {
                        return InteractionPreconditionCheckForCanonization::HasTargets;
                    }
                }
            },
            _ => {
                return other.clone();
            }
        }
    }
}

pub fn check_and_make_interaction_preconditions(interaction : &Interaction) -> (Interaction,InteractionPreconditionCheckForCanonization) {
    match interaction {
        Interaction::Empty => {
            return (Interaction::Empty,InteractionPreconditionCheckForCanonization::IsOk);
        },
        Interaction::Action(ref act) => {
            match act.act_kind {
                ObservableActionKind::Reception => {
                    return (Interaction::Action(act.clone()),InteractionPreconditionCheckForCanonization::IsOk);
                },
                ObservableActionKind::Emission(ref targs) => {
                    if targs.len() == 0 {
                        return (Interaction::Action(act.clone()),InteractionPreconditionCheckForCanonization::IsOk);
                    } else {
                        let emission = Interaction::Action(ObservableAction{lf_id:act.lf_id,
                            act_kind:ObservableActionKind::Emission(vec![]),ms_id:act.ms_id});
                        let deployed_recs = deploy_receptions(act.ms_id, &mut targs.clone());
                        let new_int = Interaction::Strict(Box::new(emission),Box::new(deployed_recs));
                        return (new_int,InteractionPreconditionCheckForCanonization::HasTargets);
                    }
                }
            }
        },
        Interaction::Strict(ref i1, ref i2) => {
            let (new_i1,check1) = check_and_make_interaction_preconditions(i1);
            let (new_i2,check2) = check_and_make_interaction_preconditions(i2);
            return (Interaction::Strict(Box::new(new_i1),Box::new(new_i2)), check1.worst(&check2));
        },
        Interaction::Seq(ref i1, ref i2) => {
            let (new_i1,check1) = check_and_make_interaction_preconditions(i1);
            let (new_i2,check2) = check_and_make_interaction_preconditions(i2);
            return (Interaction::Seq(Box::new(new_i1),Box::new(new_i2)), check1.worst(&check2));
        },
        Interaction::Par(ref i1, ref i2) => {
            let (new_i1,check1) = check_and_make_interaction_preconditions(i1);
            let (new_i2,check2) = check_and_make_interaction_preconditions(i2);
            return (Interaction::Par(Box::new(new_i1),Box::new(new_i2)), check1.worst(&check2));
        },
        Interaction::Alt(ref i1, ref i2) => {
            let (new_i1,check1) = check_and_make_interaction_preconditions(i1);
            let (new_i2,check2) = check_and_make_interaction_preconditions(i2);
            return (Interaction::Alt(Box::new(new_i1),Box::new(new_i2)), check1.worst(&check2));
        },
        Interaction::Loop(ref sk, ref i1) => {
            let (new_i1,check1) = check_and_make_interaction_preconditions(i1);
            return (Interaction::Loop(sk.clone(),Box::new(new_i1)), check1);
        },
        Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let (new_i1,check1) = check_and_make_interaction_preconditions(i1);
            let (new_i2,check2) = check_and_make_interaction_preconditions(i2);
            return (Interaction::CoReg(cr.clone(),Box::new(new_i1),Box::new(new_i2)), InteractionPreconditionCheckForCanonization::HasCoReg);
        }
    }
}



