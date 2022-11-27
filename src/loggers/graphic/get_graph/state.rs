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




use std::path::PathBuf;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::io::output::draw_traces::interface::draw_multitrace;
use crate::io::output::graphviz::cluster::cluster::GraphVizCluster;
use crate::io::output::graphviz::colors::{DotTranslatable, GraphvizColor};
use crate::io::output::graphviz::node::node::GraphVizNode;
use crate::io::output::graphviz::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyle, GvNodeStyleKind};
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;


fn make_anchor_node(state_id : u32) -> GraphVizNode {
    return GraphVizNode{id:format!("a{:}", state_id),
        style:vec![GraphvizNodeStyleItem::Label("".to_string()),
                   GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis]),
                   GraphvizNodeStyleItem::Peripheries(0),
                   GraphvizNodeStyleItem::Height(0),GraphvizNodeStyleItem::Width(0)
        ]};
}

pub fn make_graphic_logger_state(temp_folder : &String,
                                gen_ctx : &GeneralContext,
                                state_id : u32,
                                interaction : &Interaction,
                                as_sd : bool,
                                as_term : bool,
                                with_mu : Option<(&CoLocalizations,&MultiTrace,&MultiTraceAnalysisFlags,bool,bool,bool)>) -> GraphVizCluster {
    let mut nodes : Vec<Box<dyn DotTranslatable>> = vec![];
    // ***
    let mut anchored = false;
    let mut got = 0;
    if let Some((coloc,mu,flags,is_sim,crit_loop,crit_act)) = with_mu {
        got += 1;
        let node = make_graphic_logger_mu(temp_folder,gen_ctx,state_id,coloc,mu,flags,is_sim,crit_loop,crit_act);
        nodes.push(Box::new(node));
    }
    if got > 0 {
        let node = make_anchor_node(state_id);
        nodes.push(Box::new(node));
        anchored = true;
    }
    // ***
    if as_sd {
        got += 1;
        let node = make_graphic_logger_sd(temp_folder,gen_ctx,state_id,interaction);
        nodes.push(Box::new(node));
    }
    if !anchored && got > 0 {
        let node = make_anchor_node(state_id);
        nodes.push(Box::new(node));
        anchored = true;
    }
    if as_term {
        let node = make_graphic_logger_tt(temp_folder,gen_ctx,state_id,interaction);
        nodes.push(Box::new(node));
    }
    if !anchored {
        let node = make_anchor_node(state_id);
        nodes.push(Box::new(node));
    }
    // ***
    let node_id = format!("n{:}", state_id);
    // ***
    let mut cluster_gv_options : GraphvizNodeStyle = Vec::new();
    cluster_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::lightgrey ));
    cluster_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    // ***
    return GraphVizCluster::new(node_id,cluster_gv_options,nodes,vec![]);
}



fn make_graphic_logger_sd(temp_folder : &String,
                          gen_ctx : &GeneralContext,
                          state_id : u32,
                          interaction : &Interaction) -> GraphVizNode {
    let mut node_gv_options : GraphvizNodeStyle = Vec::new();
    let int_image_file_name = format!("isd{}", state_id);
    if interaction != &Interaction::Empty {
        draw_interaction(gen_ctx,
                         interaction,
                         &InteractionGraphicalRepresentation::AsSequenceDiagram,
                         &"temp".to_string() ,
                         temp_folder,
                         &int_image_file_name);
        // ***
        let int_image_file_path : PathBuf = [temp_folder, &format!("{}.png",int_image_file_name)].iter().collect();
        // ***
        node_gv_options.push( GraphvizNodeStyleItem::Image( int_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    } else {
        node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        node_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
    }
    node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
    // ***
    return GraphVizNode{id : int_image_file_name, style:node_gv_options};
}



fn make_graphic_logger_tt(temp_folder : &String,
                          gen_ctx : &GeneralContext,
                          state_id : u32,
                          interaction : &Interaction) -> GraphVizNode {
    let mut node_gv_options : GraphvizNodeStyle = Vec::new();
    let int_image_file_name = format!("itt{}", state_id);
    if interaction != &Interaction::Empty {
        draw_interaction(gen_ctx,
                         interaction,
                         &InteractionGraphicalRepresentation::AsTerm,
                         &"temp".to_string() ,
                         temp_folder,
                         &int_image_file_name);
        // ***
        let int_image_file_path : PathBuf = [temp_folder, &format!("{}.png",int_image_file_name)].iter().collect();
        // ***
        node_gv_options.push( GraphvizNodeStyleItem::Image( int_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    } else {
        node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        node_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
    }
    node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
    // ***
    return GraphVizNode{id : int_image_file_name, style:node_gv_options};
}


fn make_graphic_logger_mu(temp_folder : &String,
                          gen_ctx : &GeneralContext,
                          state_id : u32,
                          co_localizations : &CoLocalizations,
                          multi_trace : &MultiTrace,
                          flags : &MultiTraceAnalysisFlags,
                          is_simulation : bool,
                          sim_crit_loop : bool,
                          sim_crit_act : bool) -> GraphVizNode {
    let mu_image_file_name = format!("mu{}.png", state_id);
    // ***
    draw_multitrace(gen_ctx,co_localizations,multi_trace,flags,is_simulation,sim_crit_loop,sim_crit_act,temp_folder,&mu_image_file_name);
    // ***
    let mut node_gv_options : GraphvizNodeStyle = Vec::new();
    let mu_image_file_path : PathBuf = [temp_folder, &mu_image_file_name].iter().collect();
    node_gv_options.push( GraphvizNodeStyleItem::Image( mu_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
    node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
    // ***
    return GraphVizNode{id : format!("mu{:}", state_id), style:node_gv_options};
}