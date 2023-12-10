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


use std::fs::File;
use std::io::Write;
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
use crate::nfa_translation::experiments2::run_nfa_generation_experiment2;
use crate::nfa_translation::experiments::run_nfa_generation_experiment;


pub fn cli_nfa_experiment2(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {

            let num_tries : u32 = match matches.value_of("num_tries") {
                None => {
                    1
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let number_of_interactions : u32 = match matches.value_of("num_ints") {
                None => {
                    30
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let gen_depth : u32 = match matches.value_of("gen_depth") {
                None => {
                    10
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let max_symbols : u32 = match matches.value_of("max_symbols") {
                None => {
                    100
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let max_par : u32 = match matches.value_of("max_par") {
                None => {
                    6
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            println!("num_ints : {:}, gen_depth : {:}, max_symbols : {:}, max_par : {:}",
                     number_of_interactions,
                     gen_depth,
                     max_symbols,
                     max_par);

            /*let output_file_name = if matches.is_present("output") {
                let extracted = matches.value_of("output").unwrap();
                format!("{}.csv", extracted)
            } else {
                let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
                format!("{}_exp.csv", file_name)
            };*/
            let output_file_name = format!("{}_exp.csv", Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap());

            let csv_results = run_nfa_generation_experiment2(number_of_interactions,
                                                             &gen_ctx,
                                                             num_tries,
                                                             gen_depth,
                                                             max_symbols,
                                                             max_par);

            let mut file = File::create(output_file_name.clone()).unwrap();
            file.write(csv_results.as_bytes() );

            let mut ret_print = vec![];
            ret_print.push( "".to_string());
            ret_print.push( format!(
                "Generated {:} random interactions with gen depth {:}",
                number_of_interactions,
                gen_depth)
            );
            ret_print.push( format!(
                "and max symbols {:}",
                max_symbols)
            );
            ret_print.push( format!(
                "using signature from file '{:}'",
                hsf_file_path)
            );
            ret_print.push( "and default symbols probabilities".to_string());
            ret_print.push( "".to_string());
            ret_print.push( "generated incremental and compositional NFAs".to_string());
            ret_print.push( format!(
                "collected metrics in file '{:}'",
                output_file_name)
            );

            return (ret_print,0);
        }
    }
}