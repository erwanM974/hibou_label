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




use std::fmt;
use graph_process_manager_core::delegate::priorities::AbstractPriorities;
use crate::core::execution::trace::trace::TraceAction;
use crate::process::canon::step::CanonizationStepKind;
use crate::process::explo::step::ExplorationStepKind;


pub struct CanonizationPriorities {}

impl CanonizationPriorities {

    pub fn new() -> CanonizationPriorities {
        return CanonizationPriorities{};
    }

    pub fn default() -> CanonizationPriorities {
        CanonizationPriorities::new()
    }
}

impl fmt::Display for CanonizationPriorities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractPriorities<CanonizationStepKind> for CanonizationPriorities {
    fn get_priority_of_step(&self, step: &CanonizationStepKind) -> i32 {
        0
    }
}

