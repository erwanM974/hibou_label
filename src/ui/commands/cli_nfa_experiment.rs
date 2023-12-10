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

use autour_core::traits::characterize::AutCharacterizable;

use crate::core::execution::trace::trace::TraceAction;

use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::nfa_translation::experiments::run_nfa_generation_experiment;


pub fn cli_nfa_experiment(matches : &ArgMatches) -> (Vec<String>,u32) {
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

                    let num_tries : u32 = match matches.value_of("num_tries") {
                        None => {
                            3
                        },
                        Some( as_str ) => {
                            as_str.trim().parse::<u32>().unwrap()
                        }
                    };

                    let states_lim : usize = match matches.value_of("states_lim") {
                        None => {
                            10
                        },
                        Some( as_str ) => {
                            as_str.trim().parse::<usize>().unwrap()
                        }
                    };

                    let result = run_nfa_generation_experiment(int,gen_ctx,num_tries,states_lim);

                    let mut ret_print = vec![];
                    // ***
                    ret_print.push( "".to_string());
                    ret_print.push( format!("Generating FAs from Interaction from file '{}'",hsf_file_path) );
                    ret_print.push( format!("NFA via operational method : time : {:?}μs, num states {:?}",
                                            result.nfa_operational_median_time,
                                            result.nfa_operational.transitions.len() ) );
                    ret_print.push( format!("minimized NFA after operational method : time to minimize : {:?}μs , num states {:?}",
                                            result.nfa_kw_med_time_from_opera,
                                            result.nfa_minimized_kw_from_opera.transitions.len() ) );
                    ret_print.push( format!("minimized DFA after operational method : time to minimize : {:?}μs , num states {:?}",
                                            result.mindfa_med_time_from_opera,
                                            result.dfa_minimized_from_opera.transitions.len() ) );
                    ret_print.push( format!("NFA via compositional method : time : {:?}μs , num states {:?}",
                                            result.nfa_compositional_median_time,
                                            result.nfa_compositional.transitions.len() ) );

                    if result.nfa_operational.equals(&result.nfa_compositional) {
                        ret_print.push("compositional NFA is a correct translation".to_string());
                    } else {
                        ret_print.push("compositional NFA is not a correct translation".to_string());
                    }
                    ret_print.push( format!("minimized NFA after compositional method : time to minimize : {:?}μs , num states {:?}",
                                            result.nfa_kw_med_time_from_compo,
                                            result.nfa_minimized_kw_from_compo.transitions.len() ) );
                    ret_print.push( format!("minimized DFA after compositional method : time to minimize : {:?}μs , num states {:?}",
                                            result.mindfa_med_time_from_compo,
                                            result.dfa_minimized_from_compo.transitions.len() ) );

                    // ***

                    if matches.is_present("draw") {
                        let int_name = format!("{}_int",file_name);
                        let orig_nfa_name = format!("{}_orig_nfa",file_name);
                        let min_nfa_name = format!("{}_mini_nfa",file_name);
                    }

                    return (ret_print,0);
                }
            }
        }
    }
}