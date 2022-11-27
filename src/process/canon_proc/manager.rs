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
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::position::position::Position;
use crate::core::transformation::transfodef::InteractionTransformation;
use crate::core::transformation::transfokind::InteractionTransformationKind;
use crate::process::abstract_proc::common::{FilterEliminationKind, HibouSearchStrategy};
use crate::process::abstract_proc::generic::{GenericNode, GenericStep};
use crate::process::abstract_proc::manager::{GenericProcessManager, GenericProcessPriorities};
use crate::process::canon_proc::interface::conf::CanonizationConfig;
use crate::process::canon_proc::interface::filter::{CanonizationFilter, CanonizationFilterCriterion};
use crate::process::canon_proc::interface::logger::CanonizationLogger;
use crate::process::canon_proc::interface::node::CanonizationNodeKind;
use crate::process::canon_proc::interface::step::CanonizationStepKind;
use crate::process::canon_proc::transformations::get_transfos::get_canonize_transfos;
use crate::process::canon_proc::transformations::phases::CanonizationPhase;



pub enum IntRefOrOldIdRef<'l> {
    IntRef(&'l Interaction),
    OldIDRef(u32)
}


pub struct CanonizationProcessManager {
    pub(crate) manager: GenericProcessManager<CanonizationConfig>,
    pub(crate) get_all_transfos : bool,
    pub(crate) node_has_child_interaction : HashSet<u32>,
    phase1_known : HashMap<Interaction,u32>, // interaction term and corresponding state id
    phase2_known : HashMap<Interaction,u32> // interaction term and corresponding state id
}

impl CanonizationProcessManager {

    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               filters : Vec<CanonizationFilter>,
               priorities : GenericProcessPriorities<CanonizationConfig>,
               loggers : Vec<Box< dyn CanonizationLogger>>,
               get_all_transfos : bool) -> CanonizationProcessManager {
        let manager = GenericProcessManager::new(
            gen_ctx,
            strategy,
            filters,
            priorities,
            loggers
        );
        return CanonizationProcessManager{manager,
            get_all_transfos,
            node_has_child_interaction:HashSet::new(),
            phase1_known:HashMap::new(),
            phase2_known:HashMap::new()};
    }

    pub fn canonize(&mut self,
                   interaction : Interaction) -> u32 {
        self.init_loggers(&interaction);
        // ***
        let mut next_state_id : u32 = 1;
        let mut node_counter : u32 = 0;
        self.enqueue_next_node_in_canonization(next_state_id,interaction,CanonizationPhase::FirstDefactorize,0);
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
                                    next_to_process.kind,
                                    next_to_process.parent_id,
                                    new_state_id,
                                    node_counter) {
                None => {},
                Some( (new_interaction,new_depth,new_phase) ) => {
                    self.node_has_child_interaction.insert(next_to_process.parent_id);
                    node_counter = node_counter + 1;
                    self.enqueue_next_node_in_canonization(new_state_id,
                                                          new_interaction,
                                                           new_phase,
                                                          new_depth);
                }
            }
            // ***
            parent_state.remaining_ids_to_process.remove(&next_to_process.id_as_child);
            if parent_state.remaining_ids_to_process.len() > 0 {
                self.manager.remember_state(next_to_process.parent_id,parent_state);
            } else {
                let parent_has_child_interaction = self.node_has_child_interaction.remove(&next_to_process.parent_id);
                if !parent_has_child_interaction {
                    self.manager.queue_set_last_reached_has_no_child();
                }
            }
            // ***
        }
        // ***
        self.term_loggers();
        // ***
        return node_counter;
    }

    fn enqueue_next_node_in_canonization(&mut self,
                                        parent_id    : u32,
                                        interaction : Interaction,
                                         phase : CanonizationPhase,
                                        depth       : u32) {
        // ***
        let transfos = get_canonize_transfos(&interaction,&phase,self.get_all_transfos);
        // ***
        let mut id_as_child : u32 = 0;
        // ***
        let mut to_enqueue : Vec<GenericStep<CanonizationConfig>> = Vec::new();
        for transfo in transfos {
            id_as_child = id_as_child + 1;
            let generic_step = GenericStep{parent_id,id_as_child,kind:CanonizationStepKind::Transform(transfo)};
            to_enqueue.push( generic_step );
        }
        // ***
        if id_as_child == 0 {
            if phase == CanonizationPhase::FirstDefactorize {
                id_as_child = id_as_child + 1;
                let generic_step = GenericStep{parent_id,id_as_child,kind:CanonizationStepKind::ChangePhase};
                to_enqueue.push( generic_step );
            }
        }
        // ***
        if id_as_child > 0 {
            let remaining_ids_to_process : HashSet<u32> = HashSet::from_iter((1..(id_as_child+1)).collect::<Vec<u32>>().iter().cloned() );
            let generic_node = GenericNode{kind:CanonizationNodeKind{interaction,phase},remaining_ids_to_process,depth};
            self.manager.remember_state( parent_id, generic_node );
            self.manager.enqueue_new_steps( parent_id, to_enqueue, depth );
        } else {
            self.manager.queue_set_last_reached_has_no_child();
        }
    }

    fn process_step(&mut self,
                        parent_state : &GenericNode<CanonizationConfig>,
                        step_kind   : CanonizationStepKind,
                        parent_id : u32,
                        new_state_id : u32,
                        node_counter : u32) -> Option<(Interaction,u32,CanonizationPhase)> {
        match step_kind {
            CanonizationStepKind::Transform( transfo ) => {
                match &parent_state.kind.phase {
                    CanonizationPhase::FirstDefactorize => {
                        match self.phase1_known.get(&transfo.result) {
                            None => {
                                self.phase1_known.insert(transfo.result.clone(),new_state_id);
                                return self.process_new_transformation_step(parent_state,transfo,parent_id,new_state_id,node_counter);
                            },
                            Some(old_state_id) => {
                                self.transformation_loggers(&parent_state.kind.phase,
                                                            &transfo.kind,
                                                            &transfo.position,
                                                            &IntRefOrOldIdRef::OldIDRef(*old_state_id),
                                                            parent_id,
                                                            new_state_id);
                                return None;
                            }
                        }
                    },
                    CanonizationPhase::SecondFactorize => {
                        match self.phase2_known.get(&transfo.result) {
                            None => {
                                self.phase2_known.insert(transfo.result.clone(),new_state_id);
                                return self.process_new_transformation_step(parent_state,transfo,parent_id,new_state_id,node_counter);
                            },
                            Some(old_state_id) => {
                                self.transformation_loggers(&parent_state.kind.phase,
                                                            &transfo.kind,
                                                            &transfo.position,
                                                            &IntRefOrOldIdRef::OldIDRef(*old_state_id),
                                                            parent_id,
                                                            new_state_id);
                                return None;
                            }
                        }
                    }
                }
            },
            CanonizationStepKind::ChangePhase => {
                match self.phase2_known.get(&parent_state.kind.interaction) {
                    None => {
                        self.phase2_known.insert(parent_state.kind.interaction.clone(),new_state_id);
                        self.phase_change_loggers(&IntRefOrOldIdRef::IntRef(&parent_state.kind.interaction),
                                                  parent_id,
                                                  new_state_id);
                        let new_depth = parent_state.depth + 1;
                        return Some( (parent_state.kind.interaction.clone(),new_depth,CanonizationPhase::SecondFactorize) );
                    },
                    Some(old_state_id) => {
                        self.phase_change_loggers(&IntRefOrOldIdRef::OldIDRef(*old_state_id),
                                                  parent_id,
                                                  new_state_id);
                        return None;
                    }
                }
            }
        }
    }



    fn process_new_transformation_step(&mut self,
                        parent_state : &GenericNode<CanonizationConfig>,
                        transfo   : InteractionTransformation,
                        parent_id : u32,
                        new_state_id : u32,
                        node_counter : u32) -> Option<(Interaction,u32,CanonizationPhase)> {
        let new_depth = parent_state.depth + 1;
        match self.manager.apply_filters(new_depth,node_counter,&CanonizationFilterCriterion) {
            None => {
                self.transformation_loggers(&parent_state.kind.phase,
                                            &transfo.kind,
                                            &transfo.position,
                                            &IntRefOrOldIdRef::IntRef(&transfo.result),
                                            parent_id,
                                            new_state_id);
                return Some( (transfo.result,new_depth,parent_state.kind.phase.clone()) );
            },
            Some( elim_kind ) => {
                self.filtered_loggers(&parent_state.kind.phase,
                                      parent_id,
                                      new_state_id,
                                      &elim_kind);
                return None;
            }
        }
    }

    fn init_loggers(&mut self, interaction : &Interaction) {
        for logger in self.manager.loggers.iter_mut() {
            (*logger).log_init(interaction, &self.manager.gen_ctx);
        }
    }

    fn term_loggers(&mut self) {
        let mut options_as_strs = (&self).manager.get_basic_options_as_strings();
        options_as_strs.insert(0, "process=canonization".to_string());
        options_as_strs.push( format!("search_all={}", self.get_all_transfos.to_string()) );
        for logger in self.manager.loggers.iter_mut() {
            (*logger).log_term(&options_as_strs);
        }
    }

    fn filtered_loggers(&mut self,
                        phase : &CanonizationPhase,
                        parent_state_id : u32,
                        new_state_id : u32,
                        elim_kind : &FilterEliminationKind) {
        for logger in self.manager.loggers.iter_mut() {
            logger.log_filtered(phase,
                                parent_state_id,
                                new_state_id,
                                elim_kind);
        }
    }

    fn transformation_loggers(&mut self,
                              current_phase : &CanonizationPhase,
                              transfo_kind : &InteractionTransformationKind,
                              position : &Position,
                              new_interaction : &IntRefOrOldIdRef,
                              parent_state_id : u32,
                              new_state_id : u32) {
        for logger in self.manager.loggers.iter_mut() {
            logger.log_transformation(&self.manager.gen_ctx,
                                      current_phase,
                                      parent_state_id,
                                      new_state_id,
                                      transfo_kind,
                                      position,
                                      new_interaction);
        }
    }

    fn phase_change_loggers(&mut self,
                            new_interaction : &IntRefOrOldIdRef,
                            parent_state_id : u32,
                            new_state_id : u32) {
        for logger in self.manager.loggers.iter_mut() {
            (*logger).log_phase_change(&self.manager.gen_ctx,parent_state_id,new_state_id,new_interaction);
        }
    }

}

