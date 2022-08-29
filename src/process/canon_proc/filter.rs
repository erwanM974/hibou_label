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



use crate::proc_refactoring::abstract_proc::AbstractFilter;
use crate::proc_refactoring::explo_proc::conf::ExplorationConfig;
use crate::process::hibou_process::{FilterEliminationKind};

pub struct ExplorationFilterCriterion {
    pub loop_depth : u32
}



pub enum ExplorationFilter {
    MaxLoopInstanciation(u32),
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl std::string::ToString for ExplorationFilter {
    fn to_string(&self) -> String {
        match self {
            ExplorationFilter::MaxLoopInstanciation(num) => {
                return format!("MaxLoop={}",num);
            },
            ExplorationFilter::MaxProcessDepth(num) => {
                return format!("MaxDepth={}",num);
            },
            ExplorationFilter::MaxNodeNumber(num) => {
                return format!("MaxNum={}",num);
            }
        }
    }
}

impl AbstractFilter<ExplorationConfig>  for ExplorationFilter {

    fn apply_filter(&self, depth: u32, node_counter: u32, criterion: &ExplorationFilterCriterion) -> Option<FilterEliminationKind> {
        match self {
            ExplorationFilter::MaxProcessDepth( max_depth ) => {
                if depth > *max_depth {
                    return Some( FilterEliminationKind::MaxProcessDepth );
                }
            },
            ExplorationFilter::MaxLoopInstanciation( loop_num ) => {
                if criterion.loop_depth > *loop_num {
                    return Some( FilterEliminationKind::MaxLoopInstanciation );
                }
            },
            ExplorationFilter::MaxNodeNumber( max_node_number ) => {
                if node_counter >= *max_node_number {
                    return Some( FilterEliminationKind::MaxNodeNumber );
                }
            }
        }
        return None;
    }

}

