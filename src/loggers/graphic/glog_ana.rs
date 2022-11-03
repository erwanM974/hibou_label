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
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::position::position::Position;
use crate::core::execution::trace::trace::TraceAction;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::ana_proc::interface::logger::AnalysisLogger;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::process::ana_proc::logic::verdicts::CoverageVerdict;
use crate::output::rendering::graphviz::common::GraphvizColor;
use crate::output::rendering::graphviz::edge_style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle, GvEdgeLineStyle};
use crate::output::rendering::graphviz::node_style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;


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
        self.initiate();
        self.write_analysis_node(gen_ctx,co_localizations,
                               1,
                               interaction,
                                 multi_trace,
                                 flags,
                                 is_simulation,
                                 sim_crit_loop,
                                 sim_crit_act);
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
        self.write_firing_analysis(gen_ctx,
                          co_localizations,
                          new_state_id,
                          action_position,
                          executed_actions,
                          consu_set,
                          sim_map);
        // *** Transition To Firing
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LTail( format!("cluster_i{}",parent_state_id) ) );
            self.write_edge(format!("i{:}", parent_state_id), format!("f{:}", new_state_id), tran_gv_options);
        }
        // *** Resulting Interaction Node
        self.write_analysis_node(gen_ctx, co_localizations,new_state_id, new_interaction, &multi_trace,new_flags,is_simulation,sim_crit_loop,sim_crit_act);
        // *** Transition To Interaction Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LHead( format!("cluster_i{}",new_state_id) ) );
            self.write_edge(format!("f{:}", new_state_id), format!("i{:}", new_state_id), tran_gv_options);
        }
    }

    fn log_hide(&mut self,
                gen_ctx : &GeneralContext,
                co_localizations : &CoLocalizations,
                multi_trace : &MultiTrace,
                parent_state_id : u32,
                new_state_id : u32,
                lfs_to_hide : &HashSet<usize>,
                hidden_interaction : &Interaction,
                new_flags : &MultiTraceAnalysisFlags) {
        // *** Parent Interaction Node
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // *** Hiding Node
        self.write_hiding(gen_ctx,new_state_id,lfs_to_hide);
        // *** Transition To Hiding
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LineStyle( GvEdgeLineStyle::Dashed ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LTail( format!("cluster_i{}",parent_state_id) ) );
            self.write_edge(parent_interaction_node_name, format!("h{:}", new_state_id), tran_gv_options);
        }
        // *** Resulting Interaction Node
        self.write_analysis_node(gen_ctx, co_localizations,new_state_id, hidden_interaction, multi_trace, new_flags,false,false,false );
        // *** Transition To Interaction Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LineStyle( GvEdgeLineStyle::Dashed ) );
            tran_gv_options.push( GraphvizEdgeStyleItem::LHead( format!("cluster_i{}",new_state_id) ) );
            self.write_edge( format!("h{:}", new_state_id), format!("i{:}", new_state_id), tran_gv_options);
        }
    }

    fn log_filtered(&mut self,
                    parent_state_id: u32,
                    new_state_id: u32,
                    elim_kind: &FilterEliminationKind) {
        self.write_filtered(parent_state_id,new_state_id,elim_kind);
    }

    fn log_verdict(&mut self,
                   parent_state_id: u32,
                   verdict: &CoverageVerdict) {
        // ***
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // ***
        let verdict_node_name = format!("v{:}", parent_state_id);
        // *****
        let verdict_color = verdict.get_verdict_color();
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        node_gv_options.push( GraphvizNodeStyleItem::Label( verdict.to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Color( verdict_color.clone() ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Diamond) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );
        // ***
        let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
        tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
        tran_gv_options.push( GraphvizEdgeStyleItem::LTail( format!("cluster_i{}",parent_state_id) ) );
        tran_gv_options.push( GraphvizEdgeStyleItem::Color( verdict_color ) );
        // *****
        self.write_node(verdict_node_name.clone(), node_gv_options);
        self.write_edge( parent_interaction_node_name, verdict_node_name, tran_gv_options);
    }
}