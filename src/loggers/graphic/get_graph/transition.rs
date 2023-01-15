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
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::position::position::Position;
use crate::core::transformation::transfokind::InteractionTransformationKind;
use crate::io::output::draw_transitions::draw_firing::{draw_firing_analysis, draw_firing_simple};
use crate::io::output::draw_transitions::draw_hiding::draw_hiding;
use crate::io::output::draw_transitions::draw_string_label::draw_string_label;
use crate::io::output::draw_transitions::draw_transformation::draw_transformation;
use crate::io::output::graphviz::colors::GraphvizColor;
use crate::io::output::graphviz::node::node::GraphVizNode;
use crate::io::output::graphviz::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use crate::process::ana_proc::interface::step::SimulationStepKind;



pub fn make_graphic_logger_string_label(temp_folder : &String,
                                  state_id : u32,
                                  string_label : String) -> GraphVizNode {
    let sl_image_file_name = format!("sl{}", state_id);
    let sl_image_file_path : PathBuf = [temp_folder, &format!("{}.png",sl_image_file_name)].iter().collect();
    // ***
    draw_string_label(sl_image_file_path.as_path(),string_label);
    // ***
    let mut gv_node_options : GraphvizNodeStyle = Vec::new();
    gv_node_options.push( GraphvizNodeStyleItem::Image( sl_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
    gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
    gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
    // ***
    return GraphVizNode{id:sl_image_file_name,style:gv_node_options};
}





pub fn make_graphic_logger_transformation(temp_folder : &String,
                                          state_id : u32,
                                          transfo_kind : &InteractionTransformationKind,
                                          position : &Position) -> GraphVizNode {
    let tr_image_file_name = format!("t{}", state_id);
    let tr_image_file_path : PathBuf = [temp_folder, &format!("{}.png",tr_image_file_name)].iter().collect();
    // ***
    draw_transformation(tr_image_file_path.as_path(),transfo_kind,position);
    // ***
    let mut gv_node_options : GraphvizNodeStyle = Vec::new();
    gv_node_options.push( GraphvizNodeStyleItem::Image( tr_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
    gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
    gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
    // ***
    return GraphVizNode{id:tr_image_file_name,style:gv_node_options};
}




pub fn make_graphic_logger_hiding(temp_folder : &String,
                                  gen_ctx : &GeneralContext,
                                  state_id : u32,
                                  lfs_to_hide: &HashSet<usize>) -> GraphVizNode {
    let hid_image_file_name = format!("h{}", state_id);
    let hid_image_file_path : PathBuf = [temp_folder, &format!("{}.png",hid_image_file_name)].iter().collect();
    // ***
    draw_hiding(hid_image_file_path.as_path(),gen_ctx,lfs_to_hide);
    // ***
    let mut gv_node_options : GraphvizNodeStyle = Vec::new();
    gv_node_options.push( GraphvizNodeStyleItem::Image( hid_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
    gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
    gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
    // ***
    return GraphVizNode{id:hid_image_file_name,style:gv_node_options};
}



pub fn make_graphic_logger_firing(temp_folder : &String,
                                  gen_ctx : &GeneralContext,
                                  state_id : u32,
                                  action_position : &Position,
                                  executed_actions : &HashSet<TraceAction>,
                                  ana : Option<(&CoLocalizations, &HashSet<usize>, &HashMap<usize,SimulationStepKind>)>) -> GraphVizNode {
    let fir_image_file_name = format!("f{}", state_id);
    let fir_image_file_path : PathBuf = [temp_folder, &format!("{}.png",fir_image_file_name)].iter().collect();
    // ***
    match ana {
        None => {
            draw_firing_simple(fir_image_file_path.as_path(),
                               gen_ctx,
                               action_position,
                               executed_actions);
        },
        Some((colocs, consu_set,sim_map)) => {
            draw_firing_analysis(fir_image_file_path.as_path(),
                                 gen_ctx,
                                 action_position,
                                 executed_actions,
                                 colocs,
                                 consu_set,
                                 sim_map);
        }
    }
    // ***
    let mut gv_node_options : GraphvizNodeStyle = Vec::new();
    gv_node_options.push( GraphvizNodeStyleItem::Image( fir_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
    gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
    gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
    gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
    // ***
    return GraphVizNode{id:fir_image_file_name,style:gv_node_options};
}

