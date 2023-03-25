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

use std::collections::HashSet;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::position::position::Position;
use crate::core::execution::trace::trace::TraceAction;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use graphviz_dot_builder::traits::DotBuildable;
use crate::loggers::graphic::get_graph::filter::make_graphic_logger_filter;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::explo_proc::interface::logger::ExplorationLogger;
use crate::loggers::graphic::get_graph::state::make_graphic_logger_state;
use crate::loggers::graphic::get_graph::transition::make_graphic_logger_firing;
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;



impl ExplorationLogger for GraphicProcessLogger {

    fn log_init(&mut self, interaction: &Interaction, gen_ctx: &GeneralContext) {
        let init_node = make_graphic_logger_state(&self.temp_folder,gen_ctx,1,interaction,self.int_repr_sd,self.int_repr_tt,None);
        self.graph.add_cluster(init_node);
    }

    fn log_term(&mut self, options_as_str: &Vec<String>) {
        self.terminate(options_as_str);
    }

    fn log_explore(&mut self,
                   gen_ctx: &GeneralContext,
                   parent_state_id: u32,
                   new_state_id: u32,
                   action_position: &Position,
                   executed_actions: &HashSet<TraceAction>,
                   new_interaction: &Interaction) {
        self.log_new_interaction(gen_ctx,new_state_id,new_interaction);
        self.log_exec_transition(gen_ctx,parent_state_id,new_state_id,new_state_id,action_position,executed_actions);
    }

    fn log_exec_transition(&mut self, gen_ctx: &GeneralContext, origin_state_id: u32, transition_state_id : u32, target_state_id: u32, action_position: &Position, executed_actions: &HashSet<TraceAction>) {
        // ***
        let state_firing = make_graphic_logger_firing(&self.temp_folder,gen_ctx,transition_state_id,action_position,executed_actions,None);
        // *** Transition To Firing
        let tran_to_firing : GraphVizEdge;
        {
            let tran_gv_options  = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) ];
            tran_to_firing = GraphVizEdge::new(format!("a{:}", origin_state_id),
                                               Some(format!("n{}",origin_state_id)),
                                               state_firing.id.clone(),
                                               None,
                                               tran_gv_options);
        }
        // *** Transition To New Node
        let tran_to_new : GraphVizEdge;
        {
            let tran_gv_options = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) ];
            tran_to_new = GraphVizEdge::new(state_firing.id.clone(),
                                            None,
                                            format!("a{:}", target_state_id),
                                            Some(format!("n{}",target_state_id)),
                                            tran_gv_options);
        }
        // ***
        self.graph.add_node(state_firing);
        self.graph.add_edge(tran_to_firing);
        self.graph.add_edge(tran_to_new);
    }

    fn log_new_interaction(&mut self, gen_ctx: &GeneralContext, new_state_id: u32, new_interaction: &Interaction) {
        // *** Resulting New Node
        let new_node = make_graphic_logger_state(&self.temp_folder,gen_ctx,new_state_id,new_interaction,self.int_repr_sd,self.int_repr_tt,None);
        self.graph.add_cluster(new_node);
    }

    fn log_filtered(&mut self, parent_state_id: u32, new_state_id: u32, elim_kind: &FilterEliminationKind) {
        let (elim_node,elim_edge) = make_graphic_logger_filter(parent_state_id,new_state_id,elim_kind);
        self.graph.add_node(elim_node);
        self.graph.add_edge(elim_edge);
    }

    fn log_notified_lastchild_explored(&mut self, gen_ctx : &GeneralContext, parent_id: u32) {
        // ***
    }

    fn log_notified_terminal_node_explored(&mut self, gen_ctx : &GeneralContext, parent_id: u32) {
        // ***
    }
}