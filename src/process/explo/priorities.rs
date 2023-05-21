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
use crate::process::explo::step::ExplorationStepKind;


pub struct ExplorationPriorities {
    pub emission : i32,
    pub reception : i32,
    pub multi_rdv : i32,
    pub in_loop : i32
}

impl ExplorationPriorities {

    pub fn new(emission : i32,
               reception : i32,
               multi_rdv : i32,
               in_loop : i32) -> ExplorationPriorities {
        return ExplorationPriorities{emission,reception,multi_rdv,in_loop};
    }

    pub fn default() -> ExplorationPriorities {
        return ExplorationPriorities::new(0,0,0,0);
    }
}

impl fmt::Display for ExplorationPriorities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "[emission={:},reception={:},multi-rdv={:},loop={:}]",
               self.emission,
               self.reception,
               self.multi_rdv,
               self.in_loop)
    }
}

impl AbstractPriorities<ExplorationStepKind> for ExplorationPriorities {
    fn get_priority_of_step(&self, step: &ExplorationStepKind) -> i32 {
        match step {
            ExplorationStepKind::Execute( frt_elt ) => {
                let mut priority : i32 = 0;
                // ***
                let (num_em,num_rc) = TraceAction::get_actions_kinds(&frt_elt.target_actions);
                priority += num_em*self.emission;
                priority += num_rc*self.reception;
                // ***
                priority += self.multi_rdv * ( frt_elt.target_actions.len() as i32);
                priority += self.in_loop * ( frt_elt.max_loop_depth as i32);
                // ***
                return priority;
            }
        }
    }
}

