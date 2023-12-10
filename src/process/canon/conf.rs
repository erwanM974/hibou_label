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



use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use crate::process::canon::context::CanonizationContext;
use crate::process::canon::filter::elim::CanonizationFilterEliminationKind;
use crate::process::canon::filter::filter::CanonizationFilterCriterion;
use crate::process::canon::handling::handler::CanonizationProcessHandler;
use crate::process::canon::node::CanonizationNodeKind;
use crate::process::canon::param::phase::CanonizationParameterization;
use crate::process::canon::priorities::CanonizationPriorities;
use crate::process::canon::step::CanonizationStepKind;
use crate::process::canon::verdict::global::CanonizationGlobalVerdict;
use crate::process::canon::verdict::local::CanonizationLocalVerdict;


pub struct CanonizationConfig {}

pub struct CanonizationStaticLocalVerdictAnalysisProof{}

impl AbstractProcessConfiguration for CanonizationConfig {
    type Context = CanonizationContext;
    type Parameterization = CanonizationParameterization;
    type NodeKind = CanonizationNodeKind;
    type StepKind = CanonizationStepKind;
    type Priorities = CanonizationPriorities;
    type FilterCriterion = CanonizationFilterCriterion;
    type FilterEliminationKind = CanonizationFilterEliminationKind;
    type LocalVerdict = CanonizationLocalVerdict;
    type StaticLocalVerdictAnalysisProof = CanonizationStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = CanonizationGlobalVerdict;
    type ProcessHandler = CanonizationProcessHandler;
}
