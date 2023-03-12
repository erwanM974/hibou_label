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




use crate::core::execution::semantics::frontier::FrontierElement;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::process::abstract_proc::generic::AbstractStepKind;
use crate::process::explo_proc::interface::conf::ExplorationConfig;
use crate::process::explo_proc::interface::priorities::ExplorationPriorities;

pub enum ExplorationStepKind {
    Execute(FrontierElement)
}

impl AbstractStepKind<ExplorationConfig> for ExplorationStepKind {

    fn get_priority(&self, process_priorities: &ExplorationPriorities) -> i32 {
        match self {
            ExplorationStepKind::Execute( frt_elt ) => {
                let mut priority : i32 = 0;
                // ***
                let (num_em,num_rc) = TraceAction::get_actions_kinds(&frt_elt.target_actions);
                priority += num_em*process_priorities.emission;
                priority += num_rc*process_priorities.reception;
                // ***
                priority += process_priorities.multi_rdv * ( frt_elt.target_actions.len() as i32);
                priority += process_priorities.in_loop * ( frt_elt.max_loop_depth as i32);
                // ***
                return priority;
            }
        }
    }

}