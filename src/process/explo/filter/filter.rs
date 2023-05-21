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

use graph_process_manager_core::handler::filter::AbstractFilter;

use crate::process::explo::filter::elim::ExplorationFilterEliminationKind;


pub struct ExplorationFilterCriterion {
    pub loop_depth : u32
}

impl fmt::Display for ExplorationFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"max loop depth : {:}", self.loop_depth)
    }
}

pub enum ExplorationFilter {
    MaxLoopInstanciation(u32),
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl fmt::Display for ExplorationFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExplorationFilter::MaxLoopInstanciation(num) => {
                write!(f,"MaxLoop={}",num)
            },
            ExplorationFilter::MaxProcessDepth(num) => {
                write!(f,"MaxDepth={}",num)
            },
            ExplorationFilter::MaxNodeNumber(num) => {
                write!(f,"MaxNum={}",num)
            }
        }
    }
}

impl AbstractFilter<ExplorationFilterCriterion,ExplorationFilterEliminationKind>  for ExplorationFilter {

    fn apply_filter(&self,
                    depth: u32,
                    node_counter: u32,
                    criterion: &ExplorationFilterCriterion) -> Option<ExplorationFilterEliminationKind> {
        match self {
            ExplorationFilter::MaxProcessDepth( max_depth ) => {
                if depth > *max_depth {
                    return Some( ExplorationFilterEliminationKind::MaxProcessDepth );
                }
            },
            ExplorationFilter::MaxLoopInstanciation( loop_num ) => {
                if criterion.loop_depth > *loop_num {
                    return Some( ExplorationFilterEliminationKind::MaxLoopInstanciation );
                }
            },
            ExplorationFilter::MaxNodeNumber( max_node_number ) => {
                if node_counter >= *max_node_number {
                    return Some( ExplorationFilterEliminationKind::MaxNodeNumber );
                }
            }
        }
        return None;
    }

}

