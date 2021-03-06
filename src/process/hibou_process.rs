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
use std::cmp::Reverse;
use std::collections::{HashSet,HashMap};

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction};
use crate::process::log::ProcessLogger;
use crate::process::verdicts::CoverageVerdict;

use crate::process::priorities::ProcessPriorities;


#[derive(Clone, PartialEq, Debug)]
pub struct MemorizedState {
    pub interaction : Interaction,
    pub multi_trace : Option<AnalysableMultiTrace>,
    pub remaining_ids_to_process : HashSet<u32>,
    pub loop_depth : u32, // number of loop instanciations since intial interaction
    pub depth : u32       // number of execution steps since initial interaction
}

impl MemorizedState {
    pub fn new(interaction : Interaction,
               multi_trace : Option<AnalysableMultiTrace>,
               remaining_ids_to_process : HashSet<u32>,
               loop_depth : u32,
               depth : u32) -> MemorizedState {
        return MemorizedState{interaction,multi_trace,remaining_ids_to_process,loop_depth,depth};
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationStepKind {
    BeforeStart,
    AfterEnd
}

#[derive(Clone, PartialEq, Debug)]
pub enum NextToProcessKind {
    Execute(Position),
    Hide( HashSet<usize> ),
    Simulate(Position,SimulationStepKind)
}

#[derive(Clone, PartialEq, Debug)]
pub struct NextToProcess {
    pub state_id : u32,
    pub id_as_child : u32,
    pub kind : NextToProcessKind
}

impl NextToProcess {
    pub fn new(state_id : u32,
               id_as_child : u32,
               kind : NextToProcessKind) -> NextToProcess {
        return NextToProcess{state_id,id_as_child,kind};
    }
}


pub enum HibouPreFilter {
    MaxLoopInstanciation(u32),
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl std::string::ToString for HibouPreFilter {
    fn to_string(&self) -> String {
        match self {
            HibouPreFilter::MaxLoopInstanciation(num) => {
                return format!("MaxLoop={}",num);
            },
            HibouPreFilter::MaxProcessDepth(num) => {
                return format!("MaxDepth={}",num);
            },
            HibouPreFilter::MaxNodeNumber(num) => {
                return format!("MaxNum={}",num);
            }
        }
    }
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
    DFS,
    GFS(ProcessPriorities)
}

impl std::string::ToString for HibouSearchStrategy {
    fn to_string(&self) -> String {
        match self {
            HibouSearchStrategy::BFS => {
                return "BreadthFS".to_string();
            },
            HibouSearchStrategy::DFS => {
                return "DepthFS".to_string();
            },
            HibouSearchStrategy::GFS(ref pp) => {
                return format!("GreedyBestFS[{:}]", &pp.to_string());
            }
        }
    }
}
