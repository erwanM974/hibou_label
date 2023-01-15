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




use crate::core::language::syntax::action::{EmissionAction, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;

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