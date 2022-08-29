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




use crate::process::abstract_proc::generic::AbstractConfiguration;
use crate::process::explo_proc::interface::filter::{ExplorationFilter, ExplorationFilterCriterion};
use crate::process::explo_proc::interface::logger::ExplorationLogger;
use crate::process::explo_proc::interface::node::ExplorationNodeKind;
use crate::process::explo_proc::interface::priorities::ExplorationPriorities;
use crate::process::explo_proc::interface::step::ExplorationStepKind;

pub struct ExplorationConfig {}

impl AbstractConfiguration for ExplorationConfig {
    type NodeKind = ExplorationNodeKind;
    type StepKind = ExplorationStepKind;
    type Priorities = ExplorationPriorities;
    type Filter = ExplorationFilter;
    type FilterCriterion = ExplorationFilterCriterion;
    type Logger = Box<dyn ExplorationLogger>;
}

