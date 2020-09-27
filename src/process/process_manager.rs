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

pub struct HibouProcessManager {
    gen_ctx : GeneralContext,
    strategy : HibouSearchStrategy,
    sem_kind : Option<SemanticKind>,
    pre_filters : Vec<HibouPreFilter>,
    // ***
    memorized_states : HashMap<Vec<u32>,MemorizedState>,
    process_queue : ProcessQueue,
    // ***
    prioritize_action : PrioritizeActionKind,
    // ***
    loggers : Vec<Box<dyn ProcessLogger>>
}

impl HibouProcessManager {
    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               sem_kind : Option<SemanticKind>,
               pre_filters : Vec<HibouPreFilter>,
               memorized_states : HashMap<Vec<u32>,MemorizedState>,
               process_queue : ProcessQueue,
               prioritize_action:PrioritizeActionKind,
               loggers : Vec<Box<dyn ProcessLogger>>
    ) -> HibouProcessManager {
        return HibouProcessManager{gen_ctx,strategy,sem_kind,pre_filters,memorized_states,process_queue,prioritize_action,loggers};
    }

    pub fn get_options_as_strings(&self,verdict:Option<&GlobalVerdict>) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        match verdict {
            None => {
                options_str.push("process=exploration".to_string());
            },
            Some(verd) => {
                options_str.push("process=analysis".to_string());
                options_str.push( format!("semantics={}", self.sem_kind.as_ref().unwrap().to_string()) );
                options_str.push( format!("verdict={}", verd.to_string()) );
            }
        }
        options_str.push( format!("strategy={}", &self.strategy.to_string()) );
        options_str.push( format!("prioritize_actions={}", &self.prioritize_action.to_string()) );
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

    pub fn term_loggers(&mut self,verd:Option<&GlobalVerdict>) {
        let options_as_strs = (&self).get_options_as_strings(verd);
        for logger in self.loggers.iter_mut() {
            (*logger).log_term(&options_as_strs);
        }
    }

    pub fn verdict_loggers(&mut self,
                           verdict : &CoverageVerdict,
                           node_path : &Vec<u32>) {
        for logger in self.loggers.iter_mut() {
            logger.log_verdict(node_path,
                               verdict);
        }
    }

    pub fn filtered_loggers(&mut self,
                            action_position : &Position,
                            executed_action : &ObservableAction,
                            parent_node_path : &Vec<u32>,
                            current_node_path : &Vec<u32>,
                            elim_kind : &FilterEliminationKind) {
        for logger in self.loggers.iter_mut() {
            logger.log_filtered(&self.gen_ctx,
                                parent_node_path,
                                current_node_path,
                                action_position,
                                executed_action,
                                elim_kind);
        }
    }

    pub fn next_loggers(&mut self,
                        action_position : &Position,
                        executed_action : &ObservableAction,
                        new_interaction : &Interaction,
                        parent_node_path : &Vec<u32>,
                        current_node_path : &Vec<u32>,
                        remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        for logger in self.loggers.iter_mut() {
            logger.log_next(&self.gen_ctx,
                            parent_node_path,
                            current_node_path,
                            action_position,
                            executed_action,
                            new_interaction,
                            remaining_multi_trace);
        }
    }

    pub fn get_memorized_state(&self, id:&Vec<u32>) -> Option<&MemorizedState> {
        return self.memorized_states.get(id);
    }

    pub fn forget_state(&mut self, id:&Vec<u32>) {
        self.memorized_states.remove(id);
    }

    pub fn remember_state(&mut self, id:Vec<u32>, state:MemorizedState) {
        self.memorized_states.insert( id, state );
    }

    pub fn extract_from_queue(&mut self) -> Option<NextToProcess> {
        return self.process_queue.get_next();
    }

    pub fn enqueue_executions(&mut self, state_id : &Vec<u32>, to_enqueue : Vec<(u32,Position,TraceActionKind)>) {
        match &self.strategy {
            &HibouSearchStrategy::DFS => {
                let mut to_enqueue = to_enqueue;
                to_enqueue.reverse();
                for (child_id,front_pos,act_kind) in to_enqueue {
                    self.enqueue_child_node(state_id,child_id,front_pos,act_kind);
                }
            },
            &HibouSearchStrategy::BFS => {
                for (child_id,front_pos,act_kind) in to_enqueue {
                    self.enqueue_child_node(state_id,child_id,front_pos,act_kind);
                }
            }
        }
    }

    fn enqueue_child_node(&mut self,state_id: &Vec<u32>,child_id:u32,front_pos:Position,act_kind:TraceActionKind) {
        let child = NextToProcess::new(state_id.clone(),child_id,NextToProcessKind::Execute(front_pos));
        let mut priority : u32 = 0;
        match self.prioritize_action {
            PrioritizeActionKind::None => {},
            PrioritizeActionKind::Reception => {
                match act_kind {
                    TraceActionKind::Reception => {
                        priority = priority + 1;
                    },
                    _ => {}
                }
            },
            PrioritizeActionKind::Emission => {
                match act_kind {
                    TraceActionKind::Emission => {
                        priority = priority + 1;
                    },
                    _ => {}
                }
            }
        }
        self.process_queue.insert_item_left(child,priority);
    }

    pub fn process_next(&mut self,
                        parent_node_path:&Vec<u32>,
                        next_node_path:&Vec<u32>,
                        parent_interaction : &Interaction,
                        parent_multi_trace : &Option<AnalysableMultiTrace>,
                        position:Position,
                        process_node_count : u32,
                        parent_loop_depth:u32) -> Option<(Interaction,Option<AnalysableMultiTrace>,u32)> {
        // ***
        let new_loop_depth = parent_loop_depth + parent_interaction.get_loop_depth_at_pos(&position);
        // ***
        let target_action = parent_interaction.get_sub_interaction(&position).as_leaf();
        match self.apply_pre_filters(parent_node_path,new_loop_depth,process_node_count) {
            None => {
                let ex_lf_id = target_action.occupation_before();
                let new_interaction = execute(parent_interaction.clone(),position.clone(),ex_lf_id);
                // ***
                let new_multi_trace : Option<AnalysableMultiTrace>;
                match parent_multi_trace {
                    None => {
                        new_multi_trace = None;
                    },
                    Some( multi_trace ) => {
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
                self.next_loggers(&position,
                                  target_action,
                                  &new_interaction,
                                  &parent_node_path,
                                  next_node_path,
                                  &new_multi_trace);
                return Some( (new_interaction,new_multi_trace,new_loop_depth) );
            },
            Some( elim_kind ) => {
                self.filtered_loggers(&position,target_action,&parent_node_path,&next_node_path, &elim_kind);
                return None;
            }
        }
    }

    fn apply_pre_filters(&self, parent_node_path : &Vec<u32>, loop_depth : u32, node_counter : u32) -> Option<FilterEliminationKind> {
        for pre_filter in &self.pre_filters {
            match pre_filter {
                HibouPreFilter::MaxProcessDepth( depth ) => {
                    if parent_node_path.len() > *depth {
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