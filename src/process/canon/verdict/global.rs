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
use crate::core::language::syntax::interaction::Interaction;
use crate::process::canon::verdict::local::CanonizationLocalVerdict;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CanonizationGlobalVerdict {
    pub canonized_ints : Vec<Interaction>
}

impl fmt::Display for CanonizationGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractGlobalVerdict<CanonizationLocalVerdict> for CanonizationGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        false
    }

    fn get_baseline_verdict() -> Self {
        CanonizationGlobalVerdict{canonized_ints:vec![]}
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &CanonizationLocalVerdict) -> Self {
        let mut ints = self.canonized_ints;
        ints.push(local_verdict.got_interaction.clone());
        Self{canonized_ints:ints}
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


