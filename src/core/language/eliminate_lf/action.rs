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
use crate::core::language::eliminate_lf::eliminable::LifelineEliminable;
use crate::core::language::syntax::action::{EmissionAction, EmissionTargetRef, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;



impl LifelineEliminable for EmissionAction {

    fn eliminate_lifelines(&self, lfs_to_remove: &BTreeSet<usize>) -> Interaction {
        if lfs_to_remove.contains(&self.origin_lf_id) {
            let mut has_lf_tars = false;
            let mut target_lfs : Vec<usize> = Vec::new();
            for tar_ref in &self.targets {
                match tar_ref {
                    EmissionTargetRef::Lifeline( tar_lf_id ) => {
                        if !lfs_to_remove.contains( tar_lf_id ) {
                            has_lf_tars = true;
                            target_lfs.push(  *tar_lf_id );
                        }
                    },
                    EmissionTargetRef::Gate( _ ) => {}
                }
            }
            // ***
            if has_lf_tars {
                let new_act = ReceptionAction::new(None,
                                                      self.ms_id,
                                                      self.synchronicity.clone(),
                                                      target_lfs);
                return Interaction::Reception( new_act );
            } else {
                return Interaction::Empty;
            }
        } else {
            let mut targets : Vec<EmissionTargetRef> = Vec::new();
            for tar_ref in &self.targets {
                match tar_ref {
                    EmissionTargetRef::Lifeline( tar_lf_id ) => {
                        if !lfs_to_remove.contains( tar_lf_id ) {
                            targets.push(  EmissionTargetRef::Lifeline( *tar_lf_id ) );
                        }
                    },
                    EmissionTargetRef::Gate( tar_gt_id ) => {
                        targets.push(  EmissionTargetRef::Gate( *tar_gt_id ) );
                    }
                }
            }
            let new_act = EmissionAction::new(self.origin_lf_id,
                                                 self.ms_id,
                                                 self.synchronicity.clone(),
                                                 targets);
            return Interaction::Emission( new_act );
        }
    }
}



impl LifelineEliminable for ReceptionAction {
    fn eliminate_lifelines(&self, lfs_to_remove: &BTreeSet<usize>) -> Interaction {
        let mut has_lf_tars = false;
        let mut target_lfs : Vec<usize> = Vec::new();
        for tar_lf_id in &self.recipients {
            if !lfs_to_remove.contains( tar_lf_id ) {
                has_lf_tars = true;
                target_lfs.push(  *tar_lf_id );
            }
        }
        // ***
        if has_lf_tars {
            let new_act = ReceptionAction::new(self.origin_gt_id.clone(),
                                                  self.ms_id,
                                                  self.synchronicity.clone(),
                                                  target_lfs);
            return Interaction::Reception( new_act );
        } else {
            return Interaction::Empty;
        }
    }
}