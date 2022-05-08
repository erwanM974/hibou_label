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
use crate::core::trace::{AnalysableMultiTrace, MultiTraceCanal};
use crate::from_hfiles::hibou_options::HibouOptions;
use crate::proc_refactoring::ana_proc::AnalysisProcessManager;
use crate::proc_refactoring::ana_proc::priorities::AnalysisPriorities;
use crate::process::anakind::{AnalysisKind, UseLocalAnalysis};
use crate::process::hibou_process::HibouSearchStrategy;
use crate::process::verdicts::GlobalVerdict;

pub fn is_dead_local_analysis(gen_ctx : &GeneralContext, parent_analysis_kind : &AnalysisKind,use_locana : &UseLocalAnalysis, interaction : &Interaction, multi_trace : &mut AnalysableMultiTrace) -> bool {
    match use_locana {
        UseLocalAnalysis::No => {return false;},
        UseLocalAnalysis::Yes => {
            for canal in multi_trace.canals.iter_mut() {
                match perform_local_analysis(gen_ctx,parent_analysis_kind,interaction,canal) {
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


fn perform_local_analysis(gen_ctx : &GeneralContext, parent_analysis_kind : &AnalysisKind, interaction : &Interaction, canal : &MultiTraceCanal) -> GlobalVerdict {
    if canal.flag_dirty4local && canal.trace.len() > 0 {
        // ***
        match parent_analysis_kind {
            AnalysisKind::Simulate( sim_before ) => {
                if *sim_before && (canal.consumed == 0) {
                    // here we allow the simulation of actions before the start of
                    // the given component trace
                    // hence we shouldn't discard the node
                    return GlobalVerdict::Pass;
                }
            },
            _ => {}
        }
        // ***
        let local_interaction : Interaction;
        {
            let mut lfs_to_remove = gen_ctx.get_all_lfs_ids();
            for lf_id in &canal.lifelines {
                lfs_to_remove.remove( &lf_id );
            }
            local_interaction = interaction.hide(&lfs_to_remove);
        }
        // ***
        let local_mu : AnalysableMultiTrace;
        {
            let mut canals = Vec::new();
            canals.push( MultiTraceCanal::new(canal.lifelines.clone(),
                                              canal.trace.clone(),
                                              false,
                                              false,
                                              0,
                                              0,
                                              0) );
            local_mu = AnalysableMultiTrace::new(canals,0);
        }
        // ***
        let mut local_analysis_manager = AnalysisProcessManager::new(gen_ctx.clone(),
                                                                 vec![],
                                                                 HibouSearchStrategy::DFS,
                                                                 AnalysisPriorities::new(0,0,0,0,0),
                                                                 AnalysisKind::Prefix,UseLocalAnalysis::No,Some(GlobalVerdict::WeakPass),
                                                                 vec![]);
        let (local_verdict,_) = local_analysis_manager.analyze(local_interaction,local_mu);
        return local_verdict;
    } else {
        return GlobalVerdict::Pass;
    }
}