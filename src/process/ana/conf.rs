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
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::language::syntax::interaction::Interaction;

use crate::process::ana::context::AnalysisContext;
use crate::process::ana::filter::elim::AnalysisFilterEliminationKind;
use crate::process::ana::filter::filter::AnalysisFilterCriterion;
use crate::process::ana::handling::handler::AnalysisProcessHandler;
use crate::process::ana::node::flags::MultiTraceAnalysisFlags;
use crate::process::ana::node::node::AnalysisNodeKind;
use crate::process::ana::param::anakind::AnalysisKind;
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::priorities::AnalysisPriorities;
use crate::process::ana::step::AnalysisStepKind;
use crate::process::ana::verdict::global::AnalysisGlobalVerdict;
use crate::process::ana::verdict::local::AnalysisLocalVerdict;



pub struct AnalysisConfig {}


pub struct AnalysisStaticLocalVerdictAnalysisProof {
    pub ana_kind : AnalysisKind,
    pub local_coloc : CoLocalizations,
    pub local_interaction : Interaction,
    pub local_multi_trace : MultiTrace,
    pub local_flags : MultiTraceAnalysisFlags
}

impl AnalysisStaticLocalVerdictAnalysisProof {
    pub fn new(ana_kind: AnalysisKind, local_coloc: CoLocalizations, local_interaction: Interaction, local_multi_trace: MultiTrace, local_flags: MultiTraceAnalysisFlags) -> Self {
        AnalysisStaticLocalVerdictAnalysisProof { ana_kind, local_coloc, local_interaction, local_multi_trace, local_flags }
    }
}

impl AbstractProcessConfiguration for AnalysisConfig {
    type Context = AnalysisContext;
    type Parameterization = AnalysisParameterization;
    type NodeKind = AnalysisNodeKind;
    type StepKind = AnalysisStepKind;
    type Priorities = AnalysisPriorities;
    type FilterCriterion = AnalysisFilterCriterion;
    type FilterEliminationKind = AnalysisFilterEliminationKind;
    type LocalVerdict = AnalysisLocalVerdict;
    type StaticLocalVerdictAnalysisProof = AnalysisStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = AnalysisGlobalVerdict;
    type ProcessHandler = AnalysisProcessHandler;
}
