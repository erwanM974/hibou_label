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
use std::collections::HashMap;

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction};
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;
use crate::core::semantics::execute::execute;
use crate::process::verdicts::CoverageVerdict;

#[derive(Clone, PartialEq, Debug)]
pub struct ProcessStateNode {
    pub state_id : Vec<u32>,
    pub interaction : Interaction,
    pub rem_front_or_match : Vec<Position>,
    pub id_for_next_child : u32,
    pub multitrace : Option<AnalysableMultiTrace>,
    pub previous_loop_instanciations : u32
}

impl ProcessStateNode {
    pub fn new(state_id : Vec<u32>,
               interaction : Interaction,
               // ***
               rem_front_or_match : Vec<Position>,
               id_for_next_child : u32,
               // ***
               multitrace : Option<AnalysableMultiTrace>,
               previous_loop_instanciations : u32) -> ProcessStateNode {
        return ProcessStateNode {
            state_id,
            interaction,
            rem_front_or_match,
            id_for_next_child,
            multitrace,
            previous_loop_instanciations
        };
    }
}

pub enum HibouPreFilter {
    MaxLoopInstanciation(u32),
    MaxProcessDepth(usize),
    MaxNodeNumber(u32)
}

pub enum FilterEliminationKind {
    MaxLoopInstanciation,
    MaxProcessDepth,
    MaxNodeNumber
}

impl std::string::ToString for FilterEliminationKind {
    fn to_string(&self) -> String {
        match self {
            FilterEliminationKind::MaxLoopInstanciation => {
                return "MaxLoop".to_string();
            },
            FilterEliminationKind::MaxProcessDepth => {
                return "MaxDepth".to_string();
            },
            FilterEliminationKind::MaxNodeNumber => {
                return "MaxNum".to_string();
            }
        }
    }
}


pub enum HibouSearchStrategy {
    BFS,
    DFS
}



pub struct ProcessStepResult {
    pub put_back_state_node : Option<ProcessStateNode>,
    pub new_state_node : Option<ProcessStateNode>
}
