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




use crate::core::execution::trace::trace::TraceActionKind::Reception;
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, EmissionTargetRef, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::util::fold_recursive_frags::fold_recursive_par_frags;

pub fn transfo_sort_action_content(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Emission(ref em_act) => {
            let mut new_targets = em_act.targets.clone();
            new_targets.sort();
            if new_targets != em_act.targets {
                let new_emission = EmissionAction::new(em_act.origin_lf_id,em_act.ms_id,em_act.synchronicity.clone(),new_targets);
                return vec![Interaction::Emission(new_emission)];
            }
        },
        &Interaction::Reception(ref rc_act) => {
            let mut new_targets = rc_act.recipients.clone();
            new_targets.sort();
            if new_targets != rc_act.recipients {
                let new_reception = ReceptionAction::new(rc_act.origin_gt_id.clone(),rc_act.ms_id,rc_act.synchronicity.clone(),new_targets);
                return vec![Interaction::Reception(new_reception)];
            }
        },
        _ => {}
    }
    return vec![];
}


pub fn transfo_unfold_action(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Emission(ref em_act) => {
            if em_act.targets.len() == 0 {
                // nothing to transform
                return vec![];
            } else {
                let mut gates_targets = vec![];
                let mut targets_as_ints = vec![];
                for tr_ref in &em_act.targets {
                    match *tr_ref {
                        EmissionTargetRef::Lifeline(lf_id) => {
                            targets_as_ints.push(
                                Interaction::Reception(
                                    ReceptionAction::new(
                                        None,
                                        em_act.ms_id,
                                        CommunicationSynchronicity::Asynchronous,
                                        vec![lf_id])
                                )
                            );
                        },
                        EmissionTargetRef::Gate(gt_id) => {
                            gates_targets.push(EmissionTargetRef::Gate(gt_id));
                        }
                    }
                }
                if targets_as_ints.len() == 0 {
                    // nothing to transform
                    return vec![];
                }
                let new_em = Interaction::Emission(
                    EmissionAction::new(em_act.origin_lf_id,em_act.ms_id,CommunicationSynchronicity::Asynchronous,gates_targets)
                );
                let receptions = fold_recursive_par_frags(&mut targets_as_ints.iter().collect());
                let new_int = Interaction::Strict(
                    Box::new(new_em),
                    Box::new(receptions)
                );
                return vec![new_int];
            }
        },
        &Interaction::Reception(ref rc_act) => {
            match rc_act.recipients.len() {
                0 => {
                    panic!("malformed interaction : reception action with 0 recipients")
                },
                1 => {
                    // nothing to transform
                    return vec![];
                },
                _ => {
                    let mut new_recs = vec![];
                    for rec_lf_id in &rc_act.recipients {
                        new_recs.push(
                            Interaction::Reception(
                                ReceptionAction::new(
                                    rc_act.origin_gt_id.clone(),
                                    rc_act.ms_id,
                                    CommunicationSynchronicity::Asynchronous,
                                    vec![*rec_lf_id]
                                )
                            )
                        );
                    }
                    return vec![fold_recursive_par_frags(&mut new_recs.iter().collect())];
                }
            }
        },
        _ => {}
    }
    return vec![];
}
