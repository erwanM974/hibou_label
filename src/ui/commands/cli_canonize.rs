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



use std::time::Instant;
use clap::ArgMatches;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use crate::io::input::hcf::canon::interface::parse_hcf_file_for_canonize;
use crate::io::input::hcf::canon::options::HibouCanonizeOptions;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::input::hsf::interface::parse_hsf_file;
use crate::process::canon::conf::CanonizationConfig;
use crate::process::canon::context::CanonizationContext;
use crate::process::canon::node::CanonizationNodeKind;
use crate::process::canon::param::default::DefaultCanonizationProcess;
use crate::process::canon::param::phase::CanonizationParameterization;
use crate::process::canon::priorities::CanonizationPriorities;
use crate::process::canon::step::CanonizationStepKind;


pub fn cli_canonize(matches : &ArgMatches) -> (Vec<String>,u32) {
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
                    let canon_opts : HibouCanonizeOptions;
                    if matches.is_present("hcf") {
                        let hcf_file_path = matches.value_of("hcf").unwrap();
                        match parse_hcf_file_for_canonize(&gen_ctx,hcf_file_path) {
                            Err(e) => {
                                return (vec![e.to_string()],1);
                            },
                            Ok( got_canon_opt) => {
                                canon_opts = got_canon_opt;
                            }
                        }
                    } else {
                        canon_opts = HibouCanonizeOptions::default();
                    }
                    let mut ret_print = vec![];
                    // ***
                    ret_print.push( "".to_string());
                    ret_print.push( "CANONIZING process for INTERACTION".to_string());
                    ret_print.push( format!("from file '{}'",hif_file_path) );
                    ret_print.push( "".to_string());
                    // ***

                    let canon_ctx = CanonizationContext::new(gen_ctx);
                    let delegate : GenericProcessDelegate<CanonizationStepKind,CanonizationNodeKind,CanonizationPriorities> =
                        GenericProcessDelegate::new(
                            canon_opts.strategy,
                            canon_opts.priorities
                        );

                    let canon_param = CanonizationParameterization::from_default(
                        DefaultCanonizationProcess::FivePhases,
                        canon_opts.search_all
                    );

                    let mut canon_manager : GenericProcessManager<CanonizationConfig> = GenericProcessManager::new(
                        canon_ctx,
                        canon_param,
                        delegate,
                        canon_opts.filters,
                        canon_opts.loggers,
                        None,
                        true
                    );
                    // ***
                    let init_node = CanonizationNodeKind::new(int,0);
                    // ***
                    let now = Instant::now();
                    let (node_count,_) = canon_manager.start_process(init_node);
                    let elapsed_time = now.elapsed();
                    ret_print.push( format!("node count : {:?}", node_count ) );
                    ret_print.push( format!("elapsed    : {:?}", elapsed_time.as_secs_f64() ) );
                    // ***
                    return (ret_print,0);
                }
            }
        }
    }
}

