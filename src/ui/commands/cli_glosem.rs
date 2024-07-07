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


use std::path::Path;
use std::time::Instant;
use autour_core::traits::letter::AutAlphabetSubstitutable;
use autour_core::traits::repr::AutGraphvizDrawable;
use autour_core::traits::transform::AutTransformable;
use autour_core::traits::translate::AutTranslatable;


use clap::ArgMatches;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable, DotPrintable, GraphVizOutputFormat};
use crate::core::execution::trace::trace::TraceAction;

use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::nfa_translation::alphabet::get_alphabet_from_gen_ctx;
use crate::nfa_translation::compositional::get_nfa_from_interaction_via_composition;
use crate::nfa_translation::get_nfa_from_logger::get_nfa_from_interaction_exploration;
use crate::process::explo::loggers::nfait::printer::ActionNFAITPrinter;


pub fn cli_glosem(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {
            let hif_file_path = matches.value_of("hif").unwrap();
            let path_object = Path::new(hif_file_path);
            let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
            match parse_hif_file(&gen_ctx,hif_file_path) {
                Err(e) => {
                    return (vec![e.to_string()],1);
                },
                Ok( int) => {
                    let int_name = format!("{}_int",file_name);
                    let orig_nfa_name = format!("{}_orig_nfa",file_name);
                    let compositional_nfa_name = format!("{}_compositional_nfa",file_name);
                    let min_dfa_name = format!("{}_mini_dfa",file_name);

                    draw_interaction(&gen_ctx,
                                     &int,
                                     &InteractionGraphicalRepresentation::AsSequenceDiagram,
                                     &".".to_string(),
                                     &".".to_string(),
                                     &int_name);

                    let max_loop_depth : u32;
                    match matches.value_of("loop_depth") {
                        None => {
                            max_loop_depth = 1;
                        },
                        Some( as_str ) => {
                            max_loop_depth = as_str.trim().parse::<u32>().unwrap();
                        }
                    }

                    let (nfa,elapsed_get_nfa) = get_nfa_from_interaction_exploration(
                        &gen_ctx,
                        &int,
                        get_alphabet_from_gen_ctx(&gen_ctx));

                    let printer = ActionNFAITPrinter::new(get_alphabet_from_gen_ctx(&gen_ctx),gen_ctx);

                    let mut ret_print = vec![];
                    // ***
                    ret_print.push( "".to_string());
                    ret_print.push( "Generating NFA from Exploration".to_string());
                    ret_print.push( format!("of interaction from file '{}'",hsf_file_path) );
                    ret_print.push( format!("with max loop depth '{}'",max_loop_depth) );
                    ret_print.push( "".to_string());
                    // ***
                    ret_print.push( format!("orig NFA : time : {:?} , num states {:?}", elapsed_get_nfa, nfa.transitions.len() ) );
                    let now = Instant::now();
                    let min_dfa = nfa.to_dfa().minimize().to_nfa();
                    let elapsed_min_nfa = now.elapsed();
                    ret_print.push( format!("min DFA : time : {:?} , num states {:?}", elapsed_min_nfa, min_dfa.transitions.len() ) );
                    // ***
                    /*
                    let (compositional_nfa,duration) = get_nfa_from_interaction_via_composition(&printer.gen_ctx,
                                                                                                &int,
                                                                                                get_alphabet_from_gen_ctx(&printer.gen_ctx));
                    ret_print.push( format!("compositional NFA : time : {:?} , num states {:?}", duration, compositional_nfa.transitions.len() ) );

                    // ***
                     */
                    let orig_nfa_as_dot = nfa.to_dot(false,&hashset!{},&printer);
                    orig_nfa_as_dot.print_dot(&[".".to_string()],
                                              &orig_nfa_name,
                                              &GraphVizOutputFormat::png);
                    /*let compo_nfa_as_dot = compositional_nfa.to_dot(false,&hashset!{},&printer);
                    compo_nfa_as_dot.print_dot(&[".".to_string()],
                                              &compositional_nfa_name,
                                              &GraphVizOutputFormat::png);*/
                    let min_dfa_as_dot = min_dfa.to_dot(false,&hashset!{},&printer);
                    min_dfa_as_dot.print_dot(&[".".to_string()],
                                              &min_dfa_name,
                                              &GraphVizOutputFormat::png);
                    // ***
                    let mut graph = GraphVizDiGraph::new(vec![]);
                    graph.add_node(
                        GraphVizNode::new("int".to_string(),
                                          vec![
                                              GraphvizNodeStyleItem::Image(format!("{}.png",int_name)),
                                              GraphvizNodeStyleItem::Label("".to_string()),
                                              GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                          ]
                        )
                    );
                    graph.add_node(
                        GraphVizNode::new("orig_nfa".to_string(),
                                          vec![
                                              GraphvizNodeStyleItem::Label("NFA by incremental method".to_string()),
                                              GraphvizNodeStyleItem::Image(format!("{}.png",orig_nfa_name)),
                                              GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                          ]
                        )
                    );
                    /*graph.add_node(
                        GraphVizNode::new("compo_nfa".to_string(),
                                          vec![
                                              GraphvizNodeStyleItem::Image(format!("{}.png",compositional_nfa_name)),
                                              GraphvizNodeStyleItem::Label("".to_string()),
                                              GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                          ]
                        )
                    );*/
                    graph.add_node(
                        GraphVizNode::new("min_nfa".to_string(),
                                          vec![
                                              GraphvizNodeStyleItem::Image(format!("{}.png",min_dfa_name)),
                                              GraphvizNodeStyleItem::Label("".to_string()),
                                              GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                          ]
                        )
                    );
                    graph.add_edge(
                        GraphVizEdge::new("int".to_string(),
                                          None,
                                          "orig_nfa".to_string(),
                                          None,
                                          vec![])
                    );
                    /*graph.add_edge(
                        GraphVizEdge::new("int".to_string(),
                                          None,
                                          "compo_nfa".to_string(),
                                          None,
                                          vec![])
                    );*/
                    graph.add_edge(
                        GraphVizEdge::new("orig_nfa".to_string(),
                                          None,
                                          "min_nfa".to_string(),
                                          None,
                                          vec![])
                    );
                    for lf_id in 0..printer.gen_ctx.get_lf_num() {
                        let closure =
                            |x : &usize| -> bool {
                                printer.index_to_action_map.get(*x).unwrap()
                                    .iter().any(|a : &TraceAction| a.lf_id == lf_id)
                            };
                        let hid_nfa = min_dfa.to_nfait().hide_letters(false, &closure);
                        let hid_nfa_name = format!("{}_hid_{}_nfa", file_name, lf_id);
                        hid_nfa.to_dot(false,&hashset!{},&printer)
                            .print_dot(&[".".to_string()],
                                                  &hid_nfa_name,
                                                  &GraphVizOutputFormat::png);
                        graph.add_node(
                            GraphVizNode::new(format!("hid_{}_nfa", lf_id),
                                              vec![
                                                  GraphvizNodeStyleItem::Image(format!("{}.png",hid_nfa_name)),
                                                  GraphvizNodeStyleItem::Label("".to_string()),
                                                  GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                              ]
                            )
                        );
                        graph.add_edge(
                            GraphVizEdge::new("min_nfa".to_string(),
                                              None,
                                              format!("hid_{}_nfa", lf_id),
                                              None,
                                              vec![])
                        );
                        let epsilon_closed = hid_nfa.to_nfa();
                        let epsilon_closed_name = format!("{}_hid_{}_closed_nfa", file_name, lf_id);
                        epsilon_closed.to_dot(false,&hashset!{},&printer)
                            .print_dot(&[".".to_string()],
                                                  &epsilon_closed_name,
                                                  &GraphVizOutputFormat::png);
                        graph.add_node(
                            GraphVizNode::new(format!("hid_{}_closed_nfa", lf_id),
                                              vec![
                                                  GraphvizNodeStyleItem::Image(format!("{}.png",epsilon_closed_name)),
                                                  GraphvizNodeStyleItem::Label("".to_string()),
                                                  GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                              ]
                            )
                        );
                        graph.add_edge(
                            GraphVizEdge::new(format!("hid_{}_nfa", lf_id),
                                              None,
                                              format!("hid_{}_closed_nfa", lf_id),
                                              None,
                                              vec![])
                        );
                    }
                    graph.print_dot(&[".".to_string()],&format!("{}_glosem",file_name),&GraphVizOutputFormat::svg);
                    return (ret_print,0);
                }
            }
        }
    }
}