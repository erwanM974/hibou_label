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


use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use graph_process_manager_loggers::graphviz::drawer::GraphVizProcessDrawer;
use graph_process_manager_loggers::graphviz::format::GraphVizProcessLoggerLayout;
use graph_process_manager_loggers::graphviz::logger::GenericGraphVizLogger;
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use pest::iterators::Pair;

#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};

use crate::loggers::graphviz::drawer::InteractionProcessDrawer;


pub fn parse_graphic_logger<Conf : AbstractProcessConfiguration>(logger_id : u32,
                                                                 file_name : &str,
                                                                 logger_kind_pair : Pair<Rule>)
        -> GenericGraphVizLogger<Conf>
        where
            InteractionProcessDrawer: GraphVizProcessDrawer<Conf> {

    // default configuration
    let mut output_format = GraphVizOutputFormat::svg;
    let mut layout = GraphVizProcessLoggerLayout::Vertical;
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
                        output_format = GraphVizOutputFormat::png;
                    },
                    Rule::GRAPHIC_LOGGER_format_svg => {
                        output_format = GraphVizOutputFormat::svg;
                    },
                    Rule::GRAPHIC_LOGGER_layout_vertical => {
                        layout = GraphVizProcessLoggerLayout::Vertical;
                    },
                    Rule::GRAPHIC_LOGGER_layout_horizontal => {
                        layout = GraphVizProcessLoggerLayout::Horizontal;
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
    let drawer = InteractionProcessDrawer::new(format!("graphviz_temp_l{:}", logger_id),
                                               int_repr_sd,
                                               int_repr_tt);
    GenericGraphVizLogger::new(Box::new(drawer),
                               output_format,
                               layout,
                               display_legend,
                               parent_folder,
                               output_file_name)
}
