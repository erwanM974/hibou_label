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
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{multi_trace_length, MultiTrace, Trace};
use crate::core::general_context::GeneralContext;
use crate::core::language::eliminate_lf::eliminable::LifelineEliminable;
use crate::core::language::syntax::interaction::Interaction;
use crate::process::ana::conf::AnalysisConfig;
use crate::process::ana::filter::filter::AnalysisFilter;
use crate::process::ana::node::flags::{MultiTraceAnalysisFlags, TraceAnalysisFlags};
use crate::process::ana::node::node::AnalysisNodeKind;
use crate::process::ana::param::anakind::AnalysisKind;
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::priorities::AnalysisPriorities;
use crate::process::ana::step::AnalysisStepKind;
use crate::process::ana::verdict::global::AnalysisGlobalVerdict;
use crate::process::ana::context::AnalysisContext;

pub fn get_local_analysis_starting_data(gen_ctx : &GeneralContext,
                                        canal_id : usize,
                                        co_localizations : &CoLocalizations,
                                        interaction : &Interaction,
                                        multi_trace : &MultiTrace,
                                        flags : &MultiTraceAnalysisFlags) -> (CoLocalizations,Interaction,MultiTrace,MultiTraceAnalysisFlags) {
    let local_coloc : CoLocalizations;
    let locs_lf_ids : &HashSet<usize> = co_localizations.locs_lf_ids.get(canal_id).unwrap();
    local_coloc = CoLocalizations::new(vec![locs_lf_ids.clone()]);
    // ***
    let local_interaction : Interaction;
    {
        let mut lfs_to_remove = gen_ctx.get_all_lfs_ids();
        for lf_id in locs_lf_ids {
            lfs_to_remove.remove( lf_id );
        }
        local_interaction = interaction.eliminate_lifelines(&lfs_to_remove);
    }
    // ***
    let canal_trace: &Trace = multi_trace.get(canal_id).unwrap();
    let local_multi_trace : MultiTrace = vec![canal_trace.clone()];
    // ***
    let canal_flags: &TraceAnalysisFlags = flags.canals.get(canal_id).unwrap();
    let local_flags : MultiTraceAnalysisFlags = MultiTraceAnalysisFlags::new(vec![canal_flags.clone()], flags.rem_loop_in_sim, flags.rem_act_in_sim);
    // ***
    return (local_coloc,local_interaction,local_multi_trace,local_flags);
}


pub fn is_dead_local_analysis(gen_ctx : &GeneralContext,
                              co_localizations : &CoLocalizations,
                              parent_analysis_kind : &AnalysisKind,
                              use_locana : bool,
                              interaction : &Interaction,
                              multi_trace : &MultiTrace,
                              flags : &mut MultiTraceAnalysisFlags) -> Option<usize> {
    if use_locana {
        for (canal_id, colocalized_lfs) in co_localizations.locs_lf_ids.iter().enumerate() {
            let canal_flags: &mut TraceAnalysisFlags = flags.canals.get_mut(canal_id).unwrap();
            let canal_trace: &Trace = multi_trace.get(canal_id).unwrap();
            // ***
            if canal_flags.dirty4local && canal_trace.len() > canal_flags.consumed {
                let local_flags : MultiTraceAnalysisFlags = MultiTraceAnalysisFlags::new(vec![canal_flags.clone()], flags.rem_loop_in_sim, flags.rem_act_in_sim);
                let local_multi_trace : MultiTrace = vec![canal_trace.clone()];
                let local_interaction : Interaction;
                {
                    let mut lfs_to_remove = gen_ctx.get_all_lfs_ids();
                    for lf_id in colocalized_lfs {
                        lfs_to_remove.remove( &lf_id );
                    }
                    local_interaction = interaction.eliminate_lifelines(&lfs_to_remove);
                }
                let local_coloc = CoLocalizations::new(vec![colocalized_lfs.clone()]);
                match perform_local_analysis(gen_ctx,local_coloc,parent_analysis_kind,local_interaction,local_multi_trace,local_flags,vec![]) {
                    AnalysisGlobalVerdict::Fail => {
                        return Some(canal_id);
                    },
                    AnalysisGlobalVerdict::WeakFail => {
                        return Some(canal_id);
                    },
                    _ => {}
                }
            }
            // ***
            canal_flags.dirty4local = false;
        }
    }
    return None;
}




pub fn perform_local_analysis(gen_ctx : &GeneralContext,
                          local_coloc : CoLocalizations,
                          parent_analysis_kind : &AnalysisKind,
                          local_interaction : Interaction,
                          local_multi_trace : MultiTrace,
                          local_flags : MultiTraceAnalysisFlags,
                              loggers : Vec<Box< dyn AbstractProcessLogger<AnalysisConfig>>>) -> AnalysisGlobalVerdict {
    // ***
    let local_analysis_kind : AnalysisKind;
    match parent_analysis_kind {
        AnalysisKind::Simulate( sim_config ) => {
            if sim_config.sim_before {
                local_analysis_kind = AnalysisKind::Simulate(sim_config.clone());
            } else {
                local_analysis_kind = AnalysisKind::Prefix;
            }
        },
        _ => {
            local_analysis_kind = AnalysisKind::Prefix;
        }
    }
    // ***
    let new_gen_ctx= gen_ctx.clone();
    // ***
    let mut locana_filters : Vec<AnalysisFilter> = vec![];
    // ***
    let init_mu_len = multi_trace_length(&local_multi_trace);
    let locana_ctx = AnalysisContext::new(new_gen_ctx,local_coloc,local_multi_trace,init_mu_len);
    let priorities : GenericProcessPriorities<AnalysisPriorities> = GenericProcessPriorities::new(AnalysisPriorities::default(),false);
    let delegate : GenericProcessDelegate<AnalysisStepKind,AnalysisNodeKind,AnalysisPriorities> = GenericProcessDelegate::new(QueueSearchStrategy::HCS,priorities);
    let locana_param = AnalysisParameterization::new(local_analysis_kind, false);
    let mut local_analysis_manager : GenericProcessManager<AnalysisConfig> = GenericProcessManager::new(locana_ctx,
                                                                                                        locana_param,
                                                                                                        delegate,
                                                                                                        vec![],
                                                                                                        loggers,
                                                                                                        Some(AnalysisGlobalVerdict::WeakPass),
                                                                                                        true);
    let init_node = AnalysisNodeKind::new(local_interaction,local_flags,0);
    let (_,verd) = local_analysis_manager.start_process(init_node);
    return verd;
}