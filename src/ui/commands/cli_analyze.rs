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

use crate::core::execution::trace::multitrace::multi_trace_length;
use crate::io::input::hcf::ana::interface::parse_hcf_file_for_ana;
use crate::io::input::hcf::ana::options::HibouAnalyzeOptions;
use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;
use crate::io::input::htf::interface::parse_htf_file;
use crate::process::ana::conf::AnalysisConfig;
use crate::process::ana::context::AnalysisContext;
use crate::process::ana::node::flags::MultiTraceAnalysisFlags;
use crate::process::ana::node::node::AnalysisNodeKind;
use crate::process::ana::priorities::AnalysisPriorities;
use crate::process::ana::step::AnalysisStepKind;


pub fn cli_analyze(matches : &ArgMatches) -> (Vec<String>,u32) {
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
                Ok( int ) => {
                    let htf_file_path = matches.value_of("htf").unwrap();
                    match parse_htf_file(&gen_ctx,htf_file_path) {
                        Err(e) => {
                            return (vec![e.to_string()],1);
                        },
                        Ok( (co_localizations,multi_trace) ) => {
                            let ana_opts : HibouAnalyzeOptions;
                            if matches.is_present("hcf") {
                                let hcf_file_path = matches.value_of("hcf").unwrap();
                                match parse_hcf_file_for_ana(&gen_ctx,hcf_file_path) {
                                    Err(e) => {
                                        return (vec![e.to_string()],1);
                                    },
                                    Ok( got_ana_opt) => {
                                        ana_opts = got_ana_opt;
                                    }
                                }
                            } else {
                                ana_opts = HibouAnalyzeOptions::default();
                            }
                            // ***
                            let multi_trace_length = multi_trace_length(&multi_trace);
                            let mut ret_print = vec![];
                            ret_print.push( "ANALYZING TRACE".to_string());
                            ret_print.push( format!("from file '{}'",htf_file_path) );
                            ret_print.push( format!("of length '{:?}'", multi_trace_length) );
                            ret_print.push( "W.R.T. INTERACTION".to_string());
                            ret_print.push( format!("from file '{}'",hsf_file_path) );
                            if ana_opts.ana_param.partial_order_reduction {
                                ret_print.push( "WARNING : uses experimental Partial Order Reduction that returns FALSE NEGATIVES when using synchronous operator".to_string() );
                            }
                            ret_print.push( "".to_string());
                            // ***
                            let ana_ctx = AnalysisContext::new(gen_ctx,co_localizations,multi_trace,multi_trace_length);
                            let delegate : GenericProcessDelegate<AnalysisStepKind,AnalysisNodeKind,AnalysisPriorities> = GenericProcessDelegate::new(ana_opts.strategy,ana_opts.priorities);

                            let init_flags : MultiTraceAnalysisFlags;
                            match ana_opts.ana_param.ana_kind.get_sim_config() {
                                None => {
                                    init_flags = MultiTraceAnalysisFlags::new_init(ana_ctx.co_localizations.num_colocs(),
                                                                                   0,
                                                                                   0);
                                },
                                Some( sim_config ) => {
                                    init_flags = MultiTraceAnalysisFlags::new_init(ana_ctx.co_localizations.num_colocs(),
                                                                                   sim_config.get_reset_rem_loop(multi_trace_length,&int),
                                                                                   sim_config.get_reset_rem_act(multi_trace_length,&int));
                                }
                            }
                            let mut analysis_manager : GenericProcessManager<AnalysisConfig> = GenericProcessManager::new(ana_ctx,
                                                                                                                          ana_opts.ana_param,
                                                                                                                                delegate,
                                                                                                                          ana_opts.filters,
                                                                                                                          ana_opts.loggers,
                                                                                                                          ana_opts.goal,
                                                                                                                          ana_opts.use_memoization);


                            let init_node = AnalysisNodeKind::new(int,init_flags,0);
                            // ***
                            let now = Instant::now();
                            let (node_count,verdict) = analysis_manager.start_process(init_node);
                            let elapsed_time = now.elapsed();
                            ret_print.push( format!("verdict    : '{}'", verdict.to_string() ) );
                            ret_print.push( format!("node count : {:?}", node_count ) );
                            ret_print.push( format!("elapsed    : {:?}", elapsed_time.as_secs_f64() ) );
                            return (ret_print,0);
                        }
                    }
                }
            }
        }
    }
}