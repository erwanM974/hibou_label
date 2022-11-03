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
use crate::core::execution::trace::from_model::from_model::InterpretableAsTraceAction;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::language::avoid::avoids::AvoidsLifelines;
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::position::position::Position;
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;


#[derive(Clone, PartialEq, Debug)]
pub struct FrontierElement {
    pub position : Position,
    pub target_lf_ids : HashSet<usize>,
    pub target_actions : HashSet<TraceAction>,
    pub act_kind : TraceActionKind,
    pub loop_depth : u32
}


impl FrontierElement {
    pub fn new(position : Position,
               target_lf_ids : HashSet<usize>,
               target_actions : HashSet<TraceAction>,
               act_kind : TraceActionKind,
               loop_depth : u32) -> FrontierElement {
        return FrontierElement{position,target_lf_ids,target_actions,act_kind,loop_depth};
    }
}




pub fn global_frontier(interaction : &Interaction,
                   to_match : &Option<&HashSet<&TraceAction>>) -> Vec<FrontierElement> {
    match to_match {
        None => {
            return global_frontier_rec(interaction, 0);
        },
        Some( to_match_set ) => {
            let mut frt = vec![];
            for frt_elt in global_frontier_rec(interaction, 0) {
                let of_references : HashSet<&TraceAction> = frt_elt.target_actions.iter().collect();
                if of_references.is_subset(to_match_set) {
                    frt.push(frt_elt);
                }
            }
            return frt;
        }
    }
}


fn frontier_on_emission(em_act : &EmissionAction, loop_depth : u32) -> Vec<FrontierElement> {
    match &em_act.synchronicity {
        CommunicationSynchronicity::Synchronous => {
            let occupation = em_act.involved_lifelines();
            let actions = em_act.get_all_atomic_actions();
            return vec![FrontierElement::new(Position::Epsilon(None),
                                             occupation,
                                             actions,
                                             TraceActionKind::Emission,
                                             loop_depth)];
        },
        CommunicationSynchronicity::Asynchronous => {
            let emission_tract = em_act.get_first_atomic_action();
            return vec![FrontierElement::new(Position::Epsilon(None),
                                             hashset!{em_act.origin_lf_id},
                                             hashset!{emission_tract},
                                             TraceActionKind::Emission,
                                             loop_depth)];
        }
    }
}

fn frontier_on_reception(rc_act : &ReceptionAction, loop_depth : u32) -> Vec<FrontierElement> {
    match &rc_act.synchronicity {
        CommunicationSynchronicity::Synchronous => {
            let occupation = rc_act.involved_lifelines();
            let actions = rc_act.get_all_atomic_actions();
            return vec![FrontierElement::new(Position::Epsilon(None),
                                             occupation,
                                             actions,
                                             TraceActionKind::Reception,
                                             loop_depth)];
        },
        CommunicationSynchronicity::Asynchronous => {
            let mut frt = vec![];
            for (rcp_idx,rcp_lf_id) in rc_act.recipients.iter().enumerate() {
                let reception_tract = rc_act.get_specific_atomic_action(rcp_idx);
                frt.push( FrontierElement::new(Position::Epsilon(Some(rcp_idx)),
                                               hashset!{*rcp_lf_id},
                                               hashset!{reception_tract},
                                               TraceActionKind::Reception,
                                               loop_depth) );
            }
            return frt;
        }
    }
}

fn global_frontier_rec(interaction : &Interaction, loop_depth : u32) -> Vec<FrontierElement> {
    match interaction {
        Interaction::Empty => {
            return vec![];
        },
        Interaction::Emission( em_act) => {
            return frontier_on_emission(em_act, loop_depth);
        },
        Interaction::Reception( rc_act) => {
            return frontier_on_reception(rc_act, loop_depth);
        },
        Interaction::Strict(ref i1, ref i2) => {
            let mut front = push_frontier_left( &mut global_frontier_rec(i1,loop_depth) );
            if i1.express_empty() {
                front.append( &mut push_frontier_right( &mut global_frontier_rec(i2,loop_depth)) );
            }
            return front;
        },
        Interaction::Seq(ref i1, ref i2) => {
            let mut front = push_frontier_left( &mut global_frontier_rec(i1,loop_depth) );
            // ***
            for frt_elt2 in push_frontier_right( &mut global_frontier_rec(i2,loop_depth)) {
                if i1.avoids_all_of(&frt_elt2.target_lf_ids) {
                    front.push(frt_elt2);
                }
            }
            return front;
        },
        Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let mut front = push_frontier_left( &mut global_frontier_rec(i1,loop_depth) );
            // ***
            for frt_elt2 in push_frontier_right( &mut global_frontier_rec(i2,loop_depth)) {
                let mut reqs_lf_ids = frt_elt2.target_lf_ids.clone();
                for cr_lf_id in cr {
                    reqs_lf_ids.remove(cr_lf_id);
                }
                if i1.avoids_all_of(&reqs_lf_ids) {
                    front.push(frt_elt2);
                }
            }
            return front;
        },
        Interaction::Alt(ref i1, ref i2) => {
            let mut front = push_frontier_left( &mut global_frontier_rec(i1,loop_depth) );
            front.append( &mut push_frontier_right( &mut global_frontier_rec(i2,loop_depth)) );
            return front;
        },
        Interaction::Par(ref i1, ref i2) => {
            let mut front = push_frontier_left( &mut global_frontier_rec(i1,loop_depth) );
            front.append( &mut push_frontier_right( &mut global_frontier_rec(i2,loop_depth)) );
            return front;
        },
        Interaction::Loop(_, ref i1) => {
            return push_frontier_left( &mut global_frontier_rec(i1,loop_depth+1) );
        },
        _ => {
            panic!("non-conform interaction");
        }
    }
}



fn push_frontier_left(frontier : &mut Vec<FrontierElement>) -> Vec<FrontierElement> {
    return frontier.drain(..).map(|frt_elt| FrontierElement::new(Position::Left( Box::new(frt_elt.position ) ),
                                                                       frt_elt.target_lf_ids,
                                                                       frt_elt.target_actions,
                                                                       frt_elt.act_kind,
                                                                       frt_elt.loop_depth ) ).collect();
}

fn push_frontier_right(frontier : &mut Vec<FrontierElement>) -> Vec<FrontierElement> {
    return frontier.drain(..).map(|frt_elt| FrontierElement::new(Position::Right( Box::new(frt_elt.position ) ),
                                                                 frt_elt.target_lf_ids,
                                                                 frt_elt.target_actions,
                                                                 frt_elt.act_kind,
                                                                 frt_elt.loop_depth) ).collect();
}