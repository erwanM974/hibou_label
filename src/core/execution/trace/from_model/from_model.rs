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
use crate::core::execution::trace::trace::TraceAction;

pub trait PrimitiveInterpretableAsTraceAction {

    fn get_all_atomic_actions(&self) -> HashSet<TraceAction>;

    fn get_first_atomic_action(&self) -> TraceAction;

    fn get_specific_atomic_action(&self, idx : usize) -> TraceAction;

}


pub trait InteractionInterpretableAsTraceAction {

    fn get_all_trace_actions(&self) -> HashSet<TraceAction>;

    fn get_trace_actions_outside_loops(&self) -> HashSet<TraceAction>;

    fn get_atomic_actions_number(&self) -> usize;

    fn get_atomic_actions_number_outside_loops(&self) -> usize;

}
