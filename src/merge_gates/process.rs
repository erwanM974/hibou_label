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

use std::collections::{HashSet,HashMap};

use std::process::Command;

use crate::output::rendering::graphviz::graph::*;
use crate::output::rendering::graphviz::node_style::*;
use crate::output::rendering::graphviz::edge_style::*;
use crate::output::rendering::graphviz::common::*;


use crate::core::language::syntax::interaction::{Interaction};
use crate::core::language::syntax::action::*;
use crate::core::language::position::position::Position;
use crate::core::general_context::GeneralContext;

use crate::canonize::term_repr_out::to_term_repr_temp;
use crate::merge_gates::merge::get_all_merges_rec;

use crate::canonize::transformations::phases::InteractionTermTransformation;

pub fn merge_process_interaction_term(interaction : &Interaction,
                                      gen_ctx : &GeneralContext,
                                      opt_returns : bool,
                                      opt_complete : bool,
                                      opt_graphic : bool,
                                      master_name : &str) {
    // ***
    if opt_graphic {
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
        let mut file = File::create(&format!("{:}_mergeproc.dot", master_name)).unwrap();
        file.write(format!("digraph {} {{\n", master_name).as_bytes());
        file.write("overlap=false;\n".as_bytes());
        merge_process(&interaction, gen_ctx, opt_returns, opt_complete, &mut Some(&mut file));
        file.write("}\n".as_bytes());
        let status = Command::new("dot")
            .arg("-Tsvg:cairo")
            .arg(&format!("{:}_mergeproc.dot", master_name))
            .arg("-o")
            .arg(&format!("{:}_mergeproc.svg", master_name))
            .output();
    } else {
        merge_process( &interaction, gen_ctx, opt_returns, opt_complete,&mut None);
    }
}

fn merge_process(init_interaction : &Interaction,gen_ctx : &GeneralContext,opt_returns : bool, opt_complete : bool, graphic_file : &mut Option<&mut File>) {
    // ***
    // init process
    let mut queue : Vec<(u32,Interaction)> = vec![(1,init_interaction.clone())];
    let mut next_index : u32 = 2;
    // ***
    let mut known : HashMap<Interaction,u32> = HashMap::new();
    known.insert( init_interaction.clone(), 1 );
    // ***
    match graphic_file {
        None => {},
        Some( ref mut file ) => {
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
            // transition from source
            {
                let gv_edge = GraphVizEdge{origin_id : "i0".to_string(), target_id : "i1".to_string(), style : Vec::new()};
                file.write( gv_edge.to_dot_string().as_bytes() );
                file.write("\n".as_bytes() );
            }
            file.write("subgraph cluster_merge {\n".as_bytes() );
            file.write("style=filled;color=lightgoldenrod1;label=\"merging gates\";\n".as_bytes() );
            // first node
            {
                to_term_repr_temp(&"i1".to_string(),init_interaction,gen_ctx);
                let mut node_gv_options : GraphvizNodeStyle = Vec::new();
                node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
                node_gv_options.push( GraphvizNodeStyleItem::Image( "temp/i1.png".to_string() ) );
                node_gv_options.push( GraphvizNodeStyleItem::Label( "".to_string() ) );
                let gv_node = GraphVizNode{id : "i1".to_string(), style : node_gv_options};
                file.write( gv_node.to_dot_string().as_bytes() );
                file.write("\n".as_bytes() );
            }
        }
    }
    // =====================================================================================================
    // ***
    let mut finals : HashSet<(u32,Interaction)> = HashSet::new();
    while queue.len() > 0 {
        let (parent_id,parent_interaction) = queue.pop().unwrap();
        let parent_id_str = format!("i{}", parent_id);
        // ***
        let mut available_transfos = get_all_merges_rec(&parent_interaction);
        // ***
        if available_transfos.len() > 0 {
            for transformed in available_transfos {
                if known.contains_key(&transformed.result) {
                    let target_id = known.get(&transformed.result).unwrap();
                    match graphic_file {
                        None => {},
                        Some( ref mut file ) => {
                            // new transition
                            let mut tran_gv_options : GraphvizEdgeStyle = Vec::new();
                            tran_gv_options.push( GraphvizEdgeStyleItem::Label( transformed.transformation_str_description() ));
                            let gv_edge = GraphVizEdge{origin_id : parent_id_str.clone(),
                                target_id : format!("i{}",&target_id),
                                style : tran_gv_options};
                            file.write( gv_edge.to_dot_string().as_bytes() );
                            file.write("\n".as_bytes() );
                        }
                    }
                    // then discard the new interaction
                } else {
                    // ***
                    let new_id_str = format!("i{}", next_index);
                    match graphic_file {
                        None => {},
                        Some( ref mut file ) => {
                            // new interaction node
                            {
                                to_term_repr_temp(&new_id_str,&transformed.result, gen_ctx);
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
                        }
                    }
                    // save the new interaction
                    known.insert( transformed.result.clone(), next_index );
                    queue.push((next_index,transformed.result) );
                    next_index = next_index + 1;
                }
            }
        } else {
            if opt_returns {
                if opt_complete {
                    if !parent_interaction.has_gates() {
                        //TODO: create file and write
                        break;
                    }
                } else {
                    //TODO: create file and write
                    break;
                }
            }
            finals.insert( (parent_id,parent_interaction) );
        }
    }
    match graphic_file {
        None => {},
        Some( ref mut file ) => {
            file.write("}\n".as_bytes() );
        }
    }

    // ***
    // =====================================================================================================
    //assert!(finals.len() == 1);
    //let (_,canonical_interaction) = finals.remove(0);
    //return canonical_interaction;
}










