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



use std::collections::{BTreeSet, HashSet};
use crate::core::execution::trace::from_model::from_model::PrimitiveInterpretableAsTraceAction;
use crate::core::execution::trace::trace::{TraceAction};
use crate::core::language::avoid::avoids::AvoidsLifelines;
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::position::position::Position;
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FrontierElement {
    pub position : Position,
    pub target_lf_ids : BTreeSet<usize>,
    pub target_actions : BTreeSet<TraceAction>,
    pub max_loop_depth : u32
}


impl FrontierElement {
    pub fn new(position : Position,
               target_lf_ids : BTreeSet<usize>,
               target_actions : BTreeSet<TraceAction>,
               max_loop_depth : u32) -> FrontierElement {
        return FrontierElement{position,target_lf_ids,target_actions,max_loop_depth};
    }
}




pub fn global_frontier(interaction : &Interaction) -> Vec<FrontierElement> {
    global_frontier_rec(interaction, 0)
}


fn frontier_on_emission(em_act : &EmissionAction, loop_depth : u32) -> Vec<FrontierElement> {
    match &em_act.synchronicity {
        CommunicationSynchronicity::Synchronous => {
            let occupation = em_act.involved_lifelines();
            let actions = em_act.get_all_atomic_actions();
            return vec![FrontierElement::new(Position::Epsilon(None),
                                             occupation,
                                             actions,
                                             loop_depth)];
        },
        CommunicationSynchronicity::Asynchronous => {
            let emission_tract = em_act.get_first_atomic_action();
            return vec![FrontierElement::new(Position::Epsilon(None),
                                             btreeset!{em_act.origin_lf_id},
                                             btreeset!{emission_tract},
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
                                             loop_depth)];
        },
        CommunicationSynchronicity::Asynchronous => {
            let mut frt = vec![];
            for (rcp_idx,rcp_lf_id) in rc_act.recipients.iter().enumerate() {
                let reception_tract = rc_act.get_specific_atomic_action(rcp_idx);
                frt.push( FrontierElement::new(Position::Epsilon(Some(rcp_idx)),
                                               btreeset!{*rcp_lf_id},
                                               btreeset!{reception_tract},
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
            // BELOW with delayed alt
            let mut match_indices : HashSet<(usize,usize)> = hashset! {};
            let mut frt1_matched : HashSet<usize> = hashset![];
            let mut frt2_matched : HashSet<usize> = hashset![];
            // ***
            let frt1 = global_frontier_rec(i1,loop_depth);
            let frt2 = global_frontier_rec(i2,loop_depth);
            // ***
            for (frt1_idx,frt1_elt) in frt1.iter().enumerate() {
                for (frt2_idx,frt2_elt) in frt2.iter().enumerate() {
                    if frt1_elt.target_actions == frt2_elt.target_actions {
                        frt1_matched.insert(frt1_idx);
                        frt2_matched.insert(frt2_idx);
                        match_indices.insert( (frt1_idx,frt2_idx) );
                    }
                }
            }
            // ***
            let mut new_front = vec![];
            // ***
            for (frt1_idx,frt2_idx) in match_indices {
                let frt1_elt : &FrontierElement = frt1.get(frt1_idx).unwrap();
                let frt2_elt: &FrontierElement = frt2.get(frt2_idx).unwrap();
                let new_pos = Position::Both( Box::new(frt1_elt.position.clone()), Box::new(frt2_elt.position.clone()));
                let new_target_lf_ids : BTreeSet<usize> = frt1_elt.target_lf_ids.union(&frt2_elt.target_lf_ids).cloned().collect();
                let new_target_actions : BTreeSet<TraceAction> = frt1_elt.target_actions.union(&frt2_elt.target_actions).cloned().collect();
                let new_max_loop_depth = frt1_elt.max_loop_depth.max(frt2_elt.max_loop_depth);
                // ***
                new_front.push( FrontierElement::new(new_pos,
                                                     new_target_lf_ids,
                                                     new_target_actions,
                                                     new_max_loop_depth ));
            }
            // ***
            for (frt1_idx,frt1_elt) in frt1.into_iter().enumerate() {
                if !frt1_matched.contains(&frt1_idx) {
                    let shifted_pos = Position::Left(Box::new(frt1_elt.position));
                    new_front.push( FrontierElement::new(shifted_pos,
                                                         frt1_elt.target_lf_ids,
                                                         frt1_elt.target_actions,
                                                         frt1_elt.max_loop_depth ));
                }
            }
            // ***
            for (frt2_idx,frt2_elt) in frt2.into_iter().enumerate() {
                if !frt2_matched.contains(&frt2_idx) {
                    let shifted_pos = Position::Right(Box::new(frt2_elt.position));
                    new_front.push( FrontierElement::new(shifted_pos,
                                                         frt2_elt.target_lf_ids,
                                                         frt2_elt.target_actions,
                                                         frt2_elt.max_loop_depth ));
                }
            }
            // ***
            return new_front;
            // *****
            // BELOW non-delayed ALT
            /*let mut front = push_frontier_left( &mut global_frontier_rec(i1,loop_depth) );
            front.append( &mut push_frontier_right( &mut global_frontier_rec(i2,loop_depth)) );
            return front;*/
        },
        Interaction::Par(ref i1, ref i2) => {
            let mut front = push_frontier_left( &mut global_frontier_rec(i1,loop_depth) );
            front.append( &mut push_frontier_right( &mut global_frontier_rec(i2,loop_depth)) );
            return front;
        },
        Interaction::Sync(ref sync_acts,ref i1, ref i2) => {
            let sync_acts_as_set : BTreeSet<TraceAction> = BTreeSet::from_iter(sync_acts.iter().cloned());
            // ***
            let mut new_front = vec![];
            let mut rem_frt1 = vec![];
            let mut rem_frt2 = vec![];
            // ***
            for frt1_elt in global_frontier_rec(i1,loop_depth) {
                let intersect : BTreeSet<TraceAction> = frt1_elt.target_actions.intersection(&sync_acts_as_set).cloned().collect();
                if intersect.is_empty() {
                    let shifted_pos = Position::Left(Box::new(frt1_elt.position));
                    new_front.push( FrontierElement::new(shifted_pos,
                                                         frt1_elt.target_lf_ids,
                                                         frt1_elt.target_actions,
                                                         frt1_elt.max_loop_depth ));
                } else {
                    rem_frt1.push((frt1_elt, intersect) );
                }
            }
            // ***
            for frt2_elt in global_frontier_rec(i2,loop_depth) {
                let intersect : BTreeSet<TraceAction> = frt2_elt.target_actions.intersection(&sync_acts_as_set).cloned().collect();
                if intersect.is_empty() {
                    let shifted_pos = Position::Right(Box::new(frt2_elt.position));
                    new_front.push( FrontierElement::new(shifted_pos,
                                                         frt2_elt.target_lf_ids,
                                                         frt2_elt.target_actions,
                                                         frt2_elt.max_loop_depth ));
                } else {
                    rem_frt2.push((frt2_elt,intersect) );
                }
            }
            // ***
            for (frt1_elt, intersect1) in &rem_frt1 {
                for (frt2_elt,intersect2) in &rem_frt2 {
                    if intersect1 == intersect2 {
                        let new_pos = Position::Both(Box::new(frt1_elt.position.clone()),
                                                     Box::new(frt2_elt.position.clone()));
                        let new_target_lf_ids : BTreeSet<usize> = frt1_elt.target_lf_ids.union(&frt2_elt.target_lf_ids).cloned().collect();
                        let new_target_actions : BTreeSet<TraceAction> = frt1_elt.target_actions.union(&frt2_elt.target_actions).cloned().collect();
                        let new_max_loop_depth = frt1_elt.max_loop_depth.max(frt2_elt.max_loop_depth);
                        // ***
                        new_front.push( FrontierElement::new(new_pos,
                                                             new_target_lf_ids,
                                                             new_target_actions,
                                                             new_max_loop_depth ));
                    }
                }
            }
            // ***
            new_front
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
                                                                       frt_elt.max_loop_depth ) ).collect();
}

fn push_frontier_right(frontier : &mut Vec<FrontierElement>) -> Vec<FrontierElement> {
    return frontier.drain(..).map(|frt_elt| FrontierElement::new(Position::Right( Box::new(frt_elt.position ) ),
                                                                 frt_elt.target_lf_ids,
                                                                 frt_elt.target_actions,
                                                                 frt_elt.max_loop_depth) ).collect();
}