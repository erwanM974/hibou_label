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



use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::core::general_context::GeneralContext;


use crate::core::semantics::execute::execute_interaction;
use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::position::Position;
use crate::core::syntax::util::check_interaction::InteractionCharacteristics;
use crate::core::trace::{TraceAction};
use crate::process::ana_proc::multitrace::{AnalysableMultiTraceCanal, AnalysableMultiTrace, WasMultiTraceConsumedWithSimulation};
use crate::process::abstract_proc::generic::*;
use crate::process::abstract_proc::manager::*;
use crate::process::abstract_proc::common::{FilterEliminationKind, HibouSearchStrategy};
use crate::process::ana_proc::anakind::{AnalysisKind, SimulationActionCriterion, SimulationLoopCriterion, UseLocalAnalysis};
use crate::process::ana_proc::interface::conf::AnalysisConfig;
use crate::process::ana_proc::interface::filter::{AnalysisFilter, AnalysisFilterCriterion};
use crate::process::ana_proc::interface::logger::AnalysisLogger;
use crate::process::ana_proc::local_analysis::is_dead_local_analysis;
use crate::process::ana_proc::matches::*;
use crate::process::ana_proc::interface::step::{AnalysisStepKind, SimulationStepKind};
use crate::process::ana_proc::interface::node::AnalysisNodeKind;
use crate::process::ana_proc::interface::priorities::AnalysisPriorities;
use crate::process::ana_proc::verdicts::{CoverageVerdict, GlobalVerdict, InconcReason, update_global_verdict_from_new_coverage_verdict};
use crate::rendering::textual::monochrome::multi_trace::multi_trace_as_text;


pub struct AnalysisProcessManager {
    pub(crate) manager: GenericProcessManager<AnalysisConfig>,
    pub(crate) ana_kind : AnalysisKind,
    pub(crate) use_locana : UseLocalAnalysis,
    pub(crate) goal : Option<GlobalVerdict>,
    pub(crate) has_filtered_nodes : bool
}

impl AnalysisProcessManager {
    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               filters : Vec<AnalysisFilter>,
               priorities : GenericProcessPriorities<AnalysisConfig>,
               loggers : Vec<Box< dyn AnalysisLogger>>,
               ana_kind : AnalysisKind,
               use_locana : UseLocalAnalysis,
               goal : Option<GlobalVerdict>) -> AnalysisProcessManager {
        let manager = GenericProcessManager::new(
            gen_ctx,
            strategy,
            filters,
            priorities,
            loggers
        );
        return AnalysisProcessManager{manager,ana_kind,use_locana,goal,has_filtered_nodes:false};
    }

    pub fn analyze(&mut self,
                   interaction : Interaction,
                   int_characs : InteractionCharacteristics,
                   got_multi_trace : AnalysableMultiTrace) -> (GlobalVerdict,u32) {
        // ***
        let mut multi_trace = got_multi_trace;
        match &self.ana_kind {
            AnalysisKind::Simulate( sim_config ) => {
                let rem_loop_in_sim = sim_config.get_reset_rem_loop(&interaction);
                let rem_act_in_sim = sim_config.get_reset_rem_act(&interaction);
                multi_trace.rem_loop_in_sim = rem_loop_in_sim;
                multi_trace.rem_act_in_sim = rem_act_in_sim;
            },
            _ => {}
        }
        // ***
        self.init_loggers(&interaction,&multi_trace);
        // ***
        let mut next_state_id : u32 = 1;
        let mut node_counter : u32 = 0;
        let mut global_verdict = GlobalVerdict::Fail;
        // ***
        // ***
        let pursue_analysis : bool;
        match self.enqueue_next_node_in_analysis(next_state_id,
                                                 interaction,&int_characs,
                                                 multi_trace,
                                                 0,
                                                 0) {
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
                                                                 &int_characs,
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
                }/* else {
                    // here maybe do stuff to check if node is terminal etc etc
                }
                */
                // ***
            }
        }
        // ***
        if global_verdict == GlobalVerdict::Fail && self.has_filtered_nodes {
            global_verdict = GlobalVerdict::Inconc(InconcReason::FilteredNodes);
        }
        // ***
        self.term_loggers(&global_verdict);
        // ***
        return (global_verdict,node_counter);
    }

    fn enqueue_next_node_in_analysis(&mut self,
                                     parent_id    : u32,
                                     interaction : Interaction,
                                     initial_int_characs : &InteractionCharacteristics,
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
                let verdict : CoverageVerdict;
                if self.ana_kind.has_simulation() {
                    verdict = CoverageVerdict::OutSim(true);
                } else {
                    verdict = CoverageVerdict::Out(true);
                }
                self.verdict_loggers(&verdict,parent_id);
                return Some( verdict );
            }
            // ***
            let head_actions = multi_trace.head_actions();
            // ***
            // ***
            match &self.ana_kind {
                &AnalysisKind::Accept => {
                    self.add_action_matches_in_analysis(parent_id,&interaction,&head_actions,&mut id_as_child, &mut to_enqueue);
                },
                &AnalysisKind::Prefix => {
                    self.add_action_matches_in_analysis(parent_id,&interaction,&head_actions,&mut id_as_child, &mut to_enqueue);
                },
                &AnalysisKind::Hide => {
                    let mut to_hide : HashSet<usize> = HashSet::new();
                    for (canal_id,canal) in multi_trace.canals.iter().enumerate() {
                        if (canal.flag_hidden == false) && (canal.trace.len() == 0) { // && (interaction.involves_any_of(&canal.lifelines)) {
                            let canal_lifelines = self.manager.gen_ctx.co_localizations.get(canal_id).unwrap();
                            to_hide.extend( canal_lifelines )
                        }
                    }
                    //
                    if to_hide.len() > 0 {
                        id_as_child = id_as_child + 1;
                        let generic_step = GenericStep{parent_id,id_as_child:id_as_child,kind:AnalysisStepKind::Hide( to_hide )};
                        to_enqueue.push( generic_step );
                    } else {
                        self.add_action_matches_in_analysis(parent_id,&interaction,&head_actions,&mut id_as_child, &mut to_enqueue);
                    }
                },
                &AnalysisKind::Simulate(_) => {
                    self.add_simulation_matches_in_analysis(parent_id, &interaction, &multi_trace,&mut id_as_child, &mut to_enqueue);
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
            // here informs the queue
            // knows that the last node had no child and hence
            // for the HCS search strategy
            // selects the highest parent in the next step instead of continuing on as in DFS
            self.manager.queue_set_last_reached_has_no_child();
            // ***
            let verdict = self.get_coverage_verdict(initial_int_characs,&interaction,&multi_trace);
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
                        let new_multi_trace = parent_state.kind.multi_trace.update_on_hide(&self.manager.gen_ctx,&lfs_to_hide);
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
            &AnalysisStepKind::Simulate( ref frt_elt, ref consu_set, ref sim_map ) => {
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
                        let new_multi_trace = parent_state.kind.multi_trace.update_on_simulation(self.ana_kind.get_sim_config().unwrap(),
                                                                                                 consu_set,
                                                                                                 sim_map,
                                                                                                 &self.manager.gen_ctx,
                                                                                                 &frt_elt.target_lf_ids,
                                                                                                 &exe_result.affected_lifelines,
                                                                                                 target_loop_depth,
                                                                                                 &exe_result.interaction);
                        // ***
                        self.execution_loggers(&frt_elt.position,
                                               &frt_elt.target_actions,
                                               consu_set,
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
                                initial_int_characs : &InteractionCharacteristics,
                                interaction : &Interaction,
                                multi_trace : &AnalysableMultiTrace) -> CoverageVerdict {
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
                            if self.manager.gen_ctx.are_colocalizations_singletons() {
                                return CoverageVerdict::MultiPref;
                            } else {
                                return CoverageVerdict::Inconc(InconcReason::HideWithColocs);
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
            } else { /* multi-trace empty but interaction does not express empty */
                match self.ana_kind {
                    AnalysisKind::Accept => {
                        return CoverageVerdict::Out(false);
                    },
                    AnalysisKind::Prefix => {
                        return CoverageVerdict::TooShort;
                    },
                    AnalysisKind::Hide => {
                        if multi_trace.is_any_component_hidden() {
                            if self.manager.gen_ctx.are_colocalizations_singletons() {
                                return CoverageVerdict::MultiPref;
                            } else {
                                return CoverageVerdict::Inconc(InconcReason::HideWithColocs);
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
        } else { /* multi-trace not emptied */
            match self.ana_kind {
                AnalysisKind::Accept => {
                    return CoverageVerdict::Out(false);
                },
                AnalysisKind::Prefix => {
                    if multi_trace.is_any_component_empty() {
                        return CoverageVerdict::Inconc(InconcReason::LackObs);
                    } else {
                        return CoverageVerdict::Out(false);
                    }
                },
                AnalysisKind::Hide => {
                    return CoverageVerdict::Out(false);
                },
                AnalysisKind::Simulate(_) => {
                    return CoverageVerdict::OutSim(false);
                }
            }
        }
    }

    fn init_loggers(&mut self, interaction : &Interaction,remaining_multi_trace : &AnalysableMultiTrace) {
        let (is_simulation,sim_crit_loop,sim_crit_act) = self.ana_kind.get_sim_crits();
        for logger in self.manager.loggers.iter_mut() {
            (*logger).log_init( &self.manager.gen_ctx, interaction,remaining_multi_trace,is_simulation,sim_crit_loop,sim_crit_act);
        }
    }

    pub fn verdict_loggers(&mut self,
                           verdict : &CoverageVerdict,
                           parent_state_id : u32) {
        for logger in self.manager.loggers.iter_mut() {
            logger.log_verdict(parent_state_id,
                               verdict);
        }
    }

    fn term_loggers(&mut self, verdict : &GlobalVerdict) {
        let mut options_as_strs = (&self).manager.get_basic_options_as_strings();
        options_as_strs.insert(0, "process=analysis".to_string());
        options_as_strs.push( format!("analysis kind={}", self.ana_kind.to_string()) );
        options_as_strs.push( format!("local analysis={}", self.use_locana.to_string()) );
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
        for logger in self.manager.loggers.iter_mut() {
            (*logger).log_term(&options_as_strs);
        }
    }

    fn filtered_loggers(&mut self,
                        parent_state_id : u32,
                        new_state_id : u32,
                        elim_kind : &FilterEliminationKind) {
        self.has_filtered_nodes = true;
        for logger in self.manager.loggers.iter_mut() {
            logger.log_filtered(parent_state_id,
                                new_state_id,
                                elim_kind);
        }
    }

    fn execution_loggers(&mut self,
                         action_position : &Position,
                         executed_actions : &HashSet<TraceAction>,
                         consu_set : &HashSet<usize>,
                         sim_map : &HashMap<usize,SimulationStepKind>,
                         new_interaction : &Interaction,
                         parent_state_id : u32,
                         new_state_id :u32,
                         remaining_multi_trace : &AnalysableMultiTrace) {
        let (is_simulation,sim_crit_loop,sim_crit_act) = self.ana_kind.get_sim_crits();
        for logger in self.manager.loggers.iter_mut() {
            logger.log_execution(&self.manager.gen_ctx,
                                 parent_state_id,
                                 new_state_id,
                                 action_position,
                                 executed_actions,
                                 consu_set,
                                 sim_map,
                                 new_interaction,
                                 remaining_multi_trace,
                                 is_simulation,sim_crit_loop,sim_crit_act);
        }
    }

    fn hiding_loggers(&mut self,
                      lfs_to_hide : &HashSet<usize>,
                      new_interaction : &Interaction,
                      parent_state_id : u32,
                      new_state_id :u32,
                      remaining_multi_trace : &AnalysableMultiTrace) {
        for logger in self.manager.loggers.iter_mut() {
            logger.log_hide(&self.manager.gen_ctx,
                            parent_state_id,
                            new_state_id,
                            lfs_to_hide,
                            new_interaction,
                            remaining_multi_trace);
        }
    }
}

