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
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;

use crate::process::explo::verdict::local::ExplorationLocalVerdict;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ExplorationGlobalVerdict{
    HasDeadLocks,
    FoundNoDeadLocks
}

impl fmt::Display for ExplorationGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExplorationGlobalVerdict::HasDeadLocks => {
                write!(f,"HasDeadLocks")
            },
            ExplorationGlobalVerdict::FoundNoDeadLocks => {
                write!(f,"FoundNoDeadLocks")
            }
        }
    }
}

impl AbstractGlobalVerdict<ExplorationLocalVerdict> for ExplorationGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        true
    }

    fn get_baseline_verdict() -> Self {
        ExplorationGlobalVerdict::FoundNoDeadLocks
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &ExplorationLocalVerdict) -> Self {
        match self {
            ExplorationGlobalVerdict::HasDeadLocks => {
                self
            },
            ExplorationGlobalVerdict::FoundNoDeadLocks => {
                match local_verdict {
                    ExplorationLocalVerdict::DeadLocked => {
                        ExplorationGlobalVerdict::HasDeadLocks
                    },
                    _ => {
                        self
                    }
                }
            }
        }
    }

    fn is_goal_reached(&self,
                       goal: &Option<Self>) -> bool {
        false
    }

    fn update_knowing_nodes_were_filtered_out(self,
                                              has_filtered_nodes: bool) -> Self {
        self
    }

}


