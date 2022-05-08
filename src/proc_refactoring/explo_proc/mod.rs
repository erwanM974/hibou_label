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


pub(in crate::proc_refactoring::explo_proc) mod step;
pub(in crate::proc_refactoring::explo_proc) mod filter;
pub(in crate::proc_refactoring::explo_proc) mod conf;
pub(in crate::proc_refactoring::explo_proc) mod priorities;
pub(in crate::proc_refactoring::explo_proc) mod node;



use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use crate::core::general_context::GeneralContext;


use crate::core::semantics::execute::execute_interaction;
use crate::core::semantics::frontier::global_frontier;
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::position::Position;
use crate::core::trace::TraceAction;
use crate::proc_refactoring::abstract_proc::{GenericNode, GenericProcessManager, GenericStep};
use crate::proc_refactoring::explo_proc::conf::ExplorationConfig;
use crate::proc_refactoring::explo_proc::filter::{ExplorationFilter, ExplorationFilterCriterion};
use crate::proc_refactoring::explo_proc::node::ExplorationNodeKind;
use crate::proc_refactoring::explo_proc::priorities::ExplorationPriorities;
use crate::proc_refactoring::explo_proc::step::ExplorationStepKind;
use crate::process::hibou_process::{FilterEliminationKind, HibouPreFilter, HibouSearchStrategy};
use crate::process::log::ProcessLogger;
use crate::process::priorities::ProcessPriorities;


pub fn explo_priorities_translate(proc_prio : ProcessPriorities) -> ExplorationPriorities {
    return ExplorationPriorities::new(proc_prio.emission,proc_prio.reception,0,proc_prio.in_loop);
}

pub fn explo_filter_translate(proc_filter : Vec<HibouPreFilter>) -> Vec<ExplorationFilter> {
    let mut exp_filters = vec![];
    for filter in proc_filter {
        match filter {
            HibouPreFilter::MaxProcessDepth(crit) => {
                exp_filters.push( ExplorationFilter::MaxProcessDepth(crit));
            },
            HibouPreFilter::MaxLoopInstanciation(crit) => {
                exp_filters.push( ExplorationFilter::MaxLoopInstanciation(crit));
            },
            HibouPreFilter::MaxNodeNumber(crit) => {
                exp_filters.push( ExplorationFilter::MaxNodeNumber(crit));
            }
        }
    }
    return exp_filters;
}


pub struct ExplorationProcessManager {
    manager: GenericProcessManager<ExplorationConfig>,
    loggers: Vec<Box<dyn ProcessLogger>>
}

impl ExplorationProcessManager {
    pub fn new(gen_ctx : GeneralContext,
               filters : Vec<ExplorationFilter>,
               strategy : HibouSearchStrategy,
               priorities : ExplorationPriorities,
               loggers : Vec<Box<dyn ProcessLogger>>) -> ExplorationProcessManager {
        let manager = GenericProcessManager::new(
            gen_ctx,
            strategy,
            filters,
            priorities,
            HashMap::new(),
            vec![]
        );
        return ExplorationProcessManager{manager,loggers};
    }

    pub fn explore(&mut self,interaction : Interaction) -> u32 {
        self.init_loggers(&interaction);
        // ***
        let mut next_state_id : u32 = 1;
        let mut node_counter : u32 = 0;
        self.enqueue_next_node_in_exploration(next_state_id,interaction,0,0);
        next_state_id = next_state_id + 1;
        node_counter = node_counter +1;
        // ***
        // ***
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
                Some( (new_interaction,new_depth,new_loop_depth) ) => {
                    node_counter = node_counter + 1;
                    self.enqueue_next_node_in_exploration(new_state_id,
                                                     new_interaction,
                                                     new_depth,
                                                     new_loop_depth);
                }
            }
            // ***
            parent_state.remaining_ids_to_process.remove(&next_to_process.id_as_child);
            if parent_state.remaining_ids_to_process.len() > 0 {
                self.manager.remember_state(next_to_process.parent_id,parent_state);
            }
            // ***
        }
        // ***
        self.term_loggers();
        // ***
        return node_counter;
    }

    fn enqueue_next_node_in_exploration(&mut self,
                                        parent_id    : u32,
                                        interaction : Interaction,
                                        depth       : u32,
                                        loop_depth  : u32) {
        // ***
        let mut id_as_child : u32 = 0;
        // ***
        let mut to_enqueue : Vec<GenericStep<ExplorationConfig>> = Vec::new();
        for front_pos in global_frontier(&interaction,&None) {
            let generic_step = GenericStep{parent_id,id_as_child,kind:ExplorationStepKind::Execute(front_pos)};
            id_as_child = id_as_child +1;
            to_enqueue.push( generic_step );
        }
        // ***
        if id_as_child > 0 {
            let remaining_ids_to_process : HashSet<u32> = HashSet::from_iter((1..(id_as_child+1)).collect::<Vec<u32>>().iter().cloned() );
            let generic_node = GenericNode{kind:ExplorationNodeKind{interaction,loop_depth},remaining_ids_to_process,depth};
            self.manager.remember_state( parent_id, generic_node );
            self.manager.enqueue_new_steps( parent_id, to_enqueue, depth );
        }
    }

    fn process_step(&mut self,
                        parent_state : &GenericNode<ExplorationConfig>,
                        to_process   : &GenericStep<ExplorationConfig>,
                        new_state_id : u32,
                        node_counter : u32) -> Option<(Interaction,u32,u32)> {
        match &(to_process.kind) {
            &ExplorationStepKind::Execute( ref frt_elt ) => {
                let new_depth = parent_state.depth + 1;
                let new_loop_depth = parent_state.kind.loop_depth + (parent_state.kind.interaction).get_loop_depth_at_pos(&frt_elt.position);
                // ***
                match self.manager.apply_filters(new_depth,node_counter,&ExplorationFilterCriterion{loop_depth:new_loop_depth}) {
                    None => {
                        // ***
                        let exe_result = execute_interaction(&parent_state.kind.interaction,
                                                             &frt_elt.position,
                                                             &frt_elt.target_lf_ids,
                                                             false);
                        // ***
                        self.execution_loggers(&frt_elt.position,
                                               &frt_elt.target_actions,
                                               &exe_result.interaction,
                                               to_process.parent_id,
                                               new_state_id);
                        // ***
                        return Some( (exe_result.interaction,new_depth,new_loop_depth) );
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

    fn init_loggers(&mut self, interaction : &Interaction) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_init(interaction, &self.manager.gen_ctx, &None);
        }
    }

    fn term_loggers(&mut self) {
        let mut options_as_strs = (&self).manager.get_basic_options_as_strings();
        options_as_strs.insert(0, "process=exploration".to_string());
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
                             new_interaction : &Interaction,
                             parent_state_id : u32,
                             new_state_id :u32) {
        for logger in self.loggers.iter_mut() {
            logger.log_execution(&self.manager.gen_ctx,
                                 parent_state_id,
                                 new_state_id,
                                 action_position,
                                 executed_actions,
                                 &HashMap::new(),
                                 new_interaction,
                                 &None);
        }
    }
}

