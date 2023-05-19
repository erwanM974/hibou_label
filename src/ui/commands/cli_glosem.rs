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
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_loggers::nfait::logger::GenericNFAITLogger;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable, DotPrintable, GraphVizOutputFormat};
use crate::core::execution::trace::trace::TraceAction;

use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::process::explo::conf::ExplorationConfig;
use crate::process::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::process::explo::filter::filter::ExplorationFilter;
use crate::process::explo::loggers::nfait::printer::ActionNFAITPrinter;
use crate::process::explo::node::ExplorationNodeKind;
use crate::process::explo::priorities::ExplorationPriorities;
use crate::process::explo::step::ExplorationStepKind;


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
                    let min_nfa_name = format!("{}_mini_nfa",file_name);

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

                    let mut ret_print = vec![];
                    // ***
                    ret_print.push( "".to_string());
                    ret_print.push( "Generating NFA from Exploration".to_string());
                    ret_print.push( format!("of interaction from file '{}'",hsf_file_path) );
                    ret_print.push( format!("with max loop depth '{}'",max_loop_depth) );
                    ret_print.push( "".to_string());
                    // ***
                    let nfa_logger = GenericNFAITLogger::new(ActionNFAITPrinter::new(vec![],gen_ctx.clone()),
                                                             "glosem".to_string(),
                                                             None,
                                                             ".".to_string(),);
                    let explo_ctx = ExplorationContext::new(gen_ctx.clone());
                    let delegate : GenericProcessDelegate<ExplorationStepKind,ExplorationNodeKind,ExplorationPriorities> =
                        GenericProcessDelegate::new(QueueSearchStrategy::BFS,GenericProcessPriorities::new(ExplorationPriorities::default(),false));

                    let mut exploration_manager : GenericProcessManager<ExplorationConfig> =
                        GenericProcessManager::new(explo_ctx,
                                                   ExplorationParameterization{},
                                                   delegate,
                                                   vec![Box::new(ExplorationFilter::MaxLoopInstanciation(max_loop_depth))],
                                                   vec![Box::new(nfa_logger)],
                                                   None,
                                                   true);

                    // ***
                    // ***
                    let init_node = ExplorationNodeKind::new(int,0);
                    // ***
                    let now = Instant::now();
                    let (node_count,_) = exploration_manager.start_process(init_node);
                    let elapsed_get_nfa = now.elapsed();
                    // ***
                    let raw_logger = exploration_manager.get_logger(0).unwrap();
                    let nfa_logger : &GenericNFAITLogger<ExplorationConfig,usize,ActionNFAITPrinter> =
                        raw_logger.as_any().downcast_ref::<GenericNFAITLogger<ExplorationConfig,usize,ActionNFAITPrinter>>().unwrap();
                    // ***
                    let nfa = nfa_logger.get_nfait();
                    let mut min_nfa = nfa.clone();
                    let now = Instant::now();
                    min_nfa = min_nfa.minimize();
                    let elapsed_min_nfa = now.elapsed();
                    // ***
                    ret_print.push( format!("orig NFA : time : {:?} , num states {:?}", elapsed_get_nfa, nfa.transitions.len() ) );
                    ret_print.push( format!("min NFA : time : {:?} , num states {:?}", elapsed_min_nfa, min_nfa.transitions.len() ) );
                    // ***
                    let orig_nfa_as_dot = nfa.to_dot(false,&hashset!{},&nfa_logger.builder_printer);
                    orig_nfa_as_dot.print_dot(&[".".to_string()],
                                              &orig_nfa_name,
                                              &GraphVizOutputFormat::png);
                    let mini_nfa_as_dot = min_nfa.to_dot(false,&hashset!{},&nfa_logger.builder_printer);
                    mini_nfa_as_dot.print_dot(&[".".to_string()],
                                              &min_nfa_name,
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
                                              GraphvizNodeStyleItem::Image(format!("{}.png",orig_nfa_name)),
                                              GraphvizNodeStyleItem::Label("".to_string()),
                                              GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                          ]
                        )
                    );
                    graph.add_node(
                        GraphVizNode::new("min_nfa".to_string(),
                                          vec![
                                              GraphvizNodeStyleItem::Image(format!("{}.png",min_nfa_name)),
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
                    graph.add_edge(
                        GraphVizEdge::new("orig_nfa".to_string(),
                                          None,
                                          "min_nfa".to_string(),
                                          None,
                                          vec![])
                    );
                    for lf_id in 0..gen_ctx.get_lf_num() {
                        let closure =
                            |x : &usize| -> bool {
                                nfa_logger.builder_printer.index_to_action_map.get(*x).unwrap()
                                    .iter().any(|a : &TraceAction| a.lf_id == lf_id)
                            };
                        let hid_nfa = min_nfa.clone().hide_letters(false, &closure);
                        let hid_nfa_name = format!("{}_hid_{}_nfa", file_name, lf_id);
                        hid_nfa.to_dot(false,&hashset!{},&nfa_logger.builder_printer)
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
                        let epsilon_closed = hid_nfa.to_nfa().to_nfait();
                        let epsilon_closed_name = format!("{}_hid_{}_closed_nfa", file_name, lf_id);
                        epsilon_closed.to_dot(false,&hashset!{},&nfa_logger.builder_printer)
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