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


use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
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
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::core::execution::trace::trace::TraceAction;
use crate::experiments::interaction_random_gen::interface::generate_canonical_random_interaction;
use crate::experiments::interaction_random_gen::probas::InteractionSymbolsProbabilities;
use crate::io::file_extensions::HIBOU_INTERACTION_FILE_EXTENSION;

use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::io::output::to_hfiles::interaction::to_hif::interaction_to_hif;
use crate::nfa_translation::alphabet::get_alphabet_from_gen_ctx;
use crate::nfa_translation::experiments2::run_nfa_generation_experiment2;
use crate::nfa_translation::experiments::run_nfa_generation_experiment;


pub fn cli_rng_gen_interactions(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {

            let number_of_interactions : u32 = match matches.value_of("num_ints") {
                None => {
                    350
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let num_tries : u32 = match matches.value_of("num_tries") {
                None => {
                    number_of_interactions*3
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let max_depth : u32 = match matches.value_of("max_depth") {
                None => {
                    10
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let min_symbols : u32 = match matches.value_of("min_symbols") {
                None => {
                    100
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u32>().unwrap()
                }
            };

            let seed : u64 = match matches.value_of("seed") {
                None => {
                    0
                },
                Some( as_str ) => {
                    as_str.trim().parse::<u64>().unwrap()
                }
            };

            let output_folder : String = if matches.is_present("folder") {
                let extracted = matches.value_of("folder").unwrap();
                extracted.to_string()
            } else {
                "gen_ints".to_string()
            };

            let mut probas_name = "default";
            let probas = if matches.is_present("probas") {
                let extracted = matches.value_of("probas").unwrap();
                match extracted {
                    "conservative" => {
                        probas_name = "conservative";
                        InteractionSymbolsProbabilities::conservative()
                    },
                    "protocols_with_coreg" => {
                        probas_name = "conservative";
                        InteractionSymbolsProbabilities::protocols_with_coreg()
                    },
                    "default" => {
                        InteractionSymbolsProbabilities::default_non_regular()
                    },
                    _ => {
                        panic!("unknown probas for interactions generations : '{:}'", extracted)
                    }
                }
            } else {
                InteractionSymbolsProbabilities::default_non_regular()
            };



            let mut ret_print = vec![];
            ret_print.push( "generated random interactions interactions".to_string());
            ret_print.push(
                format!("with {:} interaction symbols selection probabilities",
                        probas_name)
            );
            ret_print.push(
                format!("num_ints : {:}, max_depth : {:}, min_symbols : {:}, seed : {:}",
                        number_of_interactions,
                        max_depth,
                        min_symbols,
                        seed)
            );
            ret_print.push( format!(
                "in folder '{:}'",
                output_folder)
            );

            let mut memoized_ints = HashSet::new();

            let mut rng = StdRng::seed_from_u64(seed);
            let mut x = 0;
            let mut max_tries = num_tries;
            'myloop : while x < number_of_interactions {
                println!("trying to generate interaction {} out of {}", x, number_of_interactions);
                let mut got_one = false;
                if let Some(i) = generate_canonical_random_interaction(&gen_ctx,
                                                              &mut rng,
                                                              max_depth,
                                                              min_symbols,
                                                              &probas
                ) {
                    if !memoized_ints.contains(&i) {
                        got_one = true;
                        let file_name = format!("i{:}.{:}", x, HIBOU_INTERACTION_FILE_EXTENSION);
                        let path : PathBuf = [&output_folder, &file_name].iter().collect();
                        interaction_to_hif(path.as_path(),&gen_ctx,&i);
                        memoized_ints.insert(i);
                        x += 1;
                        println!("wrote to file '{:?}'", path.as_path())
                    }
                }
                if !got_one {
                    println!("retrying...");
                    max_tries -= 1;
                    if max_tries <= 0 {
                        println!("... max retries exceeded");
                        break 'myloop;
                    }
                }

            }



            return (ret_print,0);
        }
    }
}