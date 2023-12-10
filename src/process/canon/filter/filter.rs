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
use crate::process::canon::filter::elim::CanonizationFilterEliminationKind;


pub struct CanonizationFilterCriterion {}

impl fmt::Display for CanonizationFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

pub enum CanonizationFilter {}

impl fmt::Display for CanonizationFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractFilter<CanonizationFilterCriterion,CanonizationFilterEliminationKind>  for CanonizationFilter {

    fn apply_filter(&self,
                    depth: u32,
                    node_counter: u32,
                    criterion: &CanonizationFilterCriterion) -> Option<CanonizationFilterEliminationKind> {
        None
    }

}

