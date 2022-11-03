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


use std::collections::{HashMap, HashSet};
use crate::core::language::syntax::interaction::{Interaction};
use crate::core::language::syntax::action::*;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};





pub fn check_if_merge_possible(i1 : &Interaction, i2 : &Interaction, connections : &mut HashMap<usize,usize>) -> bool {
    let (mut i1_em_acts,mut i1_rc_acts) = i1.contained_model_actions();
    let (mut i2_em_acts,mut i2_rc_acts) = i2.contained_model_actions();
    check_matching_ems_rcs(i1_em_acts,i2_rc_acts,connections);
    check_matching_ems_rcs(i2_em_acts,i1_rc_acts,connections);
    if connections.len() > 0 {
        // it is not possible to find all required connections
        return false;
    } else {
        // it may be possible to find all required connections
        return true;
    }
}



fn check_matching_ems_rcs(em_acts : HashSet<&EmissionAction>, rc_acts : HashSet<&ReceptionAction>, connections : &mut HashMap<usize,usize>) {
    let mut got_rc_acts = rc_acts;
    for em in em_acts {
        let mut rem_rc_acts = hashset!{};
        for rc in got_rc_acts {
            match is_action_merge_impossible(em,rc,connections) {
                None => {},
                Some(reason) => {
                    rem_rc_acts.insert(rc);
                }
            }
        }
        got_rc_acts = rem_rc_acts;
    }
}



enum WillNotMergeBecause {
    ReceptionFromEnvironment,
    MismatchOfMessageID,
    CouldNotFindConnection
}

fn is_action_merge_impossible(em_act : &EmissionAction, rc_act : &ReceptionAction, connections : &mut HashMap<usize,usize>) -> Option<WillNotMergeBecause> {
    match &rc_act.origin_gt_id {
        None => {
            return Some(WillNotMergeBecause::ReceptionFromEnvironment);
        },
        Some( gt_id_in_rc ) => {
            for target_ref in &em_act.targets {
                match target_ref {
                    EmissionTargetRef::Gate( gt_id_in_em) => {
                        match connections.get(gt_id_in_em) {
                            None => {},
                            Some( target_gt_id ) => {
                                if target_gt_id == gt_id_in_rc {
                                    if em_act.ms_id == rc_act.ms_id {
                                        connections.remove(gt_id_in_em);
                                        return None;
                                    } else {
                                        return Some(WillNotMergeBecause::MismatchOfMessageID);
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
    return Some(WillNotMergeBecause::CouldNotFindConnection);
}




pub fn check_models_for_merge(i1 : &Interaction, i2 : &Interaction) -> bool {
    let i1_lfs = i1.involved_lifelines();
    let i2_lfs = i2.involved_lifelines();
    if i1_lfs.is_disjoint(&i2_lfs) {
        let (i1_em_acts,i1_rc_acts) = i1.contained_model_actions();
        let (i2_em_acts,i2_rc_acts) = i2.contained_model_actions();
        // ***
        let mut got_gates: HashSet<usize> = hashset!{};
        for em_act in i1_em_acts.union(&i2_em_acts) {
            for targ_ref in &em_act.targets {
                match targ_ref {
                    EmissionTargetRef::Gate( gt_id ) => {
                        if got_gates.contains(gt_id) {
                            println!("Each gate must be used at most once");
                            return false;
                        } else {
                            got_gates.insert(*gt_id);
                        }
                    },
                    _ => {}
                }
            }
        }
        for rc_act in i1_rc_acts.union(&i2_rc_acts) {
            match &rc_act.origin_gt_id {
                None => {},
                Some(gt_id) => {
                    if got_gates.contains(gt_id) {
                        println!("Each gate must be used at most once");
                        return false;
                    } else {
                        got_gates.insert(*gt_id);
                    }
                }
            }
        }
    } else {
        println!("Interactions to merge must have disjoint sets of lifelines");
        return false;
    }
    return true;
}
