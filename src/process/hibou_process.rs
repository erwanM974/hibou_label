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
use crate::core::semantics::frontier::make_frontier;
use crate::core::semantics::execute::execute;
use crate::process::verdicts::CoverageVerdict;




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
pub enum NextToProcessKind {
    Execute(Position)
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

pub struct ProcessQueue {
    queues : HashMap<i32,Vec<NextToProcess>>
}

impl ProcessQueue {
    pub fn new() -> ProcessQueue {
        return ProcessQueue{queues:HashMap::new()}
    }

    pub fn insert_item_left(&mut self,node:NextToProcess,priority:i32) {
        match self.queues.get_mut(&priority) {
            None => {
                self.queues.insert(priority,vec![node]);
            },
            Some( queue ) => {
                queue.insert(0,node);
            }
        }
    }

    pub fn insert_item_right(&mut self,node:NextToProcess,priority:i32) {
        match self.queues.get_mut(&priority) {
            None => {
                self.queues.insert(priority,vec![node]);
            },
            Some( queue ) => {
                queue.push(node);
            }
        }
    }

    pub fn get_next(&mut self) -> Option<NextToProcess> {
        let mut keys : Vec<i32> = self.queues.keys().cloned().collect();
        keys.sort_by_key(|k| Reverse(*k));
        for k in keys {
            match self.queues.get_mut(&k) {
                None => {},
                Some( queue ) => {
                    if queue.len() > 0 {
                        let next = queue.remove(0);
                        return Some(next);
                    }
                }
            }
        }
        return None;
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
    DFS
}

impl std::string::ToString for HibouSearchStrategy {
    fn to_string(&self) -> String {
        match self {
            HibouSearchStrategy::BFS => {
                return "Breadth First Search".to_string();
            },
            HibouSearchStrategy::DFS => {
                return "Depth First Search".to_string();
            }
        }
    }
}

pub enum SemanticKind {
    Accept,
    Prefix
}


impl std::string::ToString for SemanticKind {
    fn to_string(&self) -> String {
        match self {
            SemanticKind::Accept => {
                return "accept".to_string();
            },
            SemanticKind::Prefix => {
                return "prefix".to_string();
            }
        }
    }
}