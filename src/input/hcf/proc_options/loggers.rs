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

use pest::iterators::Pair;
use crate::core::colocalizations::CoLocalizations;

use crate::core::general_context::GeneralContext;
use crate::input::error::HibouParsingError;


use crate::loggers::graphic::conf::{GraphicProcessLoggerInteractionRepresentation, GraphicProcessLoggerLayout, GraphicProcessLoggerOutputKind};
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;
use crate::loggers::tracegen::conf::TracegenProcessLoggerGeneration;
use crate::loggers::tracegen::tracegen_logger::TraceGenProcessLogger;


#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::input::hcf::parser::{HcfParser,Rule};


pub fn parse_graphic_logger(file_name : &str, logger_kind_pair : Pair<Rule>) -> GraphicProcessLogger {
    let graphic_logger : GraphicProcessLogger;
    // ***
    let graphic_logger_opts_pair = logger_kind_pair.into_inner().next();
    match graphic_logger_opts_pair {
        None => {
            graphic_logger = GraphicProcessLogger::new(file_name.to_string(),
                                                            GraphicProcessLoggerOutputKind::png,
                                                            GraphicProcessLoggerLayout::vertical,
                                                       GraphicProcessLoggerInteractionRepresentation::diagram );
        },
        Some(graphic_logger_opts) => {
            let mut output_kind = GraphicProcessLoggerOutputKind::png;
            let mut layout_kind = GraphicProcessLoggerLayout::vertical;
            for opt_pair in graphic_logger_opts.into_inner() {
                match opt_pair.as_rule() {
                    Rule::GRAPHIC_LOGGER_png => {
                        output_kind = GraphicProcessLoggerOutputKind::png;
                    },
                    Rule::GRAPHIC_LOGGER_svg => {
                        output_kind = GraphicProcessLoggerOutputKind::svg;
                    },
                    Rule::GRAPHIC_LOGGER_vertical => {
                        layout_kind = GraphicProcessLoggerLayout::vertical;
                    },
                    Rule::GRAPHIC_LOGGER_horizontal => {
                        layout_kind = GraphicProcessLoggerLayout::horizontal;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", opt_pair.as_rule());
                    }
                }
            }
            graphic_logger = GraphicProcessLogger::new(file_name.to_string(),
                                                       output_kind,
                                                       layout_kind,
                                                       GraphicProcessLoggerInteractionRepresentation::diagram);
        }
    }
    // ***
    return graphic_logger;
}



pub fn parse_tracegen_logger(gen_ctx : &GeneralContext,
                             file_name : &str,
                             logger_kind_pair : Pair<Rule>) -> Result<TraceGenProcessLogger,HibouParsingError> {
    let tracegen_logger : TraceGenProcessLogger;
    // ***
    let tracegen_logger_opts_pair = logger_kind_pair.into_inner().next();
    match tracegen_logger_opts_pair {
        None => {
            tracegen_logger = TraceGenProcessLogger::new(file_name.to_string(),
                                                         TracegenProcessLoggerGeneration::exact,
                                                         CoLocalizations::get_trivial_partition(gen_ctx.get_lf_num()));
        },
        Some(tracegen_logger_opts) => {
            let mut generation = TracegenProcessLoggerGeneration::exact;
            let mut co_localizations = CoLocalizations::get_trivial_partition(gen_ctx.get_lf_num());
            for opt_pair in tracegen_logger_opts.into_inner() {
                match opt_pair.as_rule() {
                    Rule::TRACEGEN_LOGGER_terminal => {
                        generation = TracegenProcessLoggerGeneration::terminal;
                    },
                    Rule::TRACEGEN_LOGGER_exact => {
                        generation = TracegenProcessLoggerGeneration::exact;
                    },
                    Rule::TRACEGEN_LOGGER_prefix => {
                        generation = TracegenProcessLoggerGeneration::prefixes;
                    },
                    Rule::TRACEGEN_LOGGER_partition_discrete => {
                        co_localizations = CoLocalizations::get_discrete_partition(gen_ctx.get_lf_num());
                    },
                    Rule::TRACEGEN_LOGGER_partition_trivial => {
                        co_localizations = CoLocalizations::get_trivial_partition(gen_ctx.get_lf_num());
                    },
                    Rule::TRACEGEN_LOGGER_partition_specific => {
                        let mut colocs : Vec<HashSet<usize>> = vec![];
                        // ***
                        let mut got_lfs = hashset!{};
                        // ***
                        for lfs_list_pair in opt_pair.into_inner() {
                            let mut coloc = hashset!{};
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
            tracegen_logger = TraceGenProcessLogger::new(file_name.to_string(),
                                                         generation,
                                                         co_localizations);
        }
    }
    // ***
    return Ok( tracegen_logger );
}

