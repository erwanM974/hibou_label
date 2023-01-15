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


use crate::io::output::graphviz::colors::GraphvizColor;
use crate::io::output::graphviz::edge::edge::GraphVizEdge;
use crate::io::output::graphviz::edge::style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use crate::io::output::graphviz::node::node::GraphVizNode;
use crate::io::output::graphviz::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use crate::process::ana_proc::logic::verdicts::CoverageVerdict;


pub fn make_graphic_logger_verdict(parent_state_id: u32,
                                   verdict: &CoverageVerdict) -> (GraphVizNode,GraphVizEdge) {
    let verdict_color = verdict.get_verdict_color();
    // ***
    let verd_node : GraphVizNode;
    {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        // ***
        node_gv_options.push( GraphvizNodeStyleItem::Label( verdict.to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Color( verdict_color.clone() ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontColor( GraphvizColor::beige ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontSize( 16 ) );
        node_gv_options.push( GraphvizNodeStyleItem::FontName( "times-bold".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Diamond) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );
        // ***
        verd_node = GraphVizNode{id:format!("v{:}", parent_state_id),style:node_gv_options};
    }
    // ***
    let verd_edge : GraphVizEdge;
    {
        let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
        // ***
        tran_gv_options.push( GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) );
        tran_gv_options.push( GraphvizEdgeStyleItem::Color( verdict_color ) );
        tran_gv_options.push( GraphvizEdgeStyleItem::LTail( format!("cluster_n{}",parent_state_id) ) );
        // ***
        verd_edge = GraphVizEdge::new(format!("a{:}", parent_state_id),verd_node.id.clone(),tran_gv_options);
    }
    return (verd_node,verd_edge);
}

