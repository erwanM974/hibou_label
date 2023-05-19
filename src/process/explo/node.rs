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




use std::hash::Hash;

use graph_process_manager_core::manager::config::AbstractNodeKind;
use crate::core::language::syntax::interaction::Interaction;


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct ExplorationNodeKind {
    pub interaction : Interaction,
    pub loop_depth : u32
}

impl ExplorationNodeKind {
    pub fn new(interaction: Interaction,
               loop_depth: u32) -> Self {
        ExplorationNodeKind { interaction, loop_depth }
    }
}


impl AbstractNodeKind for ExplorationNodeKind {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        if self.interaction == memoized_node.interaction {
            if self.loop_depth >= memoized_node.loop_depth {
                // means that we might have already explored more from the memoized node
                return true;
            }
        }
        return false;
    }
}

