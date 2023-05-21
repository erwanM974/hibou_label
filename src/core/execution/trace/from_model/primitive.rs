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


use std::collections::BTreeSet;

use crate::core::execution::trace::from_model::from_model::{PrimitiveInterpretableAsTraceAction};
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::language::syntax::action::{EmissionAction, EmissionTargetRef, ReceptionAction};


impl PrimitiveInterpretableAsTraceAction for EmissionAction {
    fn get_all_atomic_actions(&self) -> BTreeSet<TraceAction> {
        let mut contents : BTreeSet<TraceAction> = btreeset!{};
        contents.insert( self.get_first_atomic_action() );
        for target_ref in &self.targets {
            match target_ref {
                &EmissionTargetRef::Lifeline(tar_lf_id) => {
                    contents.insert( TraceAction::new(tar_lf_id,TraceActionKind::Reception, self.ms_id) );
                },
                _ => {}
            }
        }
        return contents;
    }

    fn get_first_atomic_action(&self) -> TraceAction {
        return TraceAction::new(self.origin_lf_id,
                                TraceActionKind::Emission,
                                self.ms_id);
    }

    fn get_specific_atomic_action(&self,idx : usize) -> TraceAction {
        match self.targets.get(idx) {
            None => {
                panic!();
            },
            Some( target_ref) => {
                match target_ref {
                    EmissionTargetRef::Lifeline(tar_lf_id) => {
                        return TraceAction::new(*tar_lf_id,
                                                TraceActionKind::Reception,
                                                self.ms_id);
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }
}


impl PrimitiveInterpretableAsTraceAction for ReceptionAction {
    fn get_all_atomic_actions(&self) -> BTreeSet<TraceAction> {
        let mut contents : BTreeSet<TraceAction> = btreeset!{};
        for rc_lf_id in &self.recipients {
            contents.insert( TraceAction::new(*rc_lf_id,
                                              TraceActionKind::Reception,
                                              self.ms_id) );
        }
        return contents;
    }

    fn get_first_atomic_action(&self) -> TraceAction {
        return self.get_specific_atomic_action(0);
    }

    fn get_specific_atomic_action(&self, idx : usize) -> TraceAction {
        return TraceAction::new(*self.recipients.get(idx).unwrap(),
                                TraceActionKind::Reception,
                                self.ms_id);
    }
}