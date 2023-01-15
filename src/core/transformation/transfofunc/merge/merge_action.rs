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



use crate::core::language::syntax::action::{EmissionAction, EmissionTargetRef, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;


pub fn transfo_merge_action(interaction : &Interaction) -> Option<Interaction> {
    match interaction {
        Interaction::And(ref i1, ref i2) => {
            match (&**i1,&**i2) {
                (Interaction::Emission(ref em_act),Interaction::Reception(ref rc_act)) => {
                    return merge_action_inner(em_act,rc_act);
                },
                (Interaction::Reception(ref rc_act),Interaction::Emission(ref em_act)) => {
                    return merge_action_inner(em_act,rc_act);
                },
                _ => {}
            }
        },
        _ => {}
    }
    return None;
}


fn merge_action_inner(emission : &EmissionAction,
                      reception : &ReceptionAction) -> Option<Interaction> {
    if emission.ms_id != reception.ms_id {
        return None;
    }
    // ***
    if emission.synchronicity != reception.synchronicity {
        return None;
    }
    // ***
    match &reception.origin_gt_id {
        None => {
            return None;
        },
        Some( gt_id_to_match) => {
            let mut is_match = false;
            let mut targets = vec![];
            for target_ref in &emission.targets {
                match target_ref {
                    EmissionTargetRef::Lifeline(lf_id) => {
                        targets.push(EmissionTargetRef::Lifeline(*lf_id));
                    },
                    EmissionTargetRef::Gate(gt_id) => {
                        if gt_id == gt_id_to_match {
                            is_match == true;
                            for recipient_lf in &reception.recipients {
                                targets.push(EmissionTargetRef::Lifeline(*recipient_lf));
                            }
                        } else {
                            targets.push(EmissionTargetRef::Gate(*gt_id));
                        }
                    }
                }
            }
            // ***
            if is_match {
                let new_action = EmissionAction::new(emission.origin_lf_id,emission.ms_id,emission.synchronicity.clone(),targets);
                return Some(Interaction::Emission(new_action));
            } else {
                return None;
            }
        }
    }
}

