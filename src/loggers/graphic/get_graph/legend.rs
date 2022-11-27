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


use crate::io::output::graphviz::node::node::GraphVizNode;
use crate::io::output::graphviz::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};


pub fn make_graphic_logger_legend(options_as_strs : &Vec<String>) -> GraphVizNode {
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
    return GraphVizNode{id:"legend".to_string(), style:legend_node_gv_options};
}

