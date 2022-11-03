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
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::position::position::Position;
use crate::core::execution::trace::trace::TraceAction;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::explo_proc::interface::logger::ExplorationLogger;
use crate::output::rendering::graphviz::edge_style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;

impl ExplorationLogger for GraphicProcessLogger {

    fn log_init(&mut self, interaction: &Interaction, gen_ctx: &GeneralContext) {
        self.initiate();
        self.write_interaction(gen_ctx,1, interaction);
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
        // ***
        self.write_firing_simple(gen_ctx,new_state_id,action_position,executed_actions);
        // *** Transition To Firing
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            self.write_edge(format!("i{:}", parent_state_id), format!("f{:}", new_state_id), tran_gv_options);
        }
        // *** Resulting Interaction Node
        self.write_interaction(gen_ctx, new_state_id, new_interaction);
        // *** Transition To Interaction Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            self.write_edge(format!("f{:}", new_state_id), format!("i{:}", new_state_id), tran_gv_options);
        }
    }

    fn log_filtered(&mut self, parent_state_id: u32, new_state_id: u32, elim_kind: &FilterEliminationKind) {
        self.write_filtered(parent_state_id,new_state_id,elim_kind);
    }

    fn log_notified_lastchild_explored(&mut self, gen_ctx : &GeneralContext, parent_id: u32) {
        // ***
    }

    fn log_notified_terminal_node_explored(&mut self, gen_ctx : &GeneralContext, parent_id: u32) {
        // ***
    }
}