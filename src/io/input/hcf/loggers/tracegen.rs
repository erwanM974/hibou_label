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


use std::collections::{BTreeSet, HashSet};
use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graph_process_manager_loggers::stepstrace::logger::GenericStepsTraceLogger;
use graph_process_manager_loggers::stepstrace::printer::StepsTraceProcessPrinter;

use pest::iterators::Pair;
use crate::core::colocalizations::CoLocalizations;

use crate::core::general_context::GeneralContext;
use crate::io::file_extensions::HIBOU_TRACE_FILE_EXTENSION;
use crate::io::input::error::HibouParsingError;



#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};

use crate::loggers::tracegen::object::TraceGenLoggerObject;
use crate::loggers::tracegen::printer::{MultiTraceProcessPrinter, TracegenProcessLoggerGeneration};


pub fn parse_tracegen_logger<Conf : AbstractProcessConfiguration>(logger_id : u32,
                                                                  gen_ctx : &GeneralContext,
                                                                  file_name : &str,
                                                                  logger_kind_pair : Pair<Rule>)
        -> Result<GenericStepsTraceLogger<Conf,TraceGenLoggerObject>,HibouParsingError>
        where
            MultiTraceProcessPrinter : StepsTraceProcessPrinter<Conf,TraceGenLoggerObject>  {
    // default configuration
    let mut generation = TracegenProcessLoggerGeneration::accepted;
    let mut co_localizations = CoLocalizations::get_trivial_partition(gen_ctx.get_lf_num());
    let mut parent_folder = format!("tracegen_l{:}", logger_id);
    let mut files_prefix = "trace".to_string();
    // ***
    match logger_kind_pair.into_inner().next() {
        None => {
            // nothing
        },
        Some(tracegen_logger_opts) => {
            for opt_pair in tracegen_logger_opts.into_inner() {
                match opt_pair.as_rule() {
                    Rule::TRACEGEN_LOGGER_parent_folder => {
                        let inner_pair = opt_pair.into_inner().next().unwrap();
                        parent_folder = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    },
                    Rule::TRACEGEN_LOGGER_trace_prefix => {
                        let inner_pair = opt_pair.into_inner().next().unwrap();
                        files_prefix = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    },
                    Rule::TRACEGEN_LOGGER_terminal => {
                        generation = TracegenProcessLoggerGeneration::terminal;
                    },
                    Rule::TRACEGEN_LOGGER_accepted => {
                        generation = TracegenProcessLoggerGeneration::accepted;
                    },
                    Rule::TRACEGEN_LOGGER_depth => {
                        let depth_int_pair = opt_pair.into_inner().next().unwrap();
                        let content_str : String = depth_int_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        let my_val : u32 = content_str.parse::<u32>().unwrap();
                        generation = TracegenProcessLoggerGeneration::atExactDepth(my_val);
                    },
                    Rule::TRACEGEN_LOGGER_modulo => {
                        let depth_int_pair = opt_pair.into_inner().next().unwrap();
                        let content_str : String = depth_int_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        let my_val : u32 = content_str.parse::<u32>().unwrap();
                        generation = TracegenProcessLoggerGeneration::atDepthModulo(my_val);
                    },
                    Rule::TRACEGEN_LOGGER_partition_discrete => {
                        co_localizations = CoLocalizations::get_discrete_partition(gen_ctx.get_lf_num());
                    },
                    Rule::TRACEGEN_LOGGER_partition_trivial => {
                        co_localizations = CoLocalizations::get_trivial_partition(gen_ctx.get_lf_num());
                    },
                    Rule::TRACEGEN_LOGGER_partition_specific => {
                        let mut colocs : Vec<BTreeSet<usize>> = vec![];
                        // ***
                        let mut got_lfs = hashset!{};
                        // ***
                        for lfs_list_pair in opt_pair.into_inner() {
                            let mut coloc = btreeset!{};
                            for lf_name_pair in lfs_list_pair.into_inner() {
                                let lf_name : String = lf_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                                match gen_ctx.get_lf_id(&lf_name) {
                                    None => {
                                        return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
                                    },
                                    Some(lf_id) => {
                                        if got_lfs.contains(&lf_id) {
                                            return Err( HibouParsingError::NonDisjointTraceComponents );
                                        }
                                        got_lfs.insert(lf_id);
                                        coloc.insert(lf_id);
                                    }
                                }
                            }
                            colocs.push(coloc);
                        }
                        // ***
                        co_localizations = CoLocalizations::new(colocs);
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", opt_pair.as_rule());
                    }
                }
            }
        }
    }
    let printer = MultiTraceProcessPrinter::new(co_localizations,generation);
    return Ok(GenericStepsTraceLogger::new(Box::new(printer),
                                           files_prefix,
                                           HIBOU_TRACE_FILE_EXTENSION.to_string(),
                                         parent_folder,
                                         ));
}

