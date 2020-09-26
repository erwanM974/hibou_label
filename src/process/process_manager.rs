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
use crate::process::verdicts::CoverageVerdict;
use crate::process::hibou_process::*;


pub struct HibouProcessManager {
    pub gen_ctx : GeneralContext,
    pub strategy : HibouSearchStrategy,
    pub pre_filters : Vec<HibouPreFilter>,
    pub loggers : Vec<Box<dyn ProcessLogger>>
}

impl HibouProcessManager {
    pub fn get_options_as_strings(&self) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        options_str.push( format!("strategy={}", &self.strategy.to_string()) );
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
}

pub fn make_matches(interaction : &Interaction, multitrace : &AnalysableMultiTrace) -> Vec<Position> {
    let mut filtered_front : Vec<Position> = Vec::new();
    for front_pos in make_frontier(interaction) {
        let front_act = interaction.get_sub_interaction(&front_pos).as_leaf();
        for canal in multitrace {
            if canal.trace.len() > 0 {
                let head_act : &TraceAction = canal.trace.get(0).unwrap();
                if head_act.is_match(front_act) {
                    filtered_front.push(front_pos);
                    break;
                }
            }
        }
    }
    return filtered_front;
}

impl HibouProcessManager {
    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               pre_filters : Vec<HibouPreFilter>,
               loggers : Vec<Box<dyn ProcessLogger>>
    ) -> HibouProcessManager {
        return HibouProcessManager{gen_ctx,strategy,pre_filters,loggers};
    }


    pub fn init_loggers(&mut self, interaction : &Interaction,remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        let options_as_strs = (&self).get_options_as_strings();
        for logger in self.loggers.iter_mut() {
            (*logger).log_init(interaction, &self.gen_ctx, &options_as_strs,remaining_multi_trace);
        }
    }

    pub fn term_loggers(&mut self) {
        for logger in self.loggers.iter_mut() {
            (*logger).log_term();
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

    pub fn extract_from_queue(&mut self,
                              state_nodes_queue : &mut Vec<ProcessStateNode>) -> ProcessStateNode {
        // ***
        let state_node : ProcessStateNode;
        match self.strategy {
            HibouSearchStrategy::BFS => {
                return state_nodes_queue.remove(0);

            },
            HibouSearchStrategy::DFS => {
                return state_nodes_queue.remove(state_nodes_queue.len() - 1);
            }
        }
    }

    pub fn put_back_in_queue(&mut self, state_nodes_queue : &mut Vec<ProcessStateNode>, to_put_back : ProcessStateNode, added_in_queue : bool) {
        match self.strategy {
            HibouSearchStrategy::BFS => {
                state_nodes_queue.insert(0,to_put_back);

            },
            HibouSearchStrategy::DFS => {
                if added_in_queue {
                    state_nodes_queue.insert(state_nodes_queue.len() - 1, to_put_back);
                } else {
                    state_nodes_queue.push(to_put_back)
                }
            }
        }
    }

    pub fn process_next(&mut self,
                        state_node : ProcessStateNode,
                        process_node_count : u32) -> ProcessStepResult {
        // ***
        let put_back_state_node : Option<ProcessStateNode>;
        let new_state_node      : Option<ProcessStateNode>;
        // ***
        // ***
        let mut new_rem_front_or_match = state_node.rem_front_or_match;
        let next_front_pos = new_rem_front_or_match.remove(0);
        // ***
        let parent_node_path = state_node.state_id;
        // ***
        let mut next_node_path = parent_node_path.clone();
        next_node_path.push( state_node.id_for_next_child );
        // ***
        let parent_loop_depth = state_node.previous_loop_instanciations;
        let new_loop_depth = parent_loop_depth + state_node.interaction.get_loop_depth_at_pos(&next_front_pos);
        // ***
        match self.apply_pre_filters(&parent_node_path,new_loop_depth,process_node_count) {
            None => {
                let target_action = state_node.interaction.get_sub_interaction(&next_front_pos).as_leaf();
                let ex_lf_id = target_action.occupation_before();
                let new_interaction = execute(state_node.interaction.clone(),next_front_pos.clone(),ex_lf_id);
                // ***
                let new_front_or_match: Vec<Position>;
                let new_rem_multitrace : Option<AnalysableMultiTrace>;
                match (state_node.multitrace.clone()) {
                    None => {
                        new_front_or_match = make_frontier(&new_interaction);
                        new_rem_multitrace = None;
                    },
                    Some( multitrace ) => {
                        let mut new_mu : AnalysableMultiTrace = Vec::new();
                        for canal in multitrace {
                            if canal.lifelines.contains(&ex_lf_id) {
                                let mut new_trace = canal.trace;
                                assert!(new_trace.len() > 0);
                                new_trace.remove(0);
                                new_mu.push( MultiTraceCanal{lifelines:canal.lifelines,trace:new_trace} )
                            } else {
                                new_mu.push(canal);
                            }
                        }
                        // ***
                        new_front_or_match = make_matches(&new_interaction,&new_mu);
                        // ***
                        new_rem_multitrace = Some(new_mu);
                    }
                }
                // ***
                self.next_loggers(&next_front_pos,
                                  target_action,
                                  &new_interaction,
                                  &parent_node_path,
                                  &next_node_path,
                                  &new_rem_multitrace);
                new_state_node = Some(ProcessStateNode::new(next_node_path,
                                                            new_interaction,
                                                            new_front_or_match,
                                                            1,
                                                            new_rem_multitrace,
                                                            new_loop_depth));
            },
            Some( elim_kind ) => {
                let target_action = state_node.interaction.get_sub_interaction(&next_front_pos).as_leaf();
                self.filtered_loggers(&next_front_pos,target_action,&parent_node_path,&next_node_path, &elim_kind);
                new_state_node = None;
            }
        }
        if new_rem_front_or_match.len() > 0 {
            put_back_state_node = Some(ProcessStateNode::new(parent_node_path,
                                                             state_node.interaction,
                                                             new_rem_front_or_match,
                                                             (state_node.id_for_next_child + 1),
                                                             state_node.multitrace,
                                                             parent_loop_depth));
        } else {
            put_back_state_node = None;
        }
        return ProcessStepResult{put_back_state_node,new_state_node};
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



}