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
use crate::io::input::error::HibouParsingError;


use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;
use crate::loggers::tracegen::conf::TracegenProcessLoggerGeneration;
use crate::loggers::tracegen::tracegen_logger::TraceGenProcessLogger;
use crate::loggers::graphic::conf::{GraphicProcessLoggerLayout,GraphicProcessLoggerOutputFormat};


#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};


pub fn parse_graphic_logger(logger_id : u32,
                            file_name : &str,
                            logger_kind_pair : Pair<Rule>) -> GraphicProcessLogger {
    // default configuration
    let mut output_format = GraphicProcessLoggerOutputFormat::svg;
    let mut layout = GraphicProcessLoggerLayout::vertical;
    let mut int_repr_sd = true;
    let mut int_repr_tt = false;
    let mut display_legend = true;
    let mut display_subprocesses = true;
    let mut parent_folder = "".to_string();
    let mut output_file_name = format!("{:}_l{:}",file_name,logger_id);
    // ***
    match logger_kind_pair.into_inner().next() {
        None => {
            // nothing
        },
        Some(graphic_logger_opts) => {
            for opt_pair in graphic_logger_opts.into_inner() {
                match opt_pair.as_rule() {
                    Rule::GRAPHIC_LOGGER_format_png => {
                        output_format = GraphicProcessLoggerOutputFormat::png;
                    },
                    Rule::GRAPHIC_LOGGER_format_svg => {
                        output_format = GraphicProcessLoggerOutputFormat::svg;
                    },
                    Rule::GRAPHIC_LOGGER_layout_vertical => {
                        layout = GraphicProcessLoggerLayout::vertical;
                    },
                    Rule::GRAPHIC_LOGGER_layout_horizontal => {
                        layout = GraphicProcessLoggerLayout::horizontal;
                    },
                    Rule::GRAPHIC_LOGGER_draw_sequence_diagram => {
                        let inner = opt_pair.into_inner().next().unwrap();
                        match inner.as_rule() {
                            Rule::HIBOU_true => {
                                int_repr_sd = true;
                            },
                            Rule::HIBOU_false => {
                                int_repr_sd = false;
                            },
                            _ => {
                                panic!("what rule then ? : {:?}", inner.as_rule());
                            }
                        }
                    },
                    Rule::GRAPHIC_LOGGER_draw_term_tree => {
                        let inner = opt_pair.into_inner().next().unwrap();
                        match inner.as_rule() {
                            Rule::HIBOU_true => {
                                int_repr_tt = true;
                            },
                            Rule::HIBOU_false => {
                                int_repr_tt = false;
                            },
                            _ => {
                                panic!("what rule then ? : {:?}", inner.as_rule());
                            }
                        }
                    },
                    Rule::GRAPHIC_LOGGER_draw_legend => {
                        let inner = opt_pair.into_inner().next().unwrap();
                        match inner.as_rule() {
                            Rule::HIBOU_true => {
                                display_legend = true;
                            },
                            Rule::HIBOU_false => {
                                display_legend = false;
                            },
                            _ => {
                                panic!("what rule then ? : {:?}", inner.as_rule());
                            }
                        }
                    },
                    Rule::GRAPHIC_LOGGER_draw_sub_processes => {
                        let inner = opt_pair.into_inner().next().unwrap();
                        match inner.as_rule() {
                            Rule::HIBOU_true => {
                                display_subprocesses = true;
                            },
                            Rule::HIBOU_false => {
                                display_subprocesses = false;
                            },
                            _ => {
                                panic!("what rule then ? : {:?}", inner.as_rule());
                            }
                        }
                    },
                    Rule::GRAPHIC_LOGGER_parent_folder => {
                        let inner_pair = opt_pair.into_inner().next().unwrap();
                        parent_folder = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    },
                    Rule::GRAPHIC_LOGGER_output_file => {
                        let inner_pair = opt_pair.into_inner().next().unwrap();
                        output_file_name = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", opt_pair.as_rule());
                    }
                }
            }
        }
    }
    // ***
    return GraphicProcessLogger::new(output_format,
                                     layout,
                                     display_legend,
                                     display_subprocesses,
                                     int_repr_sd,
                                     int_repr_tt,
                                     format!("temp_l{:}", logger_id),
                                     parent_folder,
                                     output_file_name);;
}



pub fn parse_tracegen_logger(logger_id : u32,
                             gen_ctx : &GeneralContext,
                             file_name : &str,
                             logger_kind_pair : Pair<Rule>) -> Result<TraceGenProcessLogger,HibouParsingError> {
    // default configuration
    let mut generation = TracegenProcessLoggerGeneration::terminal;
    let mut co_localizations = CoLocalizations::get_trivial_partition(gen_ctx.get_lf_num());
    let mut parent_folder = format!("tracegen_{:}",file_name);
    let mut files_prefix = "".to_string();
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
        }
    }
    return Ok(TraceGenProcessLogger::new(generation,
                                         co_localizations,
                                         parent_folder,
                                         files_prefix));
}

