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
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::position::position::Position;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::graphviz::edge::edge::GraphVizEdge;
use crate::io::output::graphviz::edge::style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle, GvEdgeLineStyle};
use crate::loggers::graphic::get_graph::filter::make_graphic_logger_filter;
use crate::loggers::graphic::get_graph::state::make_graphic_logger_state;
use crate::loggers::graphic::get_graph::transition::{make_graphic_logger_firing, make_graphic_logger_hiding};
use crate::loggers::graphic::get_graph::verdict::make_graphic_logger_verdict;
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::ana_proc::interface::logger::AnalysisLogger;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;
use crate::process::ana_proc::logic::verdicts::CoverageVerdict;


impl AnalysisLogger for GraphicProcessLogger {

    fn log_init(&mut self,
                gen_ctx : &GeneralContext,
                co_localizations : &CoLocalizations,
                multi_trace : &MultiTrace,
                interaction : &Interaction,
                flags : &MultiTraceAnalysisFlags,
                is_simulation : bool,
                sim_crit_loop : bool,
                sim_crit_act : bool) {
        let init_node = make_graphic_logger_state(&self.temp_folder,
                                                  gen_ctx,
                                                  1,
                                                  interaction,
                                                  self.int_repr_sd,
                                                  self.int_repr_tt,
                                                  Some((co_localizations,multi_trace,flags,is_simulation,sim_crit_loop,sim_crit_act)));
        self.graph.nodes.push(Box::new(init_node));
    }

    fn log_term(&mut self,
                options_as_str: &Vec<String>) {
        self.terminate(options_as_str);
    }

    fn log_execution(&mut self,
                     gen_ctx : &GeneralContext,
                     co_localizations : &CoLocalizations,
                     multi_trace : &MultiTrace,
                     parent_state_id : u32,
                     new_state_id : u32,
                     action_position : &Position,
                     executed_actions : &HashSet<TraceAction>,
                     consu_set : &HashSet<usize>,
                     sim_map : &HashMap<usize,SimulationStepKind>,
                     new_interaction : &Interaction,
                     new_flags : &MultiTraceAnalysisFlags,
                     is_simulation : bool,
                     sim_crit_loop : bool,
                     sim_crit_act : bool) {
        // ***
        let state_firing = make_graphic_logger_firing(&self.temp_folder,
                                                      gen_ctx,
                                                      new_state_id,
                                                      action_position,
                                                      executed_actions,
                                                      Some((co_localizations,consu_set,sim_map)));
        // *** Transition To Firing
        let tran_to_firing : GraphVizEdge;
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LTail( format!("cluster_n{}",parent_state_id) ) );
            tran_to_firing = GraphVizEdge::new(format!("a{:}", parent_state_id),state_firing.id.clone(),tran_gv_options);
        }
        // *** Resulting New Node
        let new_node = make_graphic_logger_state(&self.temp_folder,
                                                 gen_ctx,
                                                 new_state_id,
                                                 new_interaction,
                                                 self.int_repr_sd,
                                                 self.int_repr_tt,
                                                 Some((co_localizations,multi_trace,new_flags,is_simulation,sim_crit_loop,sim_crit_act)));
        // *** Transition To New Node
        let tran_to_new : GraphVizEdge;
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LHead( format!("cluster_n{}",new_state_id) ) );
            tran_to_new = GraphVizEdge::new(state_firing.id.clone(),format!("a{:}", new_state_id),tran_gv_options);
        }
        // ***
        self.graph.nodes.push(Box::new(state_firing));
        self.graph.edges.push(tran_to_firing);
        self.graph.nodes.push(Box::new(new_node));
        self.graph.edges.push(tran_to_new);
    }

    fn log_hide(&mut self,
                gen_ctx : &GeneralContext,
                co_localizations : &CoLocalizations,
                multi_trace : &MultiTrace,
                parent_state_id : u32,
                new_state_id : u32,
                lfs_to_hide : &HashSet<usize>,
                hidden_interaction : &Interaction,
                new_flags : &MultiTraceAnalysisFlags,
                is_simulation : bool,
                sim_crit_loop : bool,
                sim_crit_act : bool) {
        let state_hiding = make_graphic_logger_hiding(&self.temp_folder,gen_ctx,new_state_id,lfs_to_hide);
        // *** Transition To Hiding
        let tran_to_hiding : GraphVizEdge;
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LineStyle( GvEdgeLineStyle::Dashed ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LTail( format!("cluster_n{}",parent_state_id) ) );
            tran_to_hiding = GraphVizEdge::new(format!("a{:}", parent_state_id),state_hiding.id.clone(),tran_gv_options);
        }
        // *** Resulting New Node
        let new_node = make_graphic_logger_state(&self.temp_folder,
                                                 gen_ctx,
                                                 new_state_id,
                                                 hidden_interaction,
                                                 self.int_repr_sd,
                                                 self.int_repr_tt,
                                                 Some((co_localizations,multi_trace,new_flags,is_simulation,sim_crit_loop,sim_crit_act)));
        // *** Transition To New Node
        let tran_to_new : GraphVizEdge;
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LineStyle( GvEdgeLineStyle::Dashed ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LHead( format!("cluster_n{}",new_state_id) ) );
            tran_to_new = GraphVizEdge::new(state_hiding.id.clone(),format!("a{:}", new_state_id),tran_gv_options);
        }
        // ***
        self.graph.nodes.push(Box::new(state_hiding));
        self.graph.edges.push(tran_to_hiding);
        self.graph.nodes.push(Box::new(new_node));
        self.graph.edges.push(tran_to_new);
    }

    fn log_filtered(&mut self,
                    parent_state_id: u32,
                    new_state_id: u32,
                    elim_kind: &FilterEliminationKind) {
        let (elim_node,elim_edge) = make_graphic_logger_filter(parent_state_id,new_state_id,elim_kind);
        self.graph.nodes.push(Box::new(elim_node));
        self.graph.edges.push(elim_edge);
    }

    fn log_verdict(&mut self,
                   parent_state_id: u32,
                   verdict: &CoverageVerdict) {
        let (verd_node,verd_edge) = make_graphic_logger_verdict(parent_state_id,verdict);
        self.graph.nodes.push(Box::new(verd_node));
        self.graph.edges.push(verd_edge);
    }
}