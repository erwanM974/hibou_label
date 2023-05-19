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





use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;


use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};

use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::position::position::Position;
use crate::core::transformation::transfokind::InteractionTransformationKind;
use crate::io::output::draw_transitions::draw_firing::{draw_firing_analysis, draw_firing_simple};
use crate::io::output::draw_transitions::draw_hiding::draw_hiding;
use crate::io::output::draw_transitions::draw_string_label::draw_string_label;
use crate::io::output::draw_transitions::draw_transformation::draw_transformation;
use crate::loggers::graphviz::drawer::InteractionProcessDrawer;
use crate::process::ana::step::SimulationStepKind;



impl InteractionProcessDrawer {

    pub(crate) fn make_graphic_logger_string_label(&self,
                                            string_label : String,
                                            name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        draw_string_label(image_file_path.as_path(),string_label);
        // ***
        let mut gv_node_options : GraphvizNodeStyle = Vec::new();
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }

    pub(crate) fn make_graphic_logger_transformation(&self,
                                              transfo_kind : &InteractionTransformationKind,
                                              position : &Position,
                                              name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        draw_transformation(image_file_path.as_path(),transfo_kind,position);
        // ***
        let mut gv_node_options : GraphvizNodeStyle = Vec::new();
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }

    pub(crate) fn make_graphic_logger_hiding(&self,
                                      gen_ctx : &GeneralContext,
                                      lfs_to_hide: &HashSet<usize>,
                                      name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        draw_hiding(image_file_path.as_path(),gen_ctx,lfs_to_hide);
        // ***
        let mut gv_node_options : GraphvizNodeStyle = Vec::new();
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }



    pub(crate) fn make_graphic_logger_firing(&self,
                                      gen_ctx : &GeneralContext,
                                      action_position : &Position,
                                      executed_actions : &BTreeSet<TraceAction>,
                                      ana : Option<(&CoLocalizations, &HashSet<usize>, &HashMap<usize,SimulationStepKind>)>,
                                      name : String) -> GraphVizNode {
        let image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
        // ***
        match ana {
            None => {
                draw_firing_simple(image_file_path.as_path(),
                                   gen_ctx,
                                   action_position,
                                   executed_actions);
            },
            Some((colocs, consu_set,sim_map)) => {
                draw_firing_analysis(image_file_path.as_path(),
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
        gv_node_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        gv_node_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        gv_node_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        gv_node_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name,gv_node_options)
    }
}



