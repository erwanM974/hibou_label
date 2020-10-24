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
use std::collections::{HashSet,HashMap};
use std::fs;
use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};

// ***
use std::process::Command;

// ***
use crate::core::general_context::GeneralContext;
use crate::core::syntax::position::*;
use crate::core::syntax::interaction::Interaction;
use crate::core::trace::AnalysableMultiTrace;
use crate::core::syntax::action::*;


use crate::process::log::ProcessLogger;
use crate::rendering::textual::monochrome::position::position_to_text;
use crate::rendering::graphviz::graph::*;
use crate::rendering::graphviz::node_style::*;
use crate::rendering::graphviz::edge_style::*;
use crate::rendering::graphviz::common::*;
use crate::rendering::custom_draw::seqdiag::interaction::draw_interaction;
use crate::rendering::custom_draw::firing::draw_firing::{draw_firing,draw_hiding};
use crate::process::verdicts::CoverageVerdict;
use crate::core::trace::{TraceAction,TraceActionKind};

use crate::process::hibou_process::FilterEliminationKind;
// ***

use crate::process::hibou_process::*;

pub enum GraphicProcessLoggerOutputKind {
    svg,
    png
}

pub enum GraphicProcessLoggerLayout {
    horizontal,
    vertical
}

pub struct GraphicProcessLogger {
    log_name : String,
    file : File,
    output_kind:GraphicProcessLoggerOutputKind,
    layout:GraphicProcessLoggerLayout
}

impl GraphicProcessLogger {
    pub fn new(log_name : String,
               output_kind : GraphicProcessLoggerOutputKind,
               layout : GraphicProcessLoggerLayout) -> GraphicProcessLogger {
        let file = File::create(&format!("{:}.dot",log_name)).unwrap();
        // ***
        return GraphicProcessLogger{
            log_name,
            file,
            output_kind,
            layout}
    }
}

impl ProcessLogger for GraphicProcessLogger {

    fn log_verdict(&mut self,
                   parent_state_id : u32,
                   verdict : &CoverageVerdict) {
        // ***
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // ***
        let verdict_node_name = format!("v{:}", parent_state_id);
        // *****
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
        tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
        // *****
        node_gv_options.push( GraphvizNodeStyleItem::Label( verdict.to_string() ) );
        // *****
        let verdict_color = verdict.get_verdict_color();
        node_gv_options.push( GraphvizNodeStyleItem::Color( verdict_color.clone() ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Diamond) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );

        tran_gv_options.push( GraphvizEdgeStyleItem::Color( verdict_color ) );
        // *****
        let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name.clone(), target_id : verdict_node_name.clone(), style : tran_gv_options};
        let gv_node = GraphVizNode{id : verdict_node_name, style : node_gv_options};
        let mut string_to_write = gv_node.to_dot_string();
        string_to_write.push_str("\n");
        string_to_write.push_str(&gv_edge.to_dot_string());
        string_to_write.push_str("\n");
        // *****
        self.file.write( string_to_write.as_bytes() );
    }

    fn log_init(&mut self,
                interaction : &Interaction,
                gen_ctx : &GeneralContext,
                remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        // ***
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
        let gv_node_path : String = format!("./temp/{:}_i1.png", self.log_name);
        draw_interaction(&gv_node_path, interaction, gen_ctx,remaining_multi_trace);

        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        node_gv_options.push( GraphvizNodeStyleItem::Label("".to_owned()) );
        node_gv_options.push( GraphvizNodeStyleItem::Image( gv_node_path ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );

        let gv_node = GraphVizNode{id : "i1".to_owned(), style : node_gv_options};
        let mut string_to_write = gv_node.to_dot_string();
        string_to_write.push_str("\n");
        self.file.write( string_to_write.as_bytes() );
    }

    fn log_term(&mut self,
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
            let legend_node = GraphVizNode{id : "legend".to_owned(), style : legend_node_gv_options};
            let legend_as_dot_str = format!("{}\n", legend_node.to_dot_string());
            self.file.write( legend_as_dot_str.as_bytes() );
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


    fn log_execution(&mut self,
                     gen_ctx : &GeneralContext,
                     parent_state_id : u32,
                     new_state_id : u32,
                     action_position : &Position,
                     action : &TraceAction,
                     new_interaction : &Interaction,
                     remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        // *** Parent Interaction Node
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // *** Firing Node
        let current_node_name = format!("i{:}", new_state_id);
        let firing_node_name = format!("f{:}", new_state_id);
        {
            let firing_node_path : String = format!("./temp/{:}_{}.png",  self.log_name ,firing_node_name);
            draw_firing(&firing_node_path,action_position,action,gen_ctx);
            // ***
            let mut firing_gv_node_options : GraphvizNodeStyle = Vec::new();
            firing_gv_node_options.push( GraphvizNodeStyleItem::Image( firing_node_path ) );
            firing_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            firing_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let firing_gv_node = GraphVizNode{id : firing_node_name.clone(), style : firing_gv_node_options};
            self.file.write( firing_gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Firing
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name, target_id : firing_node_name.clone(), style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Resulting Interaction Node
        {
            let gv_path : String = format!("./temp/{:}_{}.png",  self.log_name ,current_node_name);
            draw_interaction(&gv_path, new_interaction, gen_ctx, remaining_multi_trace);
            // ***
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Image( gv_path ) );
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let gv_node = GraphVizNode{id : current_node_name.clone(), style : node_gv_options};
            self.file.write( gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Interaction Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : firing_node_name, target_id : current_node_name, style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
    }

    fn log_hide(&mut self,
                gen_ctx : &GeneralContext,
                parent_state_id : u32,
                new_state_id : u32,
                lfs_to_hide : &HashSet<usize>,
                hidden_interaction : &Interaction,
                remaining_multi_trace : &Option<AnalysableMultiTrace>) {
        // *** Parent Interaction Node
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        // *** Hiding Node
        let current_node_name = format!("i{:}", new_state_id);
        let hiding_node_name = format!("h{:}", new_state_id);
        {
            let hiding_node_path : String = format!("./temp/{:}_{}.png",  self.log_name ,hiding_node_name);
            draw_hiding(&hiding_node_path,lfs_to_hide,gen_ctx);
            // ***
            let mut hiding_gv_node_options : GraphvizNodeStyle = Vec::new();
            hiding_gv_node_options.push( GraphvizNodeStyleItem::Image( hiding_node_path ) );
            hiding_gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            hiding_gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let hiding_gv_node = GraphVizNode{id : hiding_node_name.clone(), style : hiding_gv_node_options};
            self.file.write( hiding_gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Hiding
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name, target_id : hiding_node_name.clone(), style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Resulting Interaction Node
        {
            let gv_path : String = format!("./temp/{:}_{}.png",  self.log_name ,current_node_name);
            draw_interaction(&gv_path, hidden_interaction, gen_ctx, remaining_multi_trace);
            // ***
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Image( gv_path ) );
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
            let gv_node = GraphVizNode{id : current_node_name.clone(), style : node_gv_options};
            self.file.write( gv_node.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
        // *** Transition To Interaction Node
        {
            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
            tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
            let gv_edge = GraphVizEdge{origin_id : hiding_node_name, target_id : current_node_name, style : tran_gv_options};
            self.file.write( gv_edge.to_dot_string().as_bytes() );
            self.file.write("\n".as_bytes() );
        }
    }

    fn log_filtered(&mut self,gen_ctx : &GeneralContext,
                    parent_state_id : u32,
                    new_state_id : u32,
                    elim_kind : &FilterEliminationKind) {
        // *** Node names
        let parent_interaction_node_name = format!("i{:}", parent_state_id);
        let elim_node_name = format!("e{:}", new_state_id);
        // ***
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
        tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
        // *****
        node_gv_options.push( GraphvizNodeStyleItem::Label( elim_kind.to_string() ) );
        // *****
        node_gv_options.push( GraphvizNodeStyleItem::Color( GraphvizColor::burlywood4 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Pentagon) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );
        // *****
        tran_gv_options.push( GraphvizEdgeStyleItem::Color( GraphvizColor::burlywood4 ) );
        // *****
        let gv_edge = GraphVizEdge{origin_id : parent_interaction_node_name.clone(), target_id : elim_node_name.clone(), style : tran_gv_options};
        let gv_node = GraphVizNode{id : elim_node_name, style : node_gv_options};
        let mut string_to_write = gv_node.to_dot_string();
        string_to_write.push_str("\n");
        string_to_write.push_str(&gv_edge.to_dot_string());
        string_to_write.push_str("\n");
        // *****
        self.file.write( string_to_write.as_bytes() );
    }

}