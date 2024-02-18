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


use autour_core::nfa::nfa::AutNFA;
use autour_core::traits::translate::AutTranslatable;
use clap::ArgMatches;
use crate::core::language::syntax::metrics::InteractionMetrics;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::input::hsf::interface::parse_hsf_file;
use crate::nfa_translation::alphabet::get_alphabet_from_gen_ctx;
use crate::nfa_translation::canonize::canonize_interaction;
use crate::nfa_translation::get_nfa_from_logger::get_nfa_from_interaction_exploration;
use crate::process::canon::param::default::DefaultCanonizationProcess;


pub fn cli_get_metrics(matches : &ArgMatches) -> (Vec<String>, u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {
            let hif_file_path = matches.value_of("hif").unwrap();
            match parse_hif_file(&gen_ctx,hif_file_path) {
                Err(e) => {
                    return (vec![e.to_string()],1);
                },
                Ok( int) => {

                    fn median(numbers: Vec<u128>) -> u128 {
                        let mut numbers = numbers;
                        numbers.sort();
                        let mid = numbers.len() / 2;
                        numbers[mid]
                    }

                    let int = canonize_interaction(&gen_ctx,&int,DefaultCanonizationProcess::BasicWithUnfoldActions);

                    let mut ret_print = vec![];
                    ret_print.push( "".to_string());
                    ret_print.push( format!("for interaction from file '{}'",hif_file_path) );

                    match matches.value_of("kind").unwrap() {
                        "INT" => {
                            ret_print.push( "GETTING INTERACTION METRICS".to_string());
                            let imetrics = InteractionMetrics::extract_from_interaction(&int);
                            ret_print.append(&mut imetrics.string_summary());
                        },
                        "operatNFA" => {
                            let alphabet = get_alphabet_from_gen_ctx(&gen_ctx);

                            let mut nfa = AutNFA::new_void_object(hashset!{0});
                            let mut durations = vec![];
                            for _ in 0..5 {
                                let (got_nfa,duration) = get_nfa_from_interaction_exploration(&gen_ctx,
                                                                                          &int,
                                                                                          alphabet.clone());
                                let in_micros = duration.as_micros();
                                nfa = got_nfa;
                                println!("via exploration translated interaction into nfa of {:} states in {:}μs", nfa.transitions.len(), in_micros);
                                durations.push(in_micros);
                            }

                            let num_states = nfa.transitions.len();
                            let num_edges = nfa.transitions.iter()
                                .fold(0,|x,t|
                                    x + t.iter().fold(0,|y,(_,c)| y + (c.len()))
                                );
                            let median_duration = median(durations);

                            ret_print.push( "GETTING OPERATIONAL NFA METRICS".to_string());
                            ret_print.push( format!("number of states              : {:}", num_states));
                            ret_print.push( format!("number of edges               : {:}", num_edges));
                            ret_print.push( format!("synthesis time (median of 5)  : {:}", median_duration));
                        },
                        "minDFA" => {
                            let alphabet = get_alphabet_from_gen_ctx(&gen_ctx);
                            let (nfa,_) = get_nfa_from_interaction_exploration(&gen_ctx,
                                                                               &int,
                                                                               alphabet.clone());
                            let dfa = nfa.to_dfa();
                            let num_states = dfa.transitions.len();
                            let num_edges = dfa.transitions.iter()
                                .fold(0,|x,t|
                                    x + t.len()
                                );

                            ret_print.push( "GETTING DFA METRICS".to_string());
                            ret_print.push( format!("number of states              : {:}", num_states));
                            ret_print.push( format!("number of edges               : {:}", num_edges));
                        },
                        x => {
                            ret_print.push( format!("unkown metrics : {:}", x));
                            ret_print.push( "".to_string());
                            ret_print.push( "requires either of INT, operatNFA or minDFA".to_string());
                        }
                    }
                    ret_print.push( "".to_string());
                    return (ret_print,0);
                }
            }
        }
    }
}