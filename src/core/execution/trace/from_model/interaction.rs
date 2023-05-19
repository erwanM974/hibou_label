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
use crate::core::execution::trace::from_model::from_model::{InteractionInterpretableAsTraceAction, PrimitiveInterpretableAsTraceAction};
use crate::core::execution::trace::trace::{TraceAction};
use crate::core::language::syntax::interaction::Interaction;

impl InteractionInterpretableAsTraceAction for Interaction {
    fn get_all_trace_actions(&self) -> BTreeSet<TraceAction> {
        match &self {
            &Interaction::Empty => {
                return btreeset!{};
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.get_all_atomic_actions();
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.get_all_atomic_actions();
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Seq(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Par(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Loop(_, i1) => {
                return i1.get_all_trace_actions();
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    fn get_trace_actions_outside_loops(&self) -> BTreeSet<TraceAction> {
        match &self {
            &Interaction::Empty => {
                return btreeset!{};
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.get_all_atomic_actions();
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.get_all_atomic_actions();
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Seq(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Par(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            &Interaction::Loop(_, i1) => {
                return btreeset!{};
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                let mut acts1 = i1.get_all_trace_actions();
                let acts2 = i2.get_all_trace_actions();
                acts1.extend(acts2);
                return acts1;
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    fn get_atomic_actions_number(&self) -> usize {
        match &self {
            &Interaction::Empty => {
                return 0;
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.get_all_atomic_actions().len();
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.get_all_atomic_actions().len();
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Par(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Loop(_, i1) => {
                return i1.get_atomic_actions_number();
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    fn get_atomic_actions_number_outside_loops(&self) -> usize {
        match &self {
            &Interaction::Empty => {
                return 0;
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.get_all_atomic_actions().len();
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.get_all_atomic_actions().len();
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Par(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            &Interaction::Loop(_, i1) => {
                return 0;
            },
            &Interaction::Sync(_, ref i1, ref i2) => {
                return i1.get_atomic_actions_number() + i2.get_atomic_actions_number();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }
}