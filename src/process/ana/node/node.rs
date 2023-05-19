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
use crate::process::ana::node::flags::{MultiTraceAnalysisFlags, TraceAnalysisFlags};


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct AnalysisNodeKind {
    pub interaction : Interaction,
    pub flags : MultiTraceAnalysisFlags,
    pub ana_loop_depth : u32
}

impl AnalysisNodeKind {
    pub fn new(interaction : Interaction,
               flags : MultiTraceAnalysisFlags,
               ana_loop_depth : u32) -> AnalysisNodeKind {
        return AnalysisNodeKind{interaction,flags,ana_loop_depth}
    }
}


impl AbstractNodeKind for AnalysisNodeKind {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        if self.interaction == memoized_node.interaction {
            if self.flags.rem_loop_in_sim > memoized_node.flags.rem_loop_in_sim || self.flags.rem_act_in_sim > memoized_node.flags.rem_act_in_sim {
                return false;
            }
            // ***
            for idx in 0..self.flags.canals.len() {
                let my_canal : &TraceAnalysisFlags  = self.flags.canals.get(idx).unwrap();
                let other_canal : &TraceAnalysisFlags = memoized_node.flags.canals.get(idx).unwrap();
                if my_canal.consumed > other_canal.consumed {
                    return false;
                }
                if my_canal.simulated_after < other_canal.simulated_after {
                    return false;
                }
                if my_canal.simulated_before < other_canal.simulated_before {
                    return false;
                }
                if my_canal.no_longer_observed && !other_canal.no_longer_observed {
                    return false;
                }
            }
            if self.ana_loop_depth >= memoized_node.ana_loop_depth {
                // means that we might have already explored more from the memoized node
                return true;
            }
        }
        return false;
    }
}

