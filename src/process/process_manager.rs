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

use std::collections::HashMap;
use std::cmp::Reverse;

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction};
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;
use crate::core::semantics::execute::execute;
use crate::process::verdicts::*;
use crate::process::hibou_process::*;
use crate::core::trace::TraceActionKind;


pub enum PrioritizeActionKind {
    Emission,
    Reception,
    None
}

impl std::string::ToString for PrioritizeActionKind {
    fn to_string(&self) -> String {
        match self {
            PrioritizeActionKind::Emission => {
                return "Emission".to_string();
            },
            PrioritizeActionKind::Reception => {
                return "Reception".to_string();
            },
            PrioritizeActionKind::None => {
                return "None".to_string();
            }
        }
    }
}

pub struct ProcessPriorities {
    pub emission : i32,
    pub reception : i32,
    pub in_loop : i32
}

impl ProcessPriorities {
    pub fn new(emission : i32,
               reception : i32,
               in_loop : i32) -> ProcessPriorities {
        return ProcessPriorities{emission,reception,in_loop};
    }
}

impl std::string::ToString for ProcessPriorities {
    fn to_string(&self) -> String {
        let mut my_str = format!("emission={:},",self.emission);
        my_str.push_str( &format!("reception={:},",self.reception));
        my_str.push_str( &format!("in_loop={:}",self.in_loop));
        return my_str;
    }
}

pub struct HibouProcessManager {
    gen_ctx : GeneralContext,
    strategy : HibouSearchStrategy,
    sem_kind : Option<SemanticKind>,
    pre_filters : Vec<HibouPreFilter>,
    // ***
    memorized_states : HashMap<u32,MemorizedState>,
    process_queue : ProcessQueue,
    // ***
    frontier_priorities : ProcessPriorities,
    // ***
    loggers : Vec<Box<dyn ProcessLogger>>
}

impl HibouProcessManager {
    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               sem_kind : Option<SemanticKind>,
               pre_filters : Vec<HibouPreFilter>,
               memorized_states : HashMap<u32,MemorizedState>,
               process_queue : ProcessQueue,
               frontier_priorities : ProcessPriorities,
               loggers : Vec<Box<dyn ProcessLogger>>
    ) -> HibouProcessManager {
        return HibouProcessManager{gen_ctx,strategy,sem_kind,pre_filters,memorized_states,process_queue,frontier_priorities,loggers};
    }

    pub fn get_options_as_strings(&self,goal_and_verdict:Option<(&GlobalVerdict,&GlobalVerdict)>) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        match goal_and_verdict {
            None => {
                options_str.push("process=exploration".to_string());
            },
            Some( (goal,verd) ) => {
                options_str.push("process=analysis".to_string());
                options_str.push( format!("semantics={}", self.sem_kind.as_ref().unwrap().to_string()) );
                options_str.push( format!("goal={}", goal.to_string()) );
                options_str.push( format!("verdict={}", verd.to_string()) );
            }
        }
        options_str.push( format!("strategy={}", &self.strategy.to_string()) );
        options_str.push( format!("frontier_priorities=[{}]", &self.frontier_priorities.to_string()) );
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

    pub fn init_loggers(&mut self, interaction : &Interaction,remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_init(interaction, &self.gen_ctx, remaining_multi_trace);
        }
    }

    pub fn term_loggers(&mut self,goal_and_verdict:Option<(&GlobalVerdict,&GlobalVerdict)>) {
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
                            action_position : &Position,
                            executed_action : &TraceAction,
                            parent_state_id : u32,
                            new_state_id : u32,
                            elim_kind : &FilterEliminationKind) {
        for logger in self.loggers.iter_mut() {
            logger.log_filtered(&self.gen_ctx,
                                parent_state_id,
                                new_state_id,
                                action_position,
                                executed_action,
                                elim_kind);
        }
    }

    pub fn execution_loggers(&mut self,
                        action_position : &Position,
                        executed_action : &TraceAction,
                        new_interaction : &Interaction,
                        parent_state_id : u32,
                        new_state_id :u32,
                        remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            logger.log_execution(&self.gen_ctx,
                            parent_state_id,
                                 new_state_id,
                            action_position,
                            executed_action,
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
        return self.process_queue.get_next();
    }

    pub fn enqueue_executions(&mut self, state_id : u32, to_enqueue : Vec<(u32,NextToProcessKind)>) {
        let mut to_enqueue_reorganize : HashMap<i32,Vec<(u32,NextToProcessKind)>> = HashMap::new();
        for (child_id,child_kind) in to_enqueue {
            match &child_kind {
                &NextToProcessKind::Execute( ref front_pos ) => {
                    let mut priority : i32 = 0;
                    // ***
                    let parent_state = self.get_memorized_state(state_id).unwrap();
                    let front_act = (parent_state.interaction).get_sub_interaction(&front_pos).as_leaf();
                    match front_act.act_kind {
                        ObservableActionKind::Reception => {
                            priority = priority + self.frontier_priorities.reception;
                        },
                        ObservableActionKind::Emission(_) => {
                            priority = priority + self.frontier_priorities.emission;
                        }
                    }
                    let loop_depth = (parent_state.interaction).get_loop_depth_at_pos(&front_pos);
                    if loop_depth > 0 {
                        priority = priority + self.frontier_priorities.in_loop;
                    }
                    // ***
                    match to_enqueue_reorganize.get_mut(&priority) {
                        None => {
                            to_enqueue_reorganize.insert(priority,vec![ (child_id,child_kind) ]);
                        },
                        Some( queue ) => {
                            queue.push((child_id,child_kind) );
                        }
                    }
                    // ***
                }
            }
        }
        // ***
        let mut to_enqueue_reorganized : Vec<(u32,NextToProcessKind)> = Vec::new();
        {
            let mut keys : Vec<i32> = to_enqueue_reorganize.keys().cloned().collect();
            keys.sort_by_key(|k| Reverse(*k));
            for k in keys {
                match to_enqueue_reorganize.get_mut(&k) {
                    None => {},
                    Some( queue ) => {
                        to_enqueue_reorganized.append( queue );
                    }
                }
            }
        }
        // ***
        match &self.strategy {
            &HibouSearchStrategy::DFS => {
                to_enqueue_reorganized.reverse();
                for (child_id,child_kind) in to_enqueue_reorganized {
                    self.enqueue_child_node(state_id,child_id,child_kind);
                }
            },
            &HibouSearchStrategy::BFS => {
                for (child_id,child_kind) in to_enqueue_reorganized {
                    self.enqueue_child_node(state_id,child_id,child_kind);
                }
            }
        }
    }

    fn enqueue_child_node(&mut self,state_id: u32,child_id:u32,child_kind:NextToProcessKind) {
        let child = NextToProcess::new(state_id,child_id,child_kind);
        match &(self.strategy) {
            &HibouSearchStrategy::DFS => {
                self.process_queue.insert_item_left(child);
            },
            &HibouSearchStrategy::BFS => {
                self.process_queue.insert_item_right(child);
            }
        }
    }

    pub fn process_next(&mut self,
                        parent_state : &MemorizedState,
                        to_process   : &NextToProcess,
                        new_state_id : u32,
                        node_counter : u32) -> Option<(Interaction,Option<AnalysableMultiTrace>,u32,u32)> {
        match &(to_process.kind) {
            &NextToProcessKind::Execute( ref position ) => {
                let new_depth = parent_state.depth + 1;
                let new_loop_depth = parent_state.loop_depth + (parent_state.interaction).get_loop_depth_at_pos(position);
                // ***
                let target_action = (parent_state.interaction).get_sub_interaction(position).as_leaf();
                match self.apply_pre_filters(new_depth,new_loop_depth,node_counter) {
                    None => {
                        let ex_lf_id = target_action.occupation_before();
                        let new_interaction = execute((parent_state.interaction).clone(),position.clone(),ex_lf_id);
                        // ***
                        let new_multi_trace : Option<AnalysableMultiTrace>;
                        match (parent_state.multi_trace).as_ref(){
                            None => {
                                new_multi_trace = None;
                            },
                            Some( ref multi_trace ) => {
                                let mut new_canals : Vec<MultiTraceCanal> = Vec::new();
                                for canal in &multi_trace.canals {
                                    if canal.lifelines.contains(&ex_lf_id) {
                                        let mut new_trace = canal.trace.clone();
                                        new_trace.remove(0);
                                        new_canals.push( MultiTraceCanal{lifelines:canal.lifelines.clone(),trace:new_trace} )
                                    } else {
                                        new_canals.push(canal.clone());
                                    }
                                }
                                new_multi_trace = Some( AnalysableMultiTrace::new(new_canals) );
                            }
                        }
                        // ***
                        self.execution_loggers(&position,
                                          &target_action.to_trace_action(),
                                          &new_interaction,
                                          to_process.state_id,
                                               new_state_id,
                                          &new_multi_trace);
                        // ***
                        return Some( (new_interaction,new_multi_trace,new_depth,new_loop_depth) );
                    },
                    Some( elim_kind ) => {
                        self.filtered_loggers(&position,
                                              &target_action.to_trace_action(),
                                              to_process.state_id,
                                              new_state_id,
                                              &elim_kind);
                        return None;
                    }
                }
            },
            _ => {
                return None;
            }
        }
    }

    fn apply_pre_filters(&self, depth : u32, loop_depth : u32, node_counter : u32) -> Option<FilterEliminationKind> {
        for pre_filter in &self.pre_filters {
            match pre_filter {
                HibouPreFilter::MaxProcessDepth( max_depth ) => {
                    if depth > *max_depth {
                        return Some( FilterEliminationKind::MaxProcessDepth );
                    }
                },
                HibouPreFilter::MaxLoopInstanciation( loop_num ) => {
                    if loop_depth > *loop_num {
                        return Some( FilterEliminationKind::MaxLoopInstanciation );
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

    pub fn get_coverage_verdict(&self,interaction:&Interaction,multi_trace:&AnalysableMultiTrace) -> CoverageVerdict {
        if multi_trace.length() == 0 {
            if interaction.express_empty() {
                return CoverageVerdict::Cov;
            } else {
                match self.sem_kind.as_ref().unwrap() {
                    SemanticKind::Accept => {
                        return CoverageVerdict::UnCov;
                    },
                    SemanticKind::Prefix => {
                        return CoverageVerdict::TooShort;
                    }
                }
            }
        } else {
            match self.sem_kind.as_ref().unwrap() {
                SemanticKind::Accept => {
                    return CoverageVerdict::UnCov;
                },
                SemanticKind::Prefix => {
                    if multi_trace.is_any_component_empty() {
                        return CoverageVerdict::LackObs;
                    } else {
                        return CoverageVerdict::Out;
                    }
                }
            }
        }
    }

}