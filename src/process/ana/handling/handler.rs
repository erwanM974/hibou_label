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


use std::collections::HashSet;
use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;
use crate::core::execution::semantics::execute::execute_interaction;
use crate::core::execution::trace::multitrace::Trace;
use crate::core::language::eliminate_lf::eliminable::LifelineEliminable;
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::process::ana::conf::{AnalysisConfig, AnalysisStaticLocalVerdictAnalysisProof};
use crate::process::ana::context::AnalysisContext;
use crate::process::ana::filter::filter::AnalysisFilterCriterion;
use crate::process::ana::handling::local_analysis::{get_local_analysis_starting_data, is_dead_local_analysis};
use crate::process::ana::node::flags::WasMultiTraceConsumedWithSimulation;
use crate::process::ana::node::node::AnalysisNodeKind;
use crate::process::ana::param::anakind::AnalysisKind;
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::step::AnalysisStepKind;
use crate::process::ana::verdict::inconc::InconcReason;
use crate::process::ana::verdict::local::AnalysisLocalVerdict;


pub struct AnalysisProcessHandler {}

impl AbstractProcessHandler<AnalysisConfig> for AnalysisProcessHandler {

    fn process_new_step(context: &AnalysisContext,
                        param : &AnalysisParameterization,
                        parent_state: &GenericNode<AnalysisNodeKind>,
                        step_to_process: &GenericStep<AnalysisStepKind>,
                        new_state_id: u32,
                        node_counter: u32) -> AnalysisNodeKind {
        match step_to_process.kind {
            AnalysisStepKind::EliminateNoLongerObserved( ref coloc_ids_to_hide ) => {
                let lfs_to_remove = context.co_localizations.get_lf_ids_from_coloc_ids(coloc_ids_to_hide);
                let new_interaction = (parent_state.kind.interaction).eliminate_lifelines(&lfs_to_remove);
                // ***
                let new_flags = parent_state.kind.flags.update_on_hide(&context.gen_ctx,coloc_ids_to_hide);
                // ***
                AnalysisNodeKind::new(new_interaction,new_flags,parent_state.kind.ana_loop_depth)
            },
            AnalysisStepKind::Execute( ref frt_elt, ref consu_set, ref sim_map ) => {
                let exe_result = execute_interaction(&parent_state.kind.interaction,
                                                     &frt_elt.position,
                                                     &frt_elt.target_lf_ids,
                                                     true);
                let affected_colos = context.co_localizations.get_coloc_ids_from_lf_ids(&exe_result.affected_lifelines);
                let new_flags = parent_state.kind.flags.update_on_execution(param.ana_kind.get_sim_config(),
                                                                            consu_set,
                                                                            sim_map,&affected_colos,
                                                                            frt_elt.max_loop_depth,
                                                                            context.init_multitrace_length,
                                                                            &exe_result.interaction);
                // ***
                let new_ana_loop_depth = parent_state.kind.ana_loop_depth + frt_elt.max_loop_depth;
                AnalysisNodeKind::new(exe_result.interaction,new_flags,new_ana_loop_depth)
            }
        }
    }

    fn get_criterion(context: &AnalysisContext,
                     param : &AnalysisParameterization,
                     parent_state: &GenericNode<AnalysisNodeKind>,
                     step_to_process: &GenericStep<AnalysisStepKind>,
                     new_state_id: u32,
                     node_counter: u32) -> AnalysisFilterCriterion {
        match step_to_process.kind {
            AnalysisStepKind::EliminateNoLongerObserved( _ ) => {
                AnalysisFilterCriterion{loop_depth:parent_state.kind.ana_loop_depth}
            },
            AnalysisStepKind::Execute( ref frt_elt, _, _ ) => {
                let loop_depth = parent_state.kind.ana_loop_depth + frt_elt.max_loop_depth;
                AnalysisFilterCriterion{loop_depth}
            }
        }
    }

    fn collect_next_steps(context: &AnalysisContext,
                          param : &AnalysisParameterization,
                          parent_node_kind: &AnalysisNodeKind)
                -> Vec<AnalysisStepKind> {

        if !parent_node_kind.flags.is_multi_trace_empty(&context.multi_trace) {
            match &param.ana_kind {
                AnalysisKind::Accept => {
                    param.get_action_matches_in_analysis(
                        param.partial_order_reduction,
                        false,
                        context,
                        &parent_node_kind.interaction,
                        &parent_node_kind.flags)
                },
                AnalysisKind::Prefix => {
                    param.get_action_matches_in_analysis(param.partial_order_reduction,
                                                         false,
                                                         context,
                                                        &parent_node_kind.interaction,

                                                        &parent_node_kind.flags)
                },
                AnalysisKind::Eliminate => {
                    let mut canals_ids_to_hide : HashSet<usize> = hashset!{};
                    for (canal_id,canal_flags) in parent_node_kind.flags.canals.iter().enumerate() {
                        let trace : &Trace = context.multi_trace.get(canal_id).unwrap();
                        if (canal_flags.no_longer_observed == false) && (trace.len() == canal_flags.consumed) {
                            canals_ids_to_hide.insert( canal_id );
                        }
                    }
                    //
                    let insert_hide_step : bool = if canals_ids_to_hide.is_empty() {
                        false
                    } else {
                        true
                        // we could also require that the interaction does not involve the lifelines to hide
                        // it is a good thing to do generally
                        // however, if we want to compute the size of the analysis graph precisely
                        // it messes up the calculations because we may have (L\H1,i,u) and (L\H2,i,u) from different paths
                        // anyways I removed it for now
                        /*
                        let lfs_to_hide = context.co_localizations.get_lf_ids_from_coloc_ids(&canals_ids_to_hide);
                        if parent_node_kind.interaction.involves_any_of(&lfs_to_hide) {
                            true
                        } else {
                            false
                        }*/
                    };
                    //
                    if insert_hide_step {
                        vec![ AnalysisStepKind::EliminateNoLongerObserved(canals_ids_to_hide) ]
                    } else {
                        param.get_action_matches_in_analysis(param.partial_order_reduction,
                                                             true,
                                                             context,
                                                       &parent_node_kind.interaction,

                                                       &parent_node_kind.flags)
                    }
                },
                AnalysisKind::Simulate(_) => {
                    param.get_simulation_matches_in_analysis(context,
                                                       &parent_node_kind.interaction,
                                                       &parent_node_kind.flags)
                }
            }
        } else {
            vec![]
        }
    }

    fn get_local_verdict_when_no_child(context: &AnalysisContext,
                                       param : &AnalysisParameterization,
                                       node_kind: &AnalysisNodeKind) -> AnalysisLocalVerdict {
        if node_kind.flags.is_multi_trace_empty(&context.multi_trace) {
            if node_kind.interaction.express_empty() {
                match param.ana_kind {
                    AnalysisKind::Accept => {
                        return AnalysisLocalVerdict::Cov;
                    },
                    AnalysisKind::Prefix => {
                        return AnalysisLocalVerdict::Cov;
                    },
                    AnalysisKind::Eliminate => {
                        if node_kind.flags.is_any_component_hidden() {
                            if context.co_localizations.are_colocalizations_singletons() {
                                return AnalysisLocalVerdict::MultiPref;
                            } else {
                                return AnalysisLocalVerdict::Inconc(InconcReason::UsingLifelineRemovalWithCoLocalizations);
                            }
                        } else {
                            return AnalysisLocalVerdict::Cov;
                        }
                    },
                    AnalysisKind::Simulate(_) => {
                        match node_kind.flags.is_simulated() {
                            WasMultiTraceConsumedWithSimulation::No => {
                                return AnalysisLocalVerdict::Cov;
                            },
                            WasMultiTraceConsumedWithSimulation::OnlyAfterEnd => {
                                return AnalysisLocalVerdict::MultiPref;
                            },
                            WasMultiTraceConsumedWithSimulation::AsSlice => {
                                return AnalysisLocalVerdict::Slice;
                            }
                        }
                    }
                }
            } else { /* multi-trace empty but interaction does not express empty */
                match param.ana_kind {
                    AnalysisKind::Accept => {
                        return AnalysisLocalVerdict::Out(false);
                    },
                    AnalysisKind::Prefix => {
                        return AnalysisLocalVerdict::TooShort;
                    },
                    AnalysisKind::Eliminate => {
                        if node_kind.flags.is_any_component_hidden() {
                            if context.co_localizations.are_colocalizations_singletons() {
                                return AnalysisLocalVerdict::MultiPref;
                            } else {
                                return AnalysisLocalVerdict::Inconc(InconcReason::UsingLifelineRemovalWithCoLocalizations);
                            }
                        } else {
                            return AnalysisLocalVerdict::TooShort;
                        }
                    },
                    AnalysisKind::Simulate(_) => {
                        match node_kind.flags.is_simulated() {
                            WasMultiTraceConsumedWithSimulation::No => {
                                return AnalysisLocalVerdict::TooShort;
                            },
                            WasMultiTraceConsumedWithSimulation::OnlyAfterEnd => {
                                return AnalysisLocalVerdict::MultiPref;
                            },
                            WasMultiTraceConsumedWithSimulation::AsSlice => {
                                return AnalysisLocalVerdict::Slice;
                            }
                        }
                    }
                }
            }
        } else { /* multi-trace not emptied */
            match param.ana_kind {
                AnalysisKind::Accept => {
                    return AnalysisLocalVerdict::Out(false);
                },
                AnalysisKind::Prefix => {
                    if node_kind.flags.is_any_component_empty(&context.multi_trace) {
                        return AnalysisLocalVerdict::Inconc(InconcReason::LackObs);
                    } else {
                        return AnalysisLocalVerdict::Out(false);
                    }
                },
                AnalysisKind::Eliminate => {
                    return AnalysisLocalVerdict::Out(false);
                },
                AnalysisKind::Simulate(_) => {
                    return AnalysisLocalVerdict::OutSim(false);
                }
            }
        }
    }

    fn get_local_verdict_from_static_analysis(context: &AnalysisContext,
                                              param : &AnalysisParameterization,
                                              node_kind: &mut AnalysisNodeKind)
            -> Option<(AnalysisLocalVerdict, AnalysisStaticLocalVerdictAnalysisProof)> {

        if let Some(locana_param) = &param.locana {
            match is_dead_local_analysis(&context.gen_ctx,
                                         &context.co_localizations,
                                         &param.ana_kind,
                                         locana_param,
                                         param.partial_order_reduction,
                                         &node_kind.interaction,
                                         &context.multi_trace,
                                         &mut node_kind.flags) {
                None => {
                    None
                },
                Some( fail_on_canal_id ) => {
                    let (local_coloc,local_interaction,local_multi_trace,local_flags) =
                        get_local_analysis_starting_data(&context.gen_ctx,
                                                         fail_on_canal_id,
                                                         &context.co_localizations,
                                                         &node_kind.interaction,
                                                         &context.multi_trace,
                                                         &node_kind.flags);
                    let data = AnalysisStaticLocalVerdictAnalysisProof::new(param.ana_kind.clone(),
                                                                            local_coloc,
                                                                            local_interaction,
                                                                            local_multi_trace,
                                                                            local_flags);
                    if param.ana_kind.has_simulation() {
                        Some( (AnalysisLocalVerdict::OutSim(true),data) )
                    } else {
                        Some( (AnalysisLocalVerdict::Out(true),data) )
                    }
                }
            }
        } else {
            None
        }
    }

    fn pursue_process_after_static_verdict(context: &AnalysisContext,
                                           param : &AnalysisParameterization,
                                           loc_verd: &AnalysisLocalVerdict) -> bool {
        match loc_verd {
            AnalysisLocalVerdict::Out(_) => {
                false
            },
            AnalysisLocalVerdict::OutSim(_) => {
                false
            },
            _ => {true}
        }
    }
}

