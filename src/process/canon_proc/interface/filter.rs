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






use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::abstract_proc::generic::AbstractFilter;
use crate::process::canon_proc::interface::conf::CanonizationConfig;

pub struct CanonizationFilterCriterion;


pub enum CanonizationFilter {
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl fmt::Display for CanonizationFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CanonizationFilter::MaxProcessDepth(num) => {
                return format!("MaxDepth={}",num);
            },
            CanonizationFilter::MaxNodeNumber(num) => {
                return format!("MaxNum={}",num);
            }
        }
    }
}

impl AbstractFilter<CanonizationConfig>  for CanonizationFilter {

    fn apply_filter(&self, depth: u32,
                    node_counter: u32,
                    criterion: &CanonizationFilterCriterion) -> Option<FilterEliminationKind> {
        match self {
            CanonizationFilter::MaxProcessDepth( max_depth ) => {
                if depth > *max_depth {
                    return Some( FilterEliminationKind::MaxProcessDepth );
                }
            },
            CanonizationFilter::MaxNodeNumber( max_node_number ) => {
                if node_counter >= *max_node_number {
                    return Some( FilterEliminationKind::MaxNodeNumber );
                }
            }
        }
        return None;
    }

}

