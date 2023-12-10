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


use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;

use crate::core::execution::semantics::execute::execute_interaction;
use crate::core::execution::semantics::frontier::global_frontier;
use crate::process::canon::conf::{CanonizationConfig, CanonizationStaticLocalVerdictAnalysisProof};
use crate::process::canon::context::CanonizationContext;
use crate::process::canon::filter::filter::CanonizationFilterCriterion;
use crate::process::canon::node::CanonizationNodeKind;
use crate::process::canon::param::phase::CanonizationParameterization;
use crate::process::canon::step::CanonizationStepKind;
use crate::process::canon::verdict::local::CanonizationLocalVerdict;


pub struct CanonizationProcessHandler {}

impl AbstractProcessHandler<CanonizationConfig> for CanonizationProcessHandler {

    fn process_new_step(context: &CanonizationContext,
                        param : &CanonizationParameterization,
                        parent_state: &GenericNode<CanonizationNodeKind>,
                        step_to_process: &GenericStep<CanonizationStepKind>,
                        new_state_id: u32,
                        node_counter: u32) -> CanonizationNodeKind {
        match step_to_process.kind {
            CanonizationStepKind::GoToNextPhase => {
                CanonizationNodeKind::new(parent_state.kind.interaction.clone(),parent_state.kind.phase + 1)
            },
            CanonizationStepKind::Transform(ref result) => {
                CanonizationNodeKind::new(result.result.clone(),parent_state.kind.phase)
            }
        }
    }

    fn get_criterion(context: &CanonizationContext,
                     param : &CanonizationParameterization,
                     parent_state: &GenericNode<CanonizationNodeKind>,
                     step_to_process: &GenericStep<CanonizationStepKind>,
                     new_state_id: u32,
                     node_counter: u32) -> CanonizationFilterCriterion {
        CanonizationFilterCriterion{}
    }

    fn collect_next_steps(context: &CanonizationContext,
                          param : &CanonizationParameterization,
                          parent_node_kind: &CanonizationNodeKind)
                -> Vec<CanonizationStepKind> {
        match param.phases.get(parent_node_kind.phase as usize) {
            None => {
                vec![]
            },
            Some(phase) => {
                let transfos = phase.get_transfos(&parent_node_kind.interaction,param.get_all);
                if transfos.is_empty() {
                    vec![CanonizationStepKind::GoToNextPhase]
                } else {
                    transfos.into_iter()
                        .map(|r|
                            CanonizationStepKind::Transform(r)
                        )
                        .collect()
                }
            }
        }
    }

    fn get_local_verdict_when_no_child(_context: &CanonizationContext,
                                       _param : &CanonizationParameterization,
                                       node_kind: &CanonizationNodeKind) -> CanonizationLocalVerdict {
        CanonizationLocalVerdict{got_interaction:node_kind.interaction.clone()}
    }

    fn get_local_verdict_from_static_analysis(_context: &CanonizationContext,
                                              _param : &CanonizationParameterization,
                                              node_kind: &mut CanonizationNodeKind)
                -> Option<(CanonizationLocalVerdict,CanonizationStaticLocalVerdictAnalysisProof)> {
        None
    }

    fn pursue_process_after_static_verdict(_context: &CanonizationContext,
                                           _param : &CanonizationParameterization,
                                           loc_verd: &CanonizationLocalVerdict) -> bool {
        true
    }
}