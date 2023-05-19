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
use crate::process::explo::conf::{ExplorationConfig, ExplorationStaticLocalVerdictAnalysisProof};
use crate::process::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::process::explo::filter::filter::ExplorationFilterCriterion;
use crate::process::explo::node::ExplorationNodeKind;
use crate::process::explo::step::ExplorationStepKind;
use crate::process::explo::verdict::local::ExplorationLocalVerdict;


pub struct ExplorationProcessHandler {}

impl AbstractProcessHandler<ExplorationConfig> for ExplorationProcessHandler {

    fn process_new_step(context: &ExplorationContext,
                        param : &ExplorationParameterization,
                        parent_state: &GenericNode<ExplorationNodeKind>,
                        step_to_process: &GenericStep<ExplorationStepKind>,
                        new_state_id: u32,
                        node_counter: u32) -> ExplorationNodeKind {
        match step_to_process.kind {
            ExplorationStepKind::Execute( ref frt_elt ) => {
                let new_loop_depth = parent_state.kind.loop_depth + frt_elt.max_loop_depth;
                let exe_result = execute_interaction(&parent_state.kind.interaction,
                                                     &frt_elt.position,
                                                     &frt_elt.target_lf_ids,
                                                     false);
                ExplorationNodeKind::new(exe_result.interaction,new_loop_depth)
            }
        }
    }

    fn get_criterion(context: &ExplorationContext,
                     param : &ExplorationParameterization,
                     parent_state: &GenericNode<ExplorationNodeKind>,
                     step_to_process: &GenericStep<ExplorationStepKind>,
                     new_state_id: u32,
                     node_counter: u32) -> ExplorationFilterCriterion {
        match step_to_process.kind {
            ExplorationStepKind::Execute( ref frt_elt ) => {
                let loop_depth = parent_state.kind.loop_depth + frt_elt.max_loop_depth;
                ExplorationFilterCriterion{loop_depth}
            }
        }
    }

    fn collect_next_steps(context: &ExplorationContext,
                          param : &ExplorationParameterization,
                          parent_state_id: u32,
                          parent_node_kind: &ExplorationNodeKind)
                -> (u32, Vec<GenericStep<ExplorationStepKind>>) {
        
        let mut glob_front = global_frontier(&parent_node_kind.interaction,&None);
        // reverse so that when one pops from right to left the actions appear from the top to the bottom
        glob_front.reverse();
        // ***
        let mut id_as_child : u32 = 0;
        // ***
        let mut to_enqueue : Vec<GenericStep<ExplorationStepKind>> = vec![];
        for front_pos in glob_front {
            let step = GenericStep::new(parent_state_id,
                                        id_as_child,
                                        ExplorationStepKind::Execute(front_pos));
            id_as_child = id_as_child + 1;
            to_enqueue.push( step );
        }
        return (id_as_child,to_enqueue);
    }

    fn get_local_verdict_when_no_child(context: &ExplorationContext,
                                       param : &ExplorationParameterization,
                                       node_kind: &ExplorationNodeKind) -> ExplorationLocalVerdict {
        if node_kind.interaction.express_empty() {
            ExplorationLocalVerdict::Accepting
        } else {
            ExplorationLocalVerdict::DeadLocked
        }
    }

    fn get_local_verdict_from_static_analysis(context: &ExplorationContext,
                                              param : &ExplorationParameterization,
                                              node_kind: &mut ExplorationNodeKind)
                -> Option<(ExplorationLocalVerdict,ExplorationStaticLocalVerdictAnalysisProof)> {
        if node_kind.interaction.express_empty() {
            Some((ExplorationLocalVerdict::Accepting,ExplorationStaticLocalVerdictAnalysisProof{}))
        } else {
            None
        }
    }

    fn pursue_process_after_static_verdict(context: &ExplorationContext,
                                           param : &ExplorationParameterization,
                                           loc_verd: &ExplorationLocalVerdict) -> bool {
        match loc_verd {
            ExplorationLocalVerdict::Accepting => {
                true
            },
            ExplorationLocalVerdict::DeadLocked => {
                false
            }
        }
    }
}