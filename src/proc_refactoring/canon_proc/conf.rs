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




use crate::proc_refactoring::abstract_proc::AbstractConfiguration;
use crate::proc_refactoring::canon_proc::filter::{ExplorationFilter, ExplorationFilterCriterion};
use crate::proc_refactoring::canon_proc::node::CanonizationNodeKind;
use crate::proc_refactoring::canon_proc::priorities::CanonizationPriorities;
use crate::proc_refactoring::canon_proc::step::CanonizationStepKind;

pub struct CanonizationConfig {}

impl AbstractConfiguration for CanonizationConfig {
    type NodeKind = CanonizationNodeKind;
    type StepKind = CanonizationStepKind;
    type PrioritiesConf = CanonizationPriorities;
    type Filter = CanonizationFilter;
    type FilterCriterion = CanonizationFilterCriterion;
}