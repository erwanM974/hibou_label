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


use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};


use crate::process::abstract_proc::common::FilterEliminationKind;


pub fn make_graphic_logger_filter(parent_state_id : u32,
                                  new_state_id : u32,
                                  elim_kind : &FilterEliminationKind) -> (GraphVizNode,GraphVizEdge) {
    // ***
    let elim_node : GraphVizNode;
    {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        // ***
        node_gv_options.push( GraphvizNodeStyleItem::Label( elim_kind.to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Color( GraphvizColor::burlywood4 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Pentagon) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );
        // ***
        elim_node = GraphVizNode{id:format!("e{:}", new_state_id),style:node_gv_options};
    }
    // ***
    let elim_edge : GraphVizEdge;
    {
        let tran_gv_options = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ),
                                    GraphvizEdgeStyleItem::Color( GraphvizColor::burlywood4 ) ];
        // ***
        elim_edge = GraphVizEdge::new(format!("a{:}", parent_state_id),
                                      Some(format!("n{}",parent_state_id)),
                                      elim_node.id.clone(),
                                      None,
                                      tran_gv_options);
    }
    return (elim_node,elim_edge);
}