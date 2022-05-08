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

pub(in crate::proc_refactoring::ana_proc) mod step;
pub(in crate::proc_refactoring::ana_proc) mod conf;
pub(in crate::proc_refactoring::ana_proc) mod priorities;
pub(in crate::proc_refactoring::ana_proc) mod node;
pub(in crate::proc_refactoring::ana_proc) mod matches;
pub(in crate::proc_refactoring::ana_proc) mod local_analysis;
pub(in crate::proc_refactoring::ana_proc) mod filter;


use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::core::general_context::GeneralContext;


use crate::core::semantics::execute::execute_interaction;
use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::position::Position;
use crate::core::trace::{AnalysableMultiTrace, TraceAction, WasMultiTraceConsumedWithSimulation};
use crate::proc_refactoring::abstract_proc::{GenericNode, GenericProcessManager, GenericStep};
use crate::proc_refactoring::ana_proc::conf::AnalysisConfig;
use crate::proc_refactoring::ana_proc::filter::{AnalysisFilter, AnalysisFilterCriterion};
use crate::proc_refactoring::ana_proc::local_analysis::is_dead_local_analysis;
use crate::proc_refactoring::ana_proc::matches::{add_action_matches_in_analysis,add_simulation_matches_in_analysis};
use crate::proc_refactoring::ana_proc::step::AnalysisStepKind;
use crate::proc_refactoring::ana_proc::node::AnalysisNodeKind;
use crate::proc_refactoring::ana_proc::priorities::AnalysisPriorities;
use crate::process::anakind::{AnalysisKind, UseLocalAnalysis};
use crate::process::hibou_process::{FilterEliminationKind, HibouPreFilter, HibouSearchStrategy, SimulationStepKind};
use crate::process::log::ProcessLogger;
use crate::process::priorities::ProcessPriorities;
use crate::process::verdicts::{CoverageVerdict, GlobalVerdict, update_global_verdict_from_new_coverage_verdict};


pub fn ana_priorities_translate(proc_prio : ProcessPriorities) -> AnalysisPriorities {
    return AnalysisPriorities::new(proc_prio.emission,proc_prio.reception,0,proc_prio.in_loop, proc_prio.simulate);
}

pub fn ana_filter_translate(proc_filter : Vec<HibouPreFilter>) -> Vec<AnalysisFilter> {
    let mut exp_filters = vec![];
    for filter in proc_filter {
        match filter {
            HibouPreFilter::MaxProcessDepth(crit) => {
                exp_filters.push( AnalysisFilter::MaxProcessDepth(crit));
            },
            HibouPreFilter::MaxLoopInstanciation(crit) => {
                exp_filters.push( AnalysisFilter::MaxLoopInstanciation(crit));
            },
            HibouPreFilter::MaxNodeNumber(crit) => {
                exp_filters.push( AnalysisFilter::MaxNodeNumber(crit));
            }
        }
    }
    return exp_filters;
}

pub struct AnalysisProcessManager {
    manager: GenericProcessManager<AnalysisConfig>,
    loggers: Vec<Box<dyn ProcessLogger>>,
    ana_kind : AnalysisKind,
    use_locana : UseLocalAnalysis,
    goal : Option<GlobalVerdict>
}

impl AnalysisProcessManager {
    pub fn new(gen_ctx : GeneralContext,
               filters : Vec<AnalysisFilter>,
               strategy : HibouSearchStrategy,
               priorities : AnalysisPriorities,
               ana_kind : AnalysisKind,use_locana : UseLocalAnalysis,
               goal : Option<GlobalVerdict>,
               loggers : Vec<Box<dyn ProcessLogger>>) -> AnalysisProcessManager {
        let manager = GenericProcessManager::new(
            gen_ctx,
            strategy,
            filters,
            priorities,
            HashMap::new(),
            vec![]
        );
        return AnalysisProcessManager{manager,loggers,ana_kind,use_locana,goal};
    }

    pub fn analyze(&mut self,
                   interaction : Interaction,
                   multi_trace : AnalysableMultiTrace) -> (GlobalVerdict,u32) {
        self.init_loggers(&interaction,&multi_trace);
        let mut multi_trace = multi_trace;
        multi_trace.remaining_loop_instantiations_in_simulation = interaction.max_nested_loop_depth();
        // ***
        let mut next_state_id : u32 = 1;
        let mut node_counter : u32 = 0;
        let mut global_verdict = GlobalVerdict::Fail;
        // ***
        // ***
        let pursue_analysis : bool;
        match self.enqueue_next_node_in_analysis(next_state_id,
                                            interaction,multi_trace,
                                            0,0) {
            None => {
                pursue_analysis = true;
            },
            Some( coverage_verdict ) => {
                global_verdict = update_global_verdict_from_new_coverage_verdict(global_verdict, coverage_verdict);
                match self.goal.as_ref() {
                    None => {
                        pursue_analysis = true;
                    },
                    Some( target_goal ) => {
                        if &global_verdict < target_goal {
                            pursue_analysis = true;
                        } else {
                            pursue_analysis = false;
                        }
                    }
                }
            }
        }
        next_state_id = next_state_id +1;
        node_counter = node_counter +1;
        // ***

        if pursue_analysis {
            while let Some(next_to_process) = self.manager.extract_from_queue() {
                let new_state_id = next_state_id;
                next_state_id = next_state_id + 1;
                // ***
                let mut parent_state = self.manager.pick_memorized_state(next_to_process.parent_id);
                // ***
                match self.process_step(&parent_state,
                                           &next_to_process,
                                           new_state_id,
                                           node_counter) {
                    None => {},
                    Some( (new_interaction,new_multi_trace,new_depth,new_loop_depth) ) => {
                        node_counter = node_counter + 1;
                        match self.enqueue_next_node_in_analysis(new_state_id,
                                                            new_interaction,
                                                            new_multi_trace,
                                                            new_depth,
                                                            new_loop_depth) {
                            None => {},
                            Some( coverage_verdict ) => {
                                global_verdict = update_global_verdict_from_new_coverage_verdict(global_verdict, coverage_verdict);
                                match self.goal.as_ref() {
                                    None => {},
                                    Some( target_goal ) => {
                                        if &global_verdict >= target_goal {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                // ***
                parent_state.remaining_ids_to_process.remove(&next_to_process.id_as_child);
                if parent_state.remaining_ids_to_process.len() > 0 {
                    self.manager.remember_state(next_to_process.parent_id,parent_state);
                }
                // ***
            }
        }
        // ***
        self.term_loggers(&global_verdict);
        // ***
        return (global_verdict,node_counter);
    }

    fn enqueue_next_node_in_analysis(&mut self,
                                        parent_id    : u32,
                                        interaction : Interaction,
                                        mut multi_trace : AnalysableMultiTrace,
                                        depth       : u32,
                                        loop_depth  : u32) -> Option<CoverageVerdict> {
        // ***
        let mut id_as_child : u32 = 0;
        let mut to_enqueue : Vec<GenericStep<AnalysisConfig>> = Vec::new();
        // ***
        if multi_trace.length() > 0 {
            // ***
            if is_dead_local_analysis(&self.manager.gen_ctx, &self.ana_kind,&self.use_locana,&interaction,&mut multi_trace) {
                let verdict = CoverageVerdict::Dead;
                self.verdict_loggers(&verdict,parent_id);
                return Some( verdict );
            }
            // ***
            let head_actions = multi_trace.head_actions();
            // ***
            // ***
            match &self.ana_kind {
                &AnalysisKind::Accept => {
                    add_action_matches_in_analysis(parent_id,&interaction,&head_actions,&mut id_as_child, &mut to_enqueue);
                },
                &AnalysisKind::Prefix => {
                    add_action_matches_in_analysis(parent_id,&interaction,&head_actions,&mut id_as_child, &mut to_enqueue);
                },
                &AnalysisKind::Hide => {
                    let mut to_hide : HashSet<usize> = HashSet::new();
                    for canal in &multi_trace.canals {
                        if (canal.flag_hidden == false) && (canal.trace.len() == 0) { // && (interaction.involves_any_of(&canal.lifelines)) {
                            to_hide.extend( canal.lifelines.clone() )
                        }
                    }
                    //
                    if to_hide.len() > 0 {
                        id_as_child = id_as_child + 1;
                        let generic_step = GenericStep{parent_id,id_as_child:id_as_child,kind:AnalysisStepKind::Hide( to_hide )};
                        to_enqueue.push( generic_step );
                    } else {
                        add_action_matches_in_analysis(parent_id,&interaction,&head_actions,&mut id_as_child, &mut to_enqueue);
                    }
                },
                &AnalysisKind::Simulate(sim_before) => {
                    add_simulation_matches_in_analysis(parent_id,&interaction,&multi_trace,sim_before,&mut id_as_child, &mut to_enqueue);
                }
            }
        }
        // ***
        if id_as_child > 0 {
            let remaining_ids_to_process : HashSet<u32> = HashSet::from_iter((1..(id_as_child+1)).collect::<Vec<u32>>().iter().cloned() );
            let generic_node = GenericNode{kind:AnalysisNodeKind{interaction,loop_depth,multi_trace},remaining_ids_to_process,depth};
            self.manager.remember_state( parent_id, generic_node );
            self.manager.enqueue_new_steps( parent_id, to_enqueue, depth );
            return None;
        } else {
            let verdict = self.get_coverage_verdict(&interaction,&multi_trace);
            self.verdict_loggers(&verdict,parent_id);
            return Some( verdict );
        }
    }

    fn process_step(&mut self,
                    parent_state : &GenericNode<AnalysisConfig>,
                    to_process   : &GenericStep<AnalysisConfig>,
                    new_state_id : u32,
                    node_counter : u32) -> Option<(Interaction,AnalysableMultiTrace,u32,u32)> {
        match &(to_process.kind) {
            &AnalysisStepKind::Hide( ref lfs_to_hide ) => {
                let new_depth = parent_state.depth + 1;
                // ***
                match self.manager.apply_filters(new_depth,node_counter,&AnalysisFilterCriterion{loop_depth:parent_state.kind.loop_depth}) {
                    None => {
                        let new_interaction = (parent_state.kind.interaction).hide(lfs_to_hide);
                        // ***
                        let new_multi_trace = parent_state.kind.multi_trace.update_on_hide(&lfs_to_hide);
                        // ***
                        self.hiding_loggers(lfs_to_hide,
                                            &new_interaction,
                                            to_process.parent_id,
                                            new_state_id,
                                            &new_multi_trace);
                        // ***
                        return Some( (new_interaction,new_multi_trace,new_depth,parent_state.kind.loop_depth) );
                    },
                    Some( elim_kind ) => {
                        self.filtered_loggers(to_process.parent_id,
                                              new_state_id,
                                              &elim_kind);
                        return None;
                    }
                }
            },
            &AnalysisStepKind::Simulate( ref frt_elt, ref sim_map ) => {
                let new_depth = parent_state.depth + 1;
                let target_loop_depth = (parent_state.kind.interaction).get_loop_depth_at_pos(&frt_elt.position);
                let new_loop_depth = parent_state.kind.loop_depth + target_loop_depth;
                // ***
                match self.manager.apply_filters(new_depth,node_counter, &AnalysisFilterCriterion{loop_depth:new_loop_depth}) {
                    None => {
                        // ***
                        let exe_result = execute_interaction(&parent_state.kind.interaction,
                                                               &frt_elt.position,
                                                               &frt_elt.target_lf_ids,
                                                               true);
                        let rem_sim_depth : u32;
                        if sim_map.len() > 0 {
                            rem_sim_depth = parent_state.kind.multi_trace.remaining_loop_instantiations_in_simulation - target_loop_depth;
                        } else {
                            rem_sim_depth = exe_result.interaction.max_nested_loop_depth();
                        }
                        let new_multi_trace = parent_state.kind.multi_trace.update_on_simulation(sim_map,
                                                                                &frt_elt.target_lf_ids,
                                                                                &exe_result.affected_lifelines,
                                                                                rem_sim_depth);
                        // ***
                        self.execution_loggers(&frt_elt.position,
                                               &frt_elt.target_actions,
                                               sim_map,
                                               &exe_result.interaction,
                                               to_process.parent_id,
                                               new_state_id,
                                               &new_multi_trace);
                        // ***
                        return Some( (exe_result.interaction,new_multi_trace,new_depth,new_loop_depth) );
                    },
                    Some( elim_kind ) => {
                        self.filtered_loggers(to_process.parent_id,
                                              new_state_id,
                                              &elim_kind);
                        return None;
                    }
                }
            }
        }
    }

    pub fn get_coverage_verdict(&self,
                                interaction:&Interaction,
                                multi_trace:&AnalysableMultiTrace) -> CoverageVerdict {
        if multi_trace.length() == 0 {
            if interaction.express_empty() {
                match self.ana_kind {
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
                match self.ana_kind {
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
            match self.ana_kind {
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

    fn init_loggers(&mut self, interaction : &Interaction,remaining_multi_trace : &AnalysableMultiTrace) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_init(interaction, &self.manager.gen_ctx, &Some(remaining_multi_trace));
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

    fn term_loggers(&mut self, verdict : &GlobalVerdict) {
        let mut options_as_strs = (&self).manager.get_basic_options_as_strings();
        options_as_strs.insert(0, "process=analysis".to_string());
        options_as_strs.push( format!("analysis kind={}", self.ana_kind.to_string()) );
        match self.use_locana {
            UseLocalAnalysis::No => {
                options_as_strs.push("local_analysis=false".to_string());
            },
            UseLocalAnalysis::Yes => {
                options_as_strs.push("local_analysis=true".to_string());
            },
            _ => {
                panic!("TODO: implement")
            }
        }
        match self.goal.as_ref() {
            None => {
                options_as_strs.push( "goal=None".to_string() );
            },
            Some( target_goal ) => {
                options_as_strs.push( format!("goal={}", target_goal.to_string()) );
            }
        }
        options_as_strs.push( format!("verdict={}", verdict.to_string()) );
        // ***
        for logger in self.loggers.iter_mut() {
            (*logger).log_term(&options_as_strs);
        }
    }

    fn filtered_loggers(&mut self,
                        parent_state_id : u32,
                        new_state_id : u32,
                        elim_kind : &FilterEliminationKind) {
        for logger in self.loggers.iter_mut() {
            logger.log_filtered(&self.manager.gen_ctx,
                                parent_state_id,
                                new_state_id,
                                elim_kind);
        }
    }

    fn execution_loggers(&mut self,
                             action_position : &Position,
                             executed_actions : &HashSet<TraceAction>,
                             sim_map : &HashMap<usize,SimulationStepKind>,
                             new_interaction : &Interaction,
                             parent_state_id : u32,
                             new_state_id :u32,
                             remaining_multi_trace : &AnalysableMultiTrace) {
        for logger in self.loggers.iter_mut() {
            logger.log_execution(&self.manager.gen_ctx,
                                 parent_state_id,
                                 new_state_id,
                                 action_position,
                                 executed_actions,
                                 sim_map,
                                 new_interaction,
                                 &Some(remaining_multi_trace));
        }
    }

    fn hiding_loggers(&mut self,
                          lfs_to_hide : &HashSet<usize>,
                          new_interaction : &Interaction,
                          parent_state_id : u32,
                          new_state_id :u32,
                          remaining_multi_trace : &AnalysableMultiTrace) {
        for logger in self.loggers.iter_mut() {
            logger.log_hide(&self.manager.gen_ctx,
                            parent_state_id,
                            new_state_id,
                            lfs_to_hide,
                            new_interaction,
                            remaining_multi_trace);
        }
    }
}

