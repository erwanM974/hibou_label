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
use std::path::PathBuf;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::position::position::Position;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::graphviz::colors::GraphvizColor;
use crate::io::output::graphviz::edge::edge::GraphVizEdge;
use crate::io::output::graphviz::edge::style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle, GvEdgeLineStyle};
use crate::io::output::graphviz::node::node::GraphVizNode;
use crate::io::output::graphviz::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use crate::loggers::graphic::conf::GraphicProcessLoggerOutputFormat;
use crate::loggers::graphic::get_graph::filter::make_graphic_logger_filter;
use crate::loggers::graphic::get_graph::state::make_graphic_logger_state;
use crate::loggers::graphic::get_graph::transition::{make_graphic_logger_firing, make_graphic_logger_hiding};
use crate::loggers::graphic::get_graph::verdict::make_graphic_logger_verdict;
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::ana_proc::interface::logger::AnalysisLogger;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::process::ana_proc::logic::anakind::AnalysisKind;
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;
use crate::process::ana_proc::logic::local_analysis::perform_local_analysis;
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

    fn log_out_on_local_analysis(&mut self,
                                 gen_ctx : &GeneralContext,
                                 parent_state_id: u32,
                                 verdict: &CoverageVerdict,
                                 parent_analysis_kind: &AnalysisKind,
                                 local_coloc: &CoLocalizations,
                                 local_interaction: &Interaction,
                                 local_multi_trace: &MultiTrace,
                                 local_flags: &MultiTraceAnalysisFlags) {
        if self.display_subprocesses {
            // ***
            let subproc_image_file_name = format!("ana{}", parent_state_id);
            let sub_graphic_logger = GraphicProcessLogger::new(GraphicProcessLoggerOutputFormat::png,
                                                               self.layout.clone(),
                                                               false,
                                                               false,
                                                               self.int_repr_sd,
                                                               self.int_repr_tt,
                                                               "./temp".to_string(),
                                                               self.temp_folder.clone(),
                                                               subproc_image_file_name.clone());
            perform_local_analysis(gen_ctx,
                                   local_coloc.clone(),
                                   parent_analysis_kind,
                                   local_interaction.clone(),
                                   local_multi_trace.clone(),
                                   local_flags.clone(),
                                   vec![Box::new(sub_graphic_logger)]);
            // ***
            let subproc_image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",subproc_image_file_name)].iter().collect();
            // ***
            let verdict_color = verdict.get_verdict_color();
            // ***
            let locana_node : GraphVizNode;
            {
                let mut gv_node_options : GraphvizNodeStyle = Vec::new();
                gv_node_options.push( GraphvizNodeStyleItem::Image( subproc_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
                gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
                gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
                gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
                gv_node_options.push( GraphvizNodeStyleItem::PenWidth(3) );
                gv_node_options.push(GraphvizNodeStyleItem::Color( verdict_color.clone() ));
                // ***
                locana_node = GraphVizNode{id:subproc_image_file_name,style:gv_node_options};
            }
            // ***
            let locana_edge : GraphVizEdge;
            {
                let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
                // ***
                tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
                tran_gv_options.push( GraphvizEdgeStyleItem::Color( verdict_color ) );
                tran_gv_options.push( GraphvizEdgeStyleItem::LTail( format!("cluster_n{}",parent_state_id) ) );
                // ***
                locana_edge = GraphVizEdge::new(format!("a{:}", parent_state_id),locana_node.id.clone(),tran_gv_options);
            }
            // ***
            self.graph.nodes.push(Box::new(locana_node));
            self.graph.edges.push(locana_edge);
            // ***
        } else {
            self.log_verdict(parent_state_id,verdict);
        }
    }
}