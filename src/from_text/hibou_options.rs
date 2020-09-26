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
use std::fs;
use std::collections::{HashSet,HashMap};
use std::collections::btree_map::BTreeMap;
use std::path::Path;

use pest::iterators::Pair;

use crate::pest::Parser;

use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;


use crate::from_text::error::HibouParsingError;
use crate::process::log::ProcessLogger;

use crate::from_text::parser::*;
use crate::rendering::process::graphic_logger::GraphicProcessLogger;
use crate::process::hibou_process::*;


pub struct HibouOptions {
    pub loggers : Vec<Box<dyn ProcessLogger>>,
    pub strategy : HibouSearchStrategy,
    pub pre_filters : Vec<HibouPreFilter>
}



impl HibouOptions {
    pub fn default() -> HibouOptions {
        return HibouOptions{loggers:Vec::new(),strategy:HibouSearchStrategy::BFS,pre_filters:Vec::new()};
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum LoggerKinds {
    graphic
}

pub fn parse_hibou_options(option_pair : Pair<Rule>, file_name : &str) -> Result<HibouOptions,HibouParsingError> {
    let mut loggers : Vec<Box<dyn ProcessLogger>> = Vec::new();
    let mut strategy : HibouSearchStrategy = HibouSearchStrategy::BFS;
    let mut pre_filters : Vec<HibouPreFilter> = Vec::new();
    // ***
    let mut got_loggers   : bool = false;
    let mut got_strategy  : bool = false;
    let mut got_pre_filters : bool = false;
    // ***
    let mut declared_loggers : HashSet<LoggerKinds> = HashSet::new();
    // ***
    for option_decl_pair in option_pair.into_inner() {
        match option_decl_pair.as_rule() {
            Rule::OPTION_LOGGER_DECL => {
                if got_loggers {
                    return Err( HibouParsingError::HsfSetupError("several 'loggers=[X]' declared in the same '@X_option' section".to_string()));
                }
                got_loggers = true;
                // ***
                for logger_kind_pair in option_decl_pair.into_inner() {
                    match logger_kind_pair.as_rule() {
                        Rule::OPTION_GRAPHIC_LOGGER => {
                            if declared_loggers.contains(&LoggerKinds::graphic) {
                                return Err( HibouParsingError::HsfSetupError("several 'graphic' loggers declared in the same '@X_option' section".to_string()));
                            }
                            declared_loggers.insert( LoggerKinds::graphic );
                            loggers.push(Box::new(GraphicProcessLogger::new(file_name.to_string() ) ) );
                        },
                        _ => {
                            panic!("what rule then ? : {:?}", logger_kind_pair.as_rule() );
                        }
                    }
                }
            },
            Rule::OPTION_STRATEGY_DECL => {
                if got_strategy {
                    return Err( HibouParsingError::HsfSetupError("several 'strategy=X' declared in the same '@X_option' section".to_string()));
                }
                got_strategy = true;
                // ***
                let strategy_pair =  option_decl_pair.into_inner().next().unwrap();
                match strategy_pair.as_rule() {
                    Rule::OPTION_STRATEGY_BFS => {
                        strategy = HibouSearchStrategy::BFS;
                    },
                    Rule::OPTION_STRATEGY_DFS => {
                        strategy = HibouSearchStrategy::DFS;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", strategy_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_PREFILTERS_DECL => {
                if got_pre_filters {
                    return Err( HibouParsingError::HsfSetupError("several 'pre_filters=[X]' declared in the same '@X_option' section".to_string()));
                }
                got_pre_filters = true;
                // ***
                for pre_filter_pair in option_decl_pair.into_inner() {
                    match pre_filter_pair.as_rule() {
                        Rule::OPTION_PREFILTER_MAX_DEPTH => {
                            let content = pre_filter_pair.into_inner().next().unwrap();
                            let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                            let my_val : usize = content_str.parse::<usize>().unwrap();
                            pre_filters.push(HibouPreFilter::MaxProcessDepth(my_val));
                        },
                        Rule::OPTION_PREFILTER_MAX_LOOP_DEPTH  => {
                            let content = pre_filter_pair.into_inner().next().unwrap();
                            let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                            let my_val : u32 = content_str.parse::<u32>().unwrap();
                            pre_filters.push(HibouPreFilter::MaxLoopInstanciation(my_val));
                        },
                        Rule::OPTION_PREFILTER_MAX_NODE_NUMBER  => {
                            let content = pre_filter_pair.into_inner().next().unwrap();
                            let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                            let my_val : u32 = content_str.parse::<u32>().unwrap();
                            pre_filters.push(HibouPreFilter::MaxNodeNumber(my_val));
                        },
                        _ => {
                            panic!("what rule then ? : {:?}", pre_filter_pair.as_rule() );
                        }
                    }
                }
            },
            _ => {
                panic!("what rule then ? : {:?}", option_decl_pair.as_rule() );
            }
        }
    }
    return Ok( HibouOptions{loggers,strategy,pre_filters} );
}