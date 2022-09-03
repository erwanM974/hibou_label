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


use crate::core::general_context::GeneralContext;
use crate::core::syntax::interaction::Interaction;
use crate::process::abstract_proc::common::HibouSearchStrategy;
use crate::process::ana_proc::anakind::{AnalysisKind, UseLocalAnalysis};
use crate::process::ana_proc::manager::AnalysisProcessManager;
use crate::process::ana_proc::interface::priorities::AnalysisPriorities;
use crate::process::ana_proc::multitrace::{AnalysableMultiTraceCanal,AnalysableMultiTrace};
use crate::process::ana_proc::verdicts::GlobalVerdict;

pub fn is_dead_local_analysis(gen_ctx : &GeneralContext,
                              parent_analysis_kind : &AnalysisKind,
                              use_locana : &UseLocalAnalysis,
                              interaction : &Interaction,
                              multi_trace : &mut AnalysableMultiTrace) -> bool {
    match use_locana {
        UseLocalAnalysis::No => {return false;},
        UseLocalAnalysis::Yes => {
            for (canal_id,canal) in multi_trace.canals.iter_mut().enumerate() {
                match perform_local_analysis(gen_ctx,parent_analysis_kind,interaction,canal_id,canal) {
                    GlobalVerdict::Fail => {
                        return true;
                    },
                    _ => {
                        canal.flag_dirty4local = false;
                    }
                }
            }
            return false;
        },
        UseLocalAnalysis::OnlyFront => {
            panic!("TODO implement");
        }
    }
}


fn perform_local_analysis(gen_ctx : &GeneralContext,
                          parent_analysis_kind : &AnalysisKind,
                          interaction : &Interaction,
                          canal_id : usize,
                          canal : &AnalysableMultiTraceCanal) -> GlobalVerdict {
    if canal.flag_dirty4local && canal.trace.len() > 0 {
        // ***
        let local_interaction : Interaction;
        {
            let mut lfs_to_remove = gen_ctx.get_all_lfs_ids();
            for lf_id in gen_ctx.co_localizations.get(canal_id).unwrap() {
                lfs_to_remove.remove( &lf_id );
            }
            local_interaction = interaction.hide(&lfs_to_remove);
        }
        // ***
        let local_mu : AnalysableMultiTrace;
        {
            let mut canals = Vec::new();
            canals.push( AnalysableMultiTraceCanal::new(
                                              canal.trace.clone(),
                                              false,
                                              false,
                                              0,
                                              0,
                                              0) );
            local_mu = AnalysableMultiTrace::new(canals,0);
        }
        // ***
        let local_analysis_kind : AnalysisKind;
        match parent_analysis_kind {
            AnalysisKind::Simulate( sim_before ) => {
                if *sim_before {
                    local_analysis_kind = AnalysisKind::Simulate(true);
                } else {
                    local_analysis_kind = AnalysisKind::Prefix;
                }
            },
            _ => {
                local_analysis_kind = AnalysisKind::Prefix;
            }
        }
        // ***
        let mut new_gen_ctx= gen_ctx.clone();
        new_gen_ctx.co_localizations = vec![ gen_ctx.co_localizations.get(canal_id).unwrap().clone() ];
        // ***
        let mut local_analysis_manager = AnalysisProcessManager::new(new_gen_ctx,
                                                                 HibouSearchStrategy::DFS,
                                                                     vec![],
                                                                 AnalysisPriorities::default(),
                                                                     vec![],
                                                                     local_analysis_kind,
                                                                     UseLocalAnalysis::No,
                                                                     Some(GlobalVerdict::WeakPass)
                                                                 );
        let (local_verdict,_) = local_analysis_manager.analyze(local_interaction,local_mu);
        return local_verdict;
    } else {
        return GlobalVerdict::Pass;
    }
}