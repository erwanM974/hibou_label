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


use pest::iterators::Pair;

use crate::core::general_context::GeneralContext;
use crate::io::input::error::HibouParsingError;
use crate::io::input::hcf::proc_options::loggers::{parse_graphic_logger};
use crate::process::abstract_proc::common::HibouSearchStrategy;
use crate::process::abstract_proc::manager::GenericProcessPriorities;
use crate::process::canon_proc::interface::logger::CanonizationLogger;
use crate::process::canon_proc::interface::conf::CanonizationConfig;
use crate::process::canon_proc::interface::filter::CanonizationFilter;

#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};
use crate::process::canon_proc::interface::priorities::CanonizationPriorities;


pub struct HibouCanonizeOptions {
    pub loggers : Vec<Box<dyn CanonizationLogger>>,
    pub strategy : HibouSearchStrategy,
    pub filters : Vec<CanonizationFilter>,
    pub priorities : GenericProcessPriorities<CanonizationConfig>,
    pub search_all : bool
}



impl HibouCanonizeOptions {
    pub fn new(loggers : Vec<Box<dyn CanonizationLogger>>,
               strategy : HibouSearchStrategy,
               filters : Vec<CanonizationFilter>,
               priorities : GenericProcessPriorities<CanonizationConfig>,
               search_all : bool) -> HibouCanonizeOptions {
        return HibouCanonizeOptions{loggers,strategy,filters,priorities,search_all};
    }

    pub fn default() -> HibouCanonizeOptions {
        return HibouCanonizeOptions::new(Vec::new(),
                                         HibouSearchStrategy::DFS,
                                         vec![],
                                         GenericProcessPriorities::Specific(CanonizationPriorities::default()),
                                         false);
    }

}




pub fn parse_canonize_options(gen_ctx: &GeneralContext,
                             option_pair : Pair<Rule>,
                             file_name : &str) -> Result<HibouCanonizeOptions,HibouParsingError> {
    let mut loggers : Vec<Box<dyn CanonizationLogger>> = Vec::new();
    let mut strategy : HibouSearchStrategy = HibouSearchStrategy::BFS;
    let mut filters : Vec<CanonizationFilter> = Vec::new();
    let mut priorities : GenericProcessPriorities<CanonizationConfig> = GenericProcessPriorities::Specific(CanonizationPriorities::default());
    let mut search_all = false;
    // ***
    for option_decl_pair in option_pair.into_inner() {
        match option_decl_pair.as_rule() {
            Rule::OPTION_LOGGER_DECL => {
                loggers = Vec::new();
                // ***
                let mut logger_id : u32 = 0;
                for logger_kind_pair in option_decl_pair.into_inner() {
                    logger_id += 1;
                    match logger_kind_pair.as_rule() {
                        Rule::OPTION_GRAPHIC_LOGGER => {
                            let glogger = parse_graphic_logger(logger_id,file_name,logger_kind_pair);
                            loggers.push( Box::new(glogger));
                        },
                        _ => {
                            panic!("what rule then ? : {:?}", logger_kind_pair.as_rule() );
                        }
                    }
                }
            },
            Rule::OPTION_STRATEGY_DECL => {
                let strategy_pair =  option_decl_pair.into_inner().next().unwrap();
                match strategy_pair.as_rule() {
                    Rule::OPTION_STRATEGY_BFS => {
                        strategy = HibouSearchStrategy::BFS;
                    },
                    Rule::OPTION_STRATEGY_DFS => {
                        strategy = HibouSearchStrategy::DFS;
                    },
                    Rule::OPTION_STRATEGY_HCS => {
                        strategy = HibouSearchStrategy::HCS;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", strategy_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_FILTERS_DECL => {
                match parse_filters(option_decl_pair) {
                    Ok( got_filters) => {
                        filters = got_filters;
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            Rule::OPTION_PRIORITIES_DECL => {
                let inner : Pair<Rule> = option_decl_pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::OPTION_PRIORITY_SPECIFIC => {
                        match parse_specific_priorities(inner) {
                            Ok( got_priorities) => {
                                priorities = GenericProcessPriorities::Specific(got_priorities);
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    Rule::OPTION_PRIORITY_RANDOM => {
                        priorities = GenericProcessPriorities::Random;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", inner.as_rule() );
                    }
                }
            },
            Rule::OPTION_CANON_searchall_yes => {
                search_all = true;
            },
            Rule::OPTION_CANON_searchall_no => {
                search_all = false;
            },
            _ => {
                panic!("what rule then ? : {:?}", option_decl_pair.as_rule() );
            }
        }
    }
    // ***
    let hoptions = HibouCanonizeOptions::new(loggers,strategy,filters,priorities,search_all);
    return Ok(hoptions);
}



fn parse_filters(filters_decl_pair : Pair<Rule>) -> Result<Vec<CanonizationFilter>,HibouParsingError> {
    let mut filters : Vec<CanonizationFilter> = Vec::new();
    for filter_pair in filters_decl_pair.into_inner() {
        match filter_pair.as_rule() {
            Rule::OPTION_FILTER_MAX_DEPTH => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(CanonizationFilter::MaxProcessDepth(my_val));
            },
            Rule::OPTION_FILTER_MAX_NODE_NUMBER  => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(CanonizationFilter::MaxNodeNumber(my_val));
            },
            Rule::OPTION_FILTER_MAX_LOOP_DEPTH  => {
                return Err(HibouParsingError::ProcessFilterError("found max loop depth filter in Canonization".to_string()));
            },
            _ => {
                panic!("what rule then ? : {:?}", filter_pair.as_rule() );
            }
        }
    }
    return Ok(filters);
}




fn parse_specific_priorities(priorities_decl_pair : Pair<Rule>) -> Result<CanonizationPriorities,HibouParsingError> {
    let mut simpl : i32 = 0;
    let mut flush : i32 = 0;
    let mut invert : i32 = 0;
    let mut deduplicate : i32 = 0;
    let mut factorize : i32 = 0;
    let mut defactorize : i32 = 0;
    // ***
    for priority_pair in priorities_decl_pair.into_inner() {
        let mut priority_contents = priority_pair.into_inner();
        let priority_kind_pair = priority_contents.next().unwrap();
        // ***
        let priority_level_pair = priority_contents.next().unwrap();
        let priority_level_str : String = priority_level_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        let priority_level : i32 = priority_level_str.parse::<i32>().unwrap();
        // ***
        match priority_kind_pair.as_rule() {
            Rule::OPTION_PRIORITY_simpl => {
                simpl = priority_level;
            },
            Rule::OPTION_PRIORITY_flush => {
                flush = priority_level;
            },
            Rule::OPTION_PRIORITY_invert => {
                invert = priority_level;
            },
            Rule::OPTION_PRIORITY_deduplicate => {
                deduplicate = priority_level;
            },
            Rule::OPTION_PRIORITY_factorize => {
                factorize = priority_level;
            },
            Rule::OPTION_PRIORITY_defactorize => {
                defactorize = priority_level;
            },
            // ***
            Rule::OPTION_PRIORITY_emission => {
                return Err(HibouParsingError::ProcessPriorityError("found emission priority in Canonization".to_string()));
            },
            Rule::OPTION_PRIORITY_reception => {
                return Err(HibouParsingError::ProcessPriorityError("found reception priority in Canonization".to_string()));
            },
            Rule::OPTION_PRIORITY_multi_rdv => {
                return Err(HibouParsingError::ProcessPriorityError("found multi-rdv priority in Canonization".to_string()));
            },
            Rule::OPTION_PRIORITY_loop => {
                return Err(HibouParsingError::ProcessPriorityError("found loop priority in Canonization".to_string()));
            },
            // ***
            Rule::OPTION_PRIORITY_hide => {
                return Err(HibouParsingError::ProcessPriorityError("found hide priority in Canonization".to_string()));
            },
            Rule::OPTION_PRIORITY_simu => {
                return Err(HibouParsingError::ProcessPriorityError("found simu priority in Canonization".to_string()));
            },
            // ***
            _ => {
                panic!("what rule then ? : {:?}", priority_kind_pair.as_rule() );
            }
        }
    }
    // ***
    let priorities = CanonizationPriorities::new(simpl,flush,invert,deduplicate,factorize,defactorize);
    return Ok(priorities);
}