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


use std::collections::BTreeSet;
use std::path::Path;
use std::time::Instant;

use autour_core::traits::repr::AutGraphvizDrawable;
use autour_process::autana::conf::NfaWordAnalysisConfig;
use autour_process::autana::context::{NfaWordAnalysisContext, NfaWordAnalysisParameterization, NfaWordAnalysisResetOn};
use autour_process::autana::loggers::glog::drawer::NfaWordAnalysisProcessDrawer;
use autour_process::autana::node::NfaWordAnalysisNodeKind;
use autour_process::autana::priorities::NfaWordAnalysisPriorities;
use autour_process::autana::step::NfaWordAnalysisStepKind;


use clap::ArgMatches;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_loggers::graphviz::format::GraphVizProcessLoggerLayout;
use graph_process_manager_loggers::graphviz::logger::GenericGraphVizLogger;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable, DotPrintable, GraphVizOutputFormat};

use crate::core::execution::trace::multitrace::multi_trace_length;
use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::input::htf::interface::parse_htf_file;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::process::explo::loggers::nfait::printer::ActionNFAITPrinter;
use crate::ui::commands::get_nfa_from_logger::get_nfa_from_interaction_exploration;


pub fn cli_nfa_ana(matches : &ArgMatches) -> (Vec<String>,u32) {
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
                    let htf_file_path = matches.value_of("htf").unwrap();
                    match parse_htf_file(&gen_ctx,htf_file_path) {
                        Err(e) => {
                            return (vec![e.to_string()],1);
                        },
                        Ok( (co_localizations,mut multi_trace) ) => {
                            // ***
                            let multi_trace_length = multi_trace_length(&multi_trace);
                            // ***
                            if co_localizations.locs_lf_ids.len() != 1 {
                                return (vec!["trace should be a global trace".to_string()],1);
                            }
                            let trace = multi_trace.remove(0);

                            let max_loop_depth = int.max_nested_loop_depth()*2;
                            let (nfa,printer,elapsed_get_nfa) = get_nfa_from_interaction_exploration(
                                "glosem".to_string(),
                                &gen_ctx,
                                &int,
                                max_loop_depth);
                            let initial_active_states : BTreeSet<usize> = nfa.initials.iter().cloned().collect();

                            if matches.is_present("draw_transformation") {
                                let int_name = format!("{}_int",file_name);
                                let nfa_name = format!("{}_nfa",file_name);
                                draw_interaction(&gen_ctx,
                                                 &int,
                                                 &InteractionGraphicalRepresentation::AsSequenceDiagram,
                                                 &".".to_string(),
                                                 &".".to_string(),
                                                 &int_name);
                                let nfa_as_dot = nfa.to_dot(false,&hashset!{},&printer);
                                nfa_as_dot.print_dot(&[".".to_string()],
                                                          &nfa_name,
                                                          &GraphVizOutputFormat::png);
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
                                    GraphVizNode::new("nfa".to_string(),
                                                      vec![
                                                          GraphvizNodeStyleItem::Image(format!("{}.png",nfa_name)),
                                                          GraphvizNodeStyleItem::Label("".to_string()),
                                                          GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
                                                      ]
                                    )
                                );
                                graph.add_edge(
                                    GraphVizEdge::new("int".to_string(),
                                                      None,
                                                      "nfa".to_string(),
                                                      None,
                                                      vec![])
                                );
                                graph.print_dot(&[".".to_string()],
                                                &format!("{}_transfo",file_name),
                                                &GraphVizOutputFormat::svg);
                            }

                            let loggers: Vec<std::boxed::Box<(dyn AbstractProcessLogger<NfaWordAnalysisConfig<ActionNFAITPrinter>> + 'static)>> = if matches.is_present("draw_analysis") {
                                let drawer = NfaWordAnalysisProcessDrawer::new("temp".to_string());
                                let graphic_logger : GenericGraphVizLogger<NfaWordAnalysisConfig<ActionNFAITPrinter>> = GenericGraphVizLogger::new(
                                    Box::new(drawer),
                                    GraphVizOutputFormat::svg,
                                    GraphVizProcessLoggerLayout::Vertical,
                                    true,
                                    ".".to_string(),
                                    format!("ana_{}",file_name));
                                vec![Box::new(graphic_logger)]
                            } else {
                                vec![]
                            };


                            let init_node = NfaWordAnalysisNodeKind::new(initial_active_states,false,0);

                            let outside_letter = printer.index_to_action_map.len();

                            let word : Vec<usize> = trace.iter().map(
                                |x|
                                    match printer.index_to_action_map.iter().position(|y| y == x) {
                                        Some(y) => {y},
                                        None => {outside_letter}
                                    }
                            ).collect();
                            let process_ctx : NfaWordAnalysisContext<ActionNFAITPrinter> = NfaWordAnalysisContext::new(nfa,printer,word);
                            let priorities : GenericProcessPriorities<NfaWordAnalysisPriorities> = GenericProcessPriorities::new(NfaWordAnalysisPriorities{},false);
                            let delegate : GenericProcessDelegate<NfaWordAnalysisStepKind,NfaWordAnalysisNodeKind,NfaWordAnalysisPriorities> = GenericProcessDelegate::new(QueueSearchStrategy::BFS,
                                                                                                                                                                           priorities);
                            let mut manager : GenericProcessManager<NfaWordAnalysisConfig<ActionNFAITPrinter>> = GenericProcessManager::new(process_ctx,
                                                                                                                                        NfaWordAnalysisParameterization::new(NfaWordAnalysisResetOn::AllStates),
                                                                                                                                        delegate,
                                                                                                                                        vec![],
                                                                                                                                        loggers,
                                                                                                                                        None,
                                                                                                                                        false);

                            let now = Instant::now();
                            let (_, verdict) = manager.start_process(init_node);
                            let elapsed_nfa_ana = now.elapsed();

                            let mut ret_print = vec![];
                            // ***
                            ret_print.push( "".to_string());
                            ret_print.push( "Trace analysis using NFA monitor".to_string());
                            ret_print.push( format!("from interaction from file '{}'",hsf_file_path) );
                            ret_print.push( format!("and trace from file '{}'",htf_file_path) );
                            ret_print.push( format!("of length '{:?}'", multi_trace_length) );
                            ret_print.push( "".to_string());
                            ret_print.push( format!("time to transform int to nfa : '{:?}'",elapsed_get_nfa.as_secs_f64()) );
                            if matches.is_present("draw_analysis") {
                                ret_print.push( format!("time of analysis using nfa : '{:?}' (slowed by graphic representation)",elapsed_nfa_ana.as_secs_f64()) );
                            } else {
                                ret_print.push( format!("time of analysis using nfa : '{:?}'",elapsed_nfa_ana.as_secs_f64()) );
                            }
                            ret_print.push( format!("verdict : '{:}'",verdict) );
                            // ***
                            return (ret_print,0);
                        }
                    }
                }
            }
        }
    }
}