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

use std::fs;
use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};

use std::collections::HashMap;

use std::process::Command;

use crate::rendering::graphviz::graph::*;
use crate::rendering::graphviz::node_style::*;
use crate::rendering::graphviz::edge_style::*;
use crate::rendering::graphviz::common::*;


use crate::core::syntax::interaction::{Interaction,ScheduleOperatorKind};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::general_context::GeneralContext;

use crate::canonize::term_repr_out::to_term_repr_temp;
use crate::canonize::measure::get_interaction_measure;
use crate::canonize::transformations::*;

pub fn canon_process_interaction_term(interaction : &Interaction, gen_ctx : &GeneralContext, name : &String) {
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
    let mut file = File::create(&format!("{:}.dot",name)).unwrap();
    file.write( format!("digraph {} {{\n", name).as_bytes() );
    file.write( "overlap=false;\n".as_bytes() );
    canonize_process(&mut file, interaction, gen_ctx);
    file.write( "}\n".as_bytes() );
    let status = Command::new("neato")
        .arg("-Tsvg:cairo")
        .arg(&format!("{:}.dot",name))
        .arg("-o")
        .arg(&format!("{:}.svg",name))
        .output();
}

fn canonize_process(file : &mut File, init_interaction : &Interaction, gen_ctx : &GeneralContext) {
    // ***
    // source node
    {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );
        node_gv_options.push( GraphvizNodeStyleItem::Label("".to_string()) );
        let gv_node = GraphVizNode{id : "i0".to_string(), style : node_gv_options};
        file.write( gv_node.to_dot_string().as_bytes() );
        file.write("\n".as_bytes() );
    }
    // ***
    // first node
    {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        node_gv_options.push( GraphvizNodeStyleItem::Image( "temp/i1.png".to_string() ) );
        node_gv_options.push( GraphvizNodeStyleItem::Label( "".to_string() ) );
        let gv_node = GraphVizNode{id : "i1".to_string(), style : node_gv_options};
        file.write( gv_node.to_dot_string().as_bytes() );
        file.write("\n".as_bytes() );
    }
    // ***
    // transition from source
    {
        to_term_repr_temp(&"i1".to_string(),init_interaction,gen_ctx);
        let gv_edge = GraphVizEdge{origin_id : "i0".to_string(), target_id : "i1".to_string(), style : Vec::new()};
        file.write( gv_edge.to_dot_string().as_bytes() );
        file.write("\n".as_bytes() );
    }
    // ***
    // main loop
    let mut queue : Vec<(u32,Interaction)> = vec![(1,init_interaction.clone())];
    let mut next_index : u32 = 2;
    let mut known : HashMap<Vec<u32>,u32> = HashMap::new();
    known.insert( get_interaction_measure(init_interaction,gen_ctx), 1 );
    // ***
    while queue.len() > 0 {
        let (parent_id,parent_interaction) = queue.pop().unwrap();
        let parent_id_str = format!("i{}", parent_id);
        for transformed in get_all_transformations_rec(&parent_interaction) {
            let new_measure = get_interaction_measure(&transformed.result,gen_ctx);
            if known.contains_key(&new_measure) {
                let target_id = known.get(&new_measure).unwrap();
                // new transition
                let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
                tran_gv_options.push( GraphvizEdgeStyleItem::Label( transformed.transformation_str_description() ));
                let gv_edge = GraphVizEdge{origin_id : parent_id_str.clone(),
                    target_id : format!("i{}",&target_id),
                    style : tran_gv_options};
                file.write( gv_edge.to_dot_string().as_bytes() );
                file.write("\n".as_bytes() );
                // then discard the new interaction
            } else {
                // ***
                let new_id_str = format!("i{}", next_index);
                // new interaction node
                {
                    to_term_repr_temp(&new_id_str,&transformed.result,gen_ctx);
                    let mut node_gv_options : GraphvizNodeStyle = Vec::new();
                    node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
                    node_gv_options.push( GraphvizNodeStyleItem::Image( format!("temp/{}.png",new_id_str) ) );
                    node_gv_options.push( GraphvizNodeStyleItem::Label( "".to_string() ) );
                    let gv_node = GraphVizNode{id : new_id_str.clone(), style : node_gv_options};
                    file.write( gv_node.to_dot_string().as_bytes() );
                    file.write("\n".as_bytes() );
                }
                // new transition
                {
                    let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
                    tran_gv_options.push( GraphvizEdgeStyleItem::Label( transformed.transformation_str_description() ));
                    let gv_edge = GraphVizEdge{origin_id : parent_id_str.clone(),
                        target_id : new_id_str,
                        style : tran_gv_options};
                    file.write( gv_edge.to_dot_string().as_bytes() );
                    file.write("\n".as_bytes() );
                }
                // save the new interaction
                known.insert( new_measure, next_index );
                queue.push((next_index,transformed.result) );
                next_index = next_index + 1;
            }
        }
    }
}










