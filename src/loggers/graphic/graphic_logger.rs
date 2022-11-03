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
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::position::position::Position;
use crate::core::language::syntax::interaction::Interaction;

use crate::loggers::graphic::conf::{GraphicProcessLoggerInteractionRepresentation, GraphicProcessLoggerLayout, GraphicProcessLoggerOutputKind};

use crate::output::rendering::custom_draw::multitrace::draw_mu::draw_multitrace;
use crate::output::rendering::custom_draw::seqdiag::interaction::draw_interaction;
use crate::output::rendering::custom_draw::transition::draw_firing::{draw_firing_simple,draw_firing_analysis};
use crate::output::rendering::custom_draw::transition::draw_hiding::draw_hiding;
use crate::output::rendering::graphviz::common::*;
use crate::output::rendering::graphviz::edge_style::*;
use crate::output::rendering::graphviz::graph::*;
use crate::output::rendering::graphviz::node_style::*;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;

// ***

pub struct GraphicProcessLogger {
    log_name : String,
    file : File,
    output_kind : GraphicProcessLoggerOutputKind,
    layout : GraphicProcessLoggerLayout,
    interaction_repr : GraphicProcessLoggerInteractionRepresentation
}

impl GraphicProcessLogger {

    pub fn new(log_name : String,
               output_kind : GraphicProcessLoggerOutputKind,
               layout : GraphicProcessLoggerLayout,
               interaction_repr : GraphicProcessLoggerInteractionRepresentation) -> GraphicProcessLogger {
        let file = File::create(&format!("{:}.dot",log_name)).unwrap();
        // ***
        return GraphicProcessLogger{
            log_name,
            file,
            output_kind,
            layout,
            interaction_repr}
    }

    pub fn initiate(&mut self) {
        // empties temp directory if exists
        match fs::remove_dir_all("./temp") {
            Ok(_) => {
                // do nothing
            },
            Err(e) => {
                // do nothing
            }
        }
        // creates temp directory
        fs::create_dir_all("./temp").unwrap();
        // ***
        self.file.write("digraph G {\n".as_bytes() );
        self.file.write("compound=true;\n".as_bytes() );
        // ***
        match self.layout {
            GraphicProcessLoggerLayout::horizontal => {
                self.file.write("rankdir=LR;\n".as_bytes() );
            },
            GraphicProcessLoggerLayout::vertical => {
                self.file.write("rankdir=TB;\n".as_bytes() );
            }
        }
        // ***
    }

    pub fn write_node(&mut self,
                      node_name : String,
                      style : GraphvizNodeStyle) {
        let gv_node = GraphVizNode{id : node_name, style : style};
        self.file.write( gv_node.to_dot_string().as_bytes() );
        self.file.write("\n".as_bytes() );
    }

    pub fn write_edge(&mut self,
                      origin_id : String,
                      target_id : String,
                      style : GraphvizEdgeStyle) {
        let gv_edge = GraphVizEdge{origin_id, target_id, style};
        self.file.write( gv_edge.to_dot_string().as_bytes() );
        self.file.write("\n".as_bytes() );
    }

    pub fn terminate(&mut self,
                options_as_strs : &Vec<String>) {

        // *** LEGEND
        {
            let mut legend_str = String::new();
            for opt_str in options_as_strs {
                legend_str.push_str(opt_str);
                legend_str.push_str("\\l");
            }
            // ***
            let mut legend_node_gv_options : GraphvizNodeStyle = Vec::new();
            legend_node_gv_options.push( GraphvizNodeStyleItem::Label( legend_str ) );
            legend_node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            legend_node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Bold,GvNodeStyleKind::Rounded]) );
            legend_node_gv_options.push( GraphvizNodeStyleItem::FontSize( 18 ) );
            // ***
            self.write_node( "legend".to_string(), legend_node_gv_options );
        }
        // ***
        self.file.write( "}".as_bytes() );
        // ***
        match self.output_kind {
            GraphicProcessLoggerOutputKind::png => {
                let status = Command::new("dot")
                    .arg("-Tpng")
                    .arg(&format!("{:}.dot",self.log_name))
                    .arg("-o")
                    .arg(&format!("{:}.png",self.log_name))
                    .output();
            },
            GraphicProcessLoggerOutputKind::svg => {
                let status = Command::new("dot")
                    .arg("-Tsvg:cairo")
                    .arg(&format!("{:}.dot",self.log_name))
                    .arg("-o")
                    .arg(&format!("{:}.svg",self.log_name))
                    .output();
            }
        }
    }

    pub fn write_multitrace(&mut self,
                        gen_ctx : &GeneralContext,
                            co_localizations : &CoLocalizations,
                        new_state_id : u32,
                        multi_trace : &MultiTrace,
                        flags : &MultiTraceAnalysisFlags,
                        is_simulation : bool,
                        sim_crit_loop : bool,
                        sim_crit_act : bool) {

        // ***
        let mu_img_path : String = format!("./temp/{:}_m{}.png",  self.log_name ,new_state_id);
        draw_multitrace(gen_ctx,
                        co_localizations,
                        &mu_img_path,
                        multi_trace,
                        flags,
                        is_simulation,
                        sim_crit_loop,
                        sim_crit_act);
        // ***
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        node_gv_options.push( GraphvizNodeStyleItem::Image( mu_img_path ) );
        node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        let current_node_name = format!("m{:}", new_state_id);
        self.write_node(current_node_name, node_gv_options);
    }

    pub fn write_interaction(&mut self,
                               gen_ctx : &GeneralContext,
                               new_state_id : u32,
                               new_interaction : &Interaction) {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        if new_interaction != &Interaction::Empty {
            let int_img_path : String = format!("./temp/{:}_i{}.png",  self.log_name ,new_state_id);
            draw_interaction(gen_ctx,&int_img_path, new_interaction);
            node_gv_options.push( GraphvizNodeStyleItem::Image( int_img_path ) );
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        } else {
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            node_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        }
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        let current_node_name = format!("i{:}", new_state_id);
        self.write_node(current_node_name, node_gv_options);
    }

    pub fn write_firing_simple(&mut self,
                        gen_ctx : &GeneralContext,
                        new_state_id : u32,
                        action_position : &Position,
                        executed_actions : &HashSet<TraceAction>,) {
        let firing_node_path : String = format!("./temp/{:}_f{}.png",  self.log_name ,new_state_id);
        draw_firing_simple(&firing_node_path,
                    gen_ctx,
                    action_position,
                    executed_actions);
        // ***
        let mut firing_gv_node_options : GraphvizNodeStyle = Vec::new();
        firing_gv_node_options.push( GraphvizNodeStyleItem::Image( firing_node_path ) );
        firing_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        firing_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        let firing_node_name = format!("f{:}", new_state_id);
        self.write_node( firing_node_name.clone(), firing_gv_node_options);
    }

    pub fn write_firing_analysis(&mut self,
                        gen_ctx : &GeneralContext,
                        co_localizations : &CoLocalizations,
                        new_state_id : u32,
                        action_position : &Position,
                        executed_actions : &HashSet<TraceAction>,
                        consu_set : &HashSet<usize>,
                        sim_map : &HashMap<usize,SimulationStepKind>) {
        let firing_node_path : String = format!("./temp/{:}_f{}.png",  self.log_name ,new_state_id);
        draw_firing_analysis(&firing_node_path,
                    gen_ctx,
                    co_localizations,
                    action_position,
                    executed_actions,
                    consu_set,
                    sim_map);
        // ***
        let mut firing_gv_node_options : GraphvizNodeStyle = Vec::new();
        firing_gv_node_options.push( GraphvizNodeStyleItem::Image( firing_node_path ) );
        firing_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        firing_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        let firing_node_name = format!("f{:}", new_state_id);
        self.write_node( firing_node_name.clone(), firing_gv_node_options);
    }

    pub fn write_hiding(&mut self,
                        gen_ctx : &GeneralContext,
                        new_state_id : u32,
                        lfs_to_hide: &HashSet<usize>) {
        let hiding_node_path : String = format!("./temp/{:}_h{}.png",  self.log_name ,new_state_id);
        draw_hiding(&hiding_node_path,gen_ctx,lfs_to_hide);
        // ***
        let mut hiding_gv_node_options : GraphvizNodeStyle = Vec::new();
        hiding_gv_node_options.push( GraphvizNodeStyleItem::Image( hiding_node_path ) );
        hiding_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        hiding_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        let hiding_node_name = format!("h{:}", new_state_id);
        self.write_node(hiding_node_name, hiding_gv_node_options);
    }

    pub fn write_filtered(&mut self,
                    parent_state_id : u32,
                    new_state_id : u32,
                    elim_kind : &FilterEliminationKind) {
        // *** Node names
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        let elim_node_name = format!("e{:}", new_state_id);
        // ***
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        // *****
        node_gv_options.push( GraphvizNodeStyleItem::Label( elim_kind.to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Color( GraphvizColor::burlywood4 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Pentagon) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );
        // ***
        self.write_node( elim_node_name.clone(), node_gv_options);
        // ***
        let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
        tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
        tran_gv_options.push( GraphvizEdgeStyleItem::Color( GraphvizColor::burlywood4 ) );
        // ***
        self.write_edge( parent_interaction_node_name, elim_node_name, tran_gv_options);
    }


    pub fn write_analysis_node(&mut self,
                               gen_ctx : &GeneralContext,
                               co_localizations : &CoLocalizations,
                               new_state_id : u32,
                               new_interaction : &Interaction,
                               multi_trace : &MultiTrace,
                               flags : &MultiTraceAnalysisFlags,
                               is_simulation : bool,
                               sim_crit_loop : bool,
                               sim_crit_act : bool) {
        self.file.write(format!("subgraph cluster_i{} {{\n",new_state_id).as_bytes() );
        self.file.write("style=filled;color=lightgrey;\n".as_bytes() );
        // first node
        self.write_multitrace(gen_ctx,
                              co_localizations,
                              new_state_id,
                              multi_trace,
                              flags,
                              is_simulation,
                              sim_crit_loop,
                              sim_crit_act);
        self.write_interaction(gen_ctx,new_state_id,new_interaction);
        //
        self.file.write("}\n".as_bytes() );
    }

}


