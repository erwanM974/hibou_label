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

use std::collections::{HashSet,HashMap};
use std::cmp::Reverse;

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction,WasMultiTraceConsumedWithSimulation};
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::global_frontier;
use crate::core::semantics::execute::{execute_interaction, ExecutionResult};
use crate::process::verdicts::*;
use crate::process::hibou_process::*;
use crate::core::trace::TraceActionKind;
use crate::from_hfiles::hibou_options::HibouOptions;

use crate::process::priorities::ProcessPriorities;


use crate::process::anakind::{AnalysisKind,UseLocalAnalysis};
use crate::process::analysis::analyze;


pub struct HibouProcessManager {
    gen_ctx : GeneralContext,
    strategy : HibouSearchStrategy,
    ana_kind : Option<AnalysisKind>,
    local_analysis : Option<UseLocalAnalysis>,
    pre_filters : Vec<HibouPreFilter>,
    // ***
    memorized_states : HashMap<u32,MemorizedState>,
    process_queue : Vec<NextToProcess>,
    // ***
    frontier_priorities : ProcessPriorities,
    // ***
    loggers : Vec<Box<dyn ProcessLogger>>
}

impl HibouProcessManager {

    pub fn get_ana_kind(&self) -> &AnalysisKind {
        return self.ana_kind.as_ref().unwrap();
    }

    pub fn get_local_analysis(&self) -> &UseLocalAnalysis {
        return self.local_analysis.as_ref().unwrap();
    }

    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               ana_kind : Option<AnalysisKind>,
               local_analysis : Option<UseLocalAnalysis>,
               pre_filters : Vec<HibouPreFilter>,
               memorized_states : HashMap<u32,MemorizedState>,
               frontier_priorities : ProcessPriorities,
               loggers : Vec<Box<dyn ProcessLogger>>
    ) -> HibouProcessManager {
        return HibouProcessManager{gen_ctx,
            strategy,
            ana_kind,
            local_analysis,
            pre_filters,
            memorized_states,
            process_queue:vec![],
            frontier_priorities,
            loggers};
    }

    pub fn get_options_as_strings(&self,goal_and_verdict:Option<(&Option<GlobalVerdict>,&GlobalVerdict)>) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        match goal_and_verdict {
            None => {
                options_str.push("process=exploration".to_string());
                options_str.push( format!("strategy={}", &self.strategy.to_string()) );
                options_str.push( format!("priorities=[{}]", &self.frontier_priorities.print_as_string(false)) );
            },
            Some( (goal,verd) ) => {
                options_str.push("process=analysis".to_string());
                options_str.push( format!("analysis kind={}", self.ana_kind.as_ref().unwrap().to_string()) );
                options_str.push( format!("strategy={}", &self.strategy.to_string()) );
                options_str.push( format!("priorities=[{}]", &self.frontier_priorities.print_as_string(true)) );
                // ***
                match self.local_analysis {
                    None => {},
                    Some( ref loc_ana ) => {
                        match loc_ana {
                            &UseLocalAnalysis::No => {
                                options_str.push("local_analysis=false".to_string());
                            },
                            &UseLocalAnalysis::Yes => {
                                options_str.push("local_analysis=true".to_string());
                            },
                            _ => {
                                panic!("TODO: implement")
                            }
                        }
                    }
                }
                match goal {
                    None => {
                        options_str.push( "goal=None".to_string() );
                    },
                    Some( target_goal ) => {
                        options_str.push( format!("goal={}", target_goal.to_string()) );
                    }
                }
                options_str.push( format!("verdict={}", verd.to_string()) );
            }
        }
        {
            let mut rem_filter = self.pre_filters.len();
            let mut filters_str = "filters=[".to_string();
            for filter in &self.pre_filters {
                filters_str.push_str( &filter.to_string() );
                rem_filter = rem_filter - 1;
                if rem_filter > 0 {
                    filters_str.push_str( "," );
                }
            }
            filters_str.push_str( "]" );
            options_str.push( filters_str );
        }
        return options_str;
    }

    fn perform_local_analysis(&self, interaction : &Interaction, canal : &MultiTraceCanal) -> GlobalVerdict {
        if canal.flag_dirty4local && canal.trace.len() > 0 {
            // ***
            match self.ana_kind.as_ref().unwrap() {
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
                let mut lfs_to_remove = self.gen_ctx.get_all_lfs_ids();
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
            let (local_verdict,_) = analyze(local_interaction,local_mu,self.gen_ctx.clone(),HibouOptions::local_analyze());
            return local_verdict;
        } else {
            return GlobalVerdict::Pass;
        }
    }

    pub fn is_dead_local_analysis(&self, interaction : &Interaction, multi_trace : &mut AnalysableMultiTrace) -> bool {
        match &self.local_analysis {
            None => { return false;},
            Some( use_locana ) => {
                match use_locana {
                    UseLocalAnalysis::No => {return false;},
                    UseLocalAnalysis::Yes => {
                        for canal in multi_trace.canals.iter_mut() {
                            match self.perform_local_analysis(interaction,canal) {
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
        }
    }

    pub fn init_loggers(&mut self, interaction : &Interaction,remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_init(interaction, &self.gen_ctx, remaining_multi_trace);
        }
    }

    pub fn term_loggers(&mut self,goal_and_verdict:Option<(&Option<GlobalVerdict>,&GlobalVerdict)>) {
        let options_as_strs = (&self).get_options_as_strings(goal_and_verdict);
        for logger in self.loggers.iter_mut() {
            (*logger).log_term(&options_as_strs);
        }
    }

    pub fn verdict_loggers(&mut self,
                           verdict : &CoverageVerdict,
                           parent_state_id : u32) {
        for logger in self.loggers.iter_mut() {
            logger.log_verdict(parent_state_id,
                               verdict);
        }
    }

    pub fn filtered_loggers(&mut self,
                            parent_state_id : u32,
                            new_state_id : u32,
                            elim_kind : &FilterEliminationKind) {
        for logger in self.loggers.iter_mut() {
            logger.log_filtered(&self.gen_ctx,
                                parent_state_id,
                                new_state_id,
                                elim_kind);
        }
    }

    pub fn execution_loggers(&mut self,
                             action_position : &Position,
                             executed_actions : &HashSet<TraceAction>,
                             sim_map : &HashMap<usize,SimulationStepKind>,
                             new_interaction : &Interaction,
                             parent_state_id : u32,
                             new_state_id :u32,
                             remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            logger.log_execution(&self.gen_ctx,
                                 parent_state_id,
                                 new_state_id,
                                 action_position,
                                 executed_actions,
                                 sim_map,
                                 new_interaction,
                                 remaining_multi_trace);
        }
    }

    pub fn hiding_loggers(&mut self,
                             lfs_to_hide : &HashSet<usize>,
                             new_interaction : &Interaction,
                             parent_state_id : u32,
                             new_state_id :u32,
                             remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            logger.log_hide(&self.gen_ctx,
                                 parent_state_id,
                                 new_state_id,
                            lfs_to_hide,
                            new_interaction,
                                 remaining_multi_trace);
        }
    }

    pub fn get_memorized_state(&self, id:u32) -> Option<&MemorizedState> {
        return self.memorized_states.get(&id);
    }

    pub fn forget_state(&mut self, id:u32) {
        self.memorized_states.remove(&id);
    }

    pub fn remember_state(&mut self, id:u32, state:MemorizedState) {
        self.memorized_states.insert( id, state );
    }

    pub fn extract_from_queue(&mut self) -> Option<NextToProcess> {
        if self.process_queue.len() > 0 {
            return Some( self.process_queue.remove(0) );
        } else {
            return None;
        }
    }

    fn get_priority_of_node(&self,
                            state_id : u32,
                            child_kind : &NextToProcessKind,
                            priorities : &ProcessPriorities,
                            node_depth : u32) -> i32 {
        let mut priority : i32 = 0;
        // ***
        let parent_state = self.get_memorized_state(state_id).unwrap();
        match &child_kind {
            &NextToProcessKind::Execute( ref front_elt ) => {
                match (parent_state.interaction).get_sub_interaction(&front_elt.position).get_leaf_action_kind() {
                    TraceActionKind::Reception => {
                        priority = priority + priorities.reception;
                    },
                    TraceActionKind::Emission => {
                        priority = priority + priorities.emission;
                    }
                }
                // ***
                let loop_depth = (parent_state.interaction).get_loop_depth_at_pos(&front_elt.position);
                if loop_depth > 0 {
                    priority = priority + priorities.in_loop;
                }
            },
            &NextToProcessKind::Hide(lf_to_hide) => {
                priority = priority + priorities.hide;
            },
            &NextToProcessKind::Simulate( ref front_elt, ref sim_map ) => {
                match (parent_state.interaction).get_sub_interaction(&front_elt.position).get_leaf_action_kind() {
                    TraceActionKind::Reception => {
                        priority = priority + priorities.reception;
                    },
                    TraceActionKind::Emission => {
                        priority = priority + priorities.emission;
                    }
                }
                // ***
                let loop_depth = (parent_state.interaction).get_loop_depth_at_pos(&front_elt.position);
                if loop_depth > 0 {
                    priority = priority + priorities.in_loop;
                }
                // ***
                if sim_map.len() > 0 {
                    priority = priority + priorities.simulate
                }
                //priority = priority + priorities.simulate*(sim_map.len() as i32);
            }
        }
        // ***
        match priorities.step {
            None => {},
            Some( step ) => {
                priority = priority + ( (node_depth as i32) * step);
            }
        }
        return priority;
    }

    pub fn enqueue_executions(&mut self,
                              state_id : u32,
                              to_enqueue : Vec<(u32,NextToProcessKind)>,
                              node_depth : u32) {
        let mut to_enqueue_reorganized : Vec<NextToProcess> = Vec::new();
        {
            let mut to_enqueue_reorganize_by_priorities : HashMap<i32,Vec<NextToProcess>> = HashMap::new();
            for (child_id,child_kind) in to_enqueue {
                let priority : i32 = self.get_priority_of_node(state_id,&child_kind,&self.frontier_priorities,node_depth);
                let child = NextToProcess::new(state_id,child_id,child_kind);
                // ***
                match to_enqueue_reorganize_by_priorities.get_mut(&priority) {
                    None => {
                        to_enqueue_reorganize_by_priorities.insert(priority,vec![ child ]);
                    },
                    Some( queue ) => {
                        queue.push(child );
                    }
                }
            }
            // ***
            {
                let mut keys : Vec<i32> = to_enqueue_reorganize_by_priorities.keys().cloned().collect();
                keys.sort_by_key(|k| Reverse(*k));
                for k in keys {
                    match to_enqueue_reorganize_by_priorities.get_mut(&k) {
                        None => {},
                        Some( queue ) => {
                            for child in queue.drain(..) {
                                to_enqueue_reorganized.push( child );
                            }
                        }
                    }
                }
            }
        }
        // ***
        match &self.strategy {
            &HibouSearchStrategy::DFS => {
                to_enqueue_reorganized.append(&mut self.process_queue);
                self.process_queue = to_enqueue_reorganized;
            },
            &HibouSearchStrategy::BFS => {
                self.process_queue.append( &mut to_enqueue_reorganized );
            }
        }
    }

    pub fn process_next(&mut self,
                        parent_state : &MemorizedState,
                        to_process   : &NextToProcess,
                        new_state_id : u32,
                        node_counter : u32) -> Option<(Interaction,Option<AnalysableMultiTrace>,u32,u32)> {
        match &(to_process.kind) {
            &NextToProcessKind::Execute( ref frt_elt ) => {
                let new_depth = parent_state.depth + 1;
                let new_loop_depth = parent_state.loop_depth + (parent_state.interaction).get_loop_depth_at_pos(&frt_elt.position);
                // ***
                match self.apply_pre_filters(new_depth,Some(new_loop_depth),node_counter) {
                    None => {
                        // ***
                        let exe_result : ExecutionResult;
                        let new_multi_trace : Option<AnalysableMultiTrace>;
                        match (parent_state.multi_trace).as_ref(){
                            None => {
                                exe_result = execute_interaction(&parent_state.interaction,
                                                                 &frt_elt.position,
                                                                 &frt_elt.target_lf_ids,
                                                                 false);
                                new_multi_trace = None;
                            },
                            Some( ref multi_trace ) => {
                                exe_result = execute_interaction(&parent_state.interaction,
                                                                 &frt_elt.position,
                                                                 &frt_elt.target_lf_ids,
                                                                 true);
                                new_multi_trace = Some(multi_trace.update_on_execution(&frt_elt.target_lf_ids,
                                                                                       &exe_result.affected_lifelines,
                                                                                       &exe_result.interaction));
                            }
                        }
                        // ***
                        self.execution_loggers(&frt_elt.position,
                                          &frt_elt.target_actions,
                                               &HashMap::new(),
                                          &exe_result.interaction,
                                          to_process.state_id,
                                               new_state_id,
                                          &new_multi_trace);
                        // ***
                        return Some( (exe_result.interaction,new_multi_trace,new_depth,new_loop_depth) );
                    },
                    Some( elim_kind ) => {
                        self.filtered_loggers(to_process.state_id,
                                              new_state_id,
                                              &elim_kind);
                        return None;
                    }
                }
            },
            &NextToProcessKind::Hide( ref lfs_to_hide ) => {
                let new_depth = parent_state.depth + 1;
                // ***
                match self.apply_pre_filters(new_depth,None,node_counter) {
                    None => {
                        let new_interaction = (parent_state.interaction).hide(lfs_to_hide);
                        // ***
                        let new_multi_trace : Option<AnalysableMultiTrace>;
                        match (parent_state.multi_trace).as_ref(){
                            None => {
                                panic!();
                            },
                            Some( ref multi_trace ) => {
                                new_multi_trace = Some(multi_trace.update_on_hide(&lfs_to_hide));
                            }
                        }
                        // ***
                        self.hiding_loggers(lfs_to_hide,
                                               &new_interaction,
                                               to_process.state_id,
                                               new_state_id,
                                               &new_multi_trace);
                        // ***
                        return Some( (new_interaction,new_multi_trace,new_depth,parent_state.loop_depth) );
                    },
                    Some( elim_kind ) => {
                        self.filtered_loggers(to_process.state_id,
                                              new_state_id,
                                              &elim_kind);
                        return None;
                    }
                }
            },
            &NextToProcessKind::Simulate( ref frt_elt, ref sim_map ) => {
                let new_depth = parent_state.depth + 1;
                let target_loop_depth = (parent_state.interaction).get_loop_depth_at_pos(&frt_elt.position);
                let new_loop_depth = parent_state.loop_depth + target_loop_depth;
                // ***
                match self.apply_pre_filters(new_depth,Some(new_loop_depth), node_counter) {
                    None => {
                        // ***
                        let exe_result : ExecutionResult;
                        let new_multi_trace : Option<AnalysableMultiTrace>;
                        match (parent_state.multi_trace).as_ref(){
                            None => {
                                exe_result = execute_interaction(&parent_state.interaction,
                                                                 &frt_elt.position,
                                                                 &frt_elt.target_lf_ids,
                                                                 false);
                                new_multi_trace = None;
                            },
                            Some( ref multi_trace ) => {
                                exe_result = execute_interaction(&parent_state.interaction,
                                                                 &frt_elt.position,
                                                                 &frt_elt.target_lf_ids,
                                                                 true);
                                let rem_sim_depth : u32;
                                if sim_map.len() > 0 {
                                    rem_sim_depth = multi_trace.remaining_loop_instantiations_in_simulation - target_loop_depth;
                                } else {
                                    rem_sim_depth = exe_result.interaction.max_nested_loop_depth();
                                }
                                new_multi_trace = Some(multi_trace.update_on_simulation(sim_map,
                                                                                        &frt_elt.target_lf_ids,
                                                                                        &exe_result.affected_lifelines,
                                                                                        rem_sim_depth));
                            }
                        }
                        // ***
                        self.execution_loggers(&frt_elt.position,
                                               &frt_elt.target_actions,
                                               sim_map,
                                               &exe_result.interaction,
                                               to_process.state_id,
                                               new_state_id,
                                               &new_multi_trace);
                        // ***
                        return Some( (exe_result.interaction,new_multi_trace,new_depth,new_loop_depth) );
                    },
                    Some( elim_kind ) => {
                        self.filtered_loggers(to_process.state_id,
                                              new_state_id,
                                              &elim_kind);
                        return None;
                    }
                }
            }
        }
    }

    fn apply_pre_filters(&self, depth : u32, loop_depth : Option<u32>, node_counter : u32) -> Option<FilterEliminationKind> {
        for pre_filter in &self.pre_filters {
            match pre_filter {
                HibouPreFilter::MaxProcessDepth( max_depth ) => {
                    if depth > *max_depth {
                        return Some( FilterEliminationKind::MaxProcessDepth );
                    }
                },
                HibouPreFilter::MaxLoopInstanciation( loop_num ) => {
                    match loop_depth {
                        None => {},
                        Some( ld ) => {
                            if ld > *loop_num {
                                return Some( FilterEliminationKind::MaxLoopInstanciation );
                            }
                        }
                    }
                },
                HibouPreFilter::MaxNodeNumber( max_node_number ) => {
                    if node_counter >= *max_node_number {
                        return Some( FilterEliminationKind::MaxNodeNumber );
                    }
                }
            }
        }
        return None;
    }

    pub fn get_coverage_verdict(&self,
                                interaction:&Interaction,
                                multi_trace:&AnalysableMultiTrace) -> CoverageVerdict {
        if multi_trace.length() == 0 {
            if interaction.express_empty() {
                match self.ana_kind.as_ref().unwrap() {
                    AnalysisKind::Accept => {
                        return CoverageVerdict::Cov;
                    },
                    AnalysisKind::Prefix => {
                        return CoverageVerdict::Cov;
                    },
                    AnalysisKind::Hide => {
                        if multi_trace.is_any_component_hidden() {
                            if multi_trace.are_colocalizations_singletons() {
                                return CoverageVerdict::MultiPref;
                            } else {
                                return CoverageVerdict::Inconc;
                            }
                        } else {
                            return CoverageVerdict::Cov;
                        }
                    },
                    AnalysisKind::Simulate(_) => {
                        match multi_trace.is_simulated() {
                            WasMultiTraceConsumedWithSimulation::No => {
                                return CoverageVerdict::Cov;
                            },
                            WasMultiTraceConsumedWithSimulation::OnlyAfterEnd => {
                                return CoverageVerdict::MultiPref;
                            },
                            WasMultiTraceConsumedWithSimulation::AsSlice => {
                                return CoverageVerdict::Slice;
                            }
                        }
                    }
                }
            } else {
                match self.ana_kind.as_ref().unwrap() {
                    AnalysisKind::Accept => {
                        return CoverageVerdict::UnCov;
                    },
                    AnalysisKind::Prefix => {
                        return CoverageVerdict::TooShort;
                    },
                    AnalysisKind::Hide => {
                        if multi_trace.is_any_component_hidden() {
                            if multi_trace.are_colocalizations_singletons() {
                                return CoverageVerdict::MultiPref;
                            } else {
                                return CoverageVerdict::Inconc;
                            }
                        } else {
                            return CoverageVerdict::TooShort;
                        }
                    },
                    AnalysisKind::Simulate(_) => {
                        match multi_trace.is_simulated() {
                            WasMultiTraceConsumedWithSimulation::No => {
                                return CoverageVerdict::TooShort;
                            },
                            WasMultiTraceConsumedWithSimulation::OnlyAfterEnd => {
                                return CoverageVerdict::MultiPref;
                            },
                            WasMultiTraceConsumedWithSimulation::AsSlice => {
                                return CoverageVerdict::Slice;
                            }
                        }
                    }
                }
            }
        } else {
            match self.ana_kind.as_ref().unwrap() {
                AnalysisKind::Accept => {
                    return CoverageVerdict::UnCov;
                },
                AnalysisKind::Prefix => {
                    if multi_trace.is_any_component_empty() {
                        return CoverageVerdict::LackObs;
                    } else {
                        return CoverageVerdict::Out;
                    }
                },
                AnalysisKind::Hide => {
                    return CoverageVerdict::Out;
                },
                AnalysisKind::Simulate(_) => {
                    return CoverageVerdict::Out;
                },
            }
        }
    }

}