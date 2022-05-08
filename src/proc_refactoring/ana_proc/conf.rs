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
use crate::proc_refactoring::ana_proc::filter::{AnalysisFilter, AnalysisFilterCriterion};
use crate::proc_refactoring::ana_proc::node::AnalysisNodeKind;
use crate::proc_refactoring::ana_proc::priorities::AnalysisPriorities;
use crate::proc_refactoring::ana_proc::step::AnalysisStepKind;

pub struct AnalysisConfig {}

impl AbstractConfiguration for AnalysisConfig {
    type NodeKind = AnalysisNodeKind;
    type StepKind = AnalysisStepKind;
    type PrioritiesConf = AnalysisPriorities;
    type Filter = AnalysisFilter;
    type FilterCriterion = AnalysisFilterCriterion;
}