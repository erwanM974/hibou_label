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


use crate::from_hfiles::error::HibouParsingError;

use crate::from_hfiles::parser::*;
use crate::loggers::graphic::conf::{GraphicProcessLoggerOutputKind, GraphicProcessLoggerLayout};
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;

use crate::from_hfiles::proc_options::loggers::parse_graphic_logger;
use crate::process::abstract_proc::common::HibouSearchStrategy;
use crate::process::ana_proc::anakind::{AnalysisKind, UseLocalAnalysis};
use crate::process::ana_proc::interface::conf::AnalysisConfig;
use crate::process::ana_proc::interface::filter::AnalysisFilter;
use crate::process::ana_proc::interface::logger::AnalysisLogger;
use crate::process::ana_proc::interface::priorities::AnalysisPriorities;
use crate::process::ana_proc::verdicts::GlobalVerdict;



pub struct HibouAnalyzeOptions {
    pub loggers : Vec<Box< dyn AnalysisLogger>>,
    pub strategy : HibouSearchStrategy,
    pub filters : Vec<AnalysisFilter>,
    pub priorities : AnalysisPriorities,
    pub ana_kind : AnalysisKind,
    pub local_analysis : UseLocalAnalysis,
    pub goal : Option<GlobalVerdict>
}

impl HibouAnalyzeOptions {
    pub fn new(loggers : Vec<Box< dyn AnalysisLogger>>,
               strategy : HibouSearchStrategy,
               filters : Vec<AnalysisFilter>,
               priorities : AnalysisPriorities,
               ana_kind : AnalysisKind,
               local_analysis : UseLocalAnalysis,
               goal : Option<GlobalVerdict>) -> HibouAnalyzeOptions {
        return HibouAnalyzeOptions{loggers,strategy,filters,priorities,ana_kind,local_analysis,goal};
    }

    pub fn default() -> HibouAnalyzeOptions {
        return HibouAnalyzeOptions{
            loggers:Vec::new(),
            strategy:HibouSearchStrategy::DFS,
            filters:Vec::new(),
            priorities:AnalysisPriorities::default(),
            ana_kind:AnalysisKind::Prefix,
            local_analysis:UseLocalAnalysis::Yes,
            goal:Some(GlobalVerdict::WeakPass)
        };
    }

    /* to be used recursively in global analyses when checking that its component local analysis are ok
       important to keep "local_analysis:UseLocalAnalysis::No" so we don't infinitely recurse!! */
    pub fn local_analyze() -> HibouAnalyzeOptions {
        return HibouAnalyzeOptions{
            loggers:Vec::new(),
            strategy:HibouSearchStrategy::DFS, // of course DFS is better here
            filters:Vec::new(),
            priorities:AnalysisPriorities::default(), // whatever works
            ana_kind:AnalysisKind::Prefix, // because only one canal, no need for hide or simulate
            local_analysis:UseLocalAnalysis::No, // No so we don't infinitely recurse
            goal:Some(GlobalVerdict::WeakPass), // it suffices to have a prefix
            };
    }

}


fn parse_filters(filters_decl_pair : Pair<Rule>) -> Result<Vec<AnalysisFilter>,HibouParsingError> {
    let mut filters : Vec<AnalysisFilter> = Vec::new();
    for filter_pair in filters_decl_pair.into_inner() {
        match filter_pair.as_rule() {
            Rule::OPTION_FILTER_MAX_DEPTH => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(AnalysisFilter::MaxProcessDepth(my_val));
            },
            Rule::OPTION_FILTER_MAX_LOOP_DEPTH  => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(AnalysisFilter::MaxLoopInstanciation(my_val));
            },
            Rule::OPTION_FILTER_MAX_NODE_NUMBER  => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(AnalysisFilter::MaxNodeNumber(my_val));
            },
            _ => {
                panic!("what rule then ? : {:?}", filter_pair.as_rule() );
            }
        }
    }
    return Ok(filters);
}

fn parse_priorities(priorities_decl_pair : Pair<Rule>) -> Result<AnalysisPriorities,HibouParsingError> {
    let mut emission : i32 = 0;
    let mut reception : i32 = 0;
    let mut multi_rdv : i32 = 0;
    let mut in_loop : i32 = 0;
    let mut hide : i32 = 0;
    let mut simu : i32 = 0;
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
            Rule::OPTION_PRIORITY_emission => {
                emission = priority_level;
            },
            Rule::OPTION_PRIORITY_reception => {
                reception = priority_level;
            },
            Rule::OPTION_PRIORITY_multi_rdv => {
                multi_rdv = priority_level;
            },
            Rule::OPTION_PRIORITY_loop => {
                in_loop = priority_level;
            },
            Rule::OPTION_PRIORITY_hide => {
                hide = priority_level;
            },
            Rule::OPTION_PRIORITY_simu => {
                simu = priority_level;
            },
            _ => {
                panic!("what rule then ? : {:?}", priority_kind_pair.as_rule() );
            }
        }
    }
    // ***
    let priorities = AnalysisPriorities::new(emission,reception,multi_rdv,in_loop,hide,simu);
    return Ok(priorities);
}

pub fn parse_analyze_options(option_pair : Pair<Rule>,
                             file_name : &str) -> Result<HibouAnalyzeOptions,HibouParsingError> {
    let mut loggers : Vec<Box<dyn AnalysisLogger>> = Vec::new();
    let mut strategy : HibouSearchStrategy = HibouSearchStrategy::BFS;
    let mut filters : Vec<AnalysisFilter> = Vec::new();
    let mut priorities = AnalysisPriorities::default();
    let mut ana_kind = AnalysisKind::Prefix;
    let mut local_analysis = UseLocalAnalysis::Yes;
    let mut goal = Some(GlobalVerdict::WeakPass);
    // ***
    for option_decl_pair in option_pair.into_inner() {
        match option_decl_pair.as_rule() {
            Rule::OPTION_LOGGER_DECL => {
                loggers = Vec::new();
                // ***
                for logger_kind_pair in option_decl_pair.into_inner() {
                    match logger_kind_pair.as_rule() {
                        Rule::OPTION_GRAPHIC_LOGGER => {
                            let glogger = parse_graphic_logger(file_name,logger_kind_pair);
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
            Rule::OPTION_FRONTIER_PRIORITIES_DECL => {
                match parse_priorities(option_decl_pair) {
                    Ok( got_priorities) => {
                        priorities = got_priorities;
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            Rule::OPTION_ANALYSIS_KIND_DECL => {
                let ana_kind_pair =  option_decl_pair.into_inner().next().unwrap();
                match ana_kind_pair.as_rule() {
                    Rule::OPTION_ANA_KIND_accept => {
                        ana_kind = AnalysisKind::Accept;
                    },
                    Rule::OPTION_ANA_KIND_prefix => {
                        ana_kind = AnalysisKind::Prefix;
                    },
                    Rule::OPTION_ANA_KIND_hide => {
                        ana_kind = AnalysisKind::Hide;
                    },
                    Rule::OPTION_ANA_KIND_simulate_prefix => {
                        ana_kind = AnalysisKind::Simulate(false);
                    },
                    Rule::OPTION_ANA_KIND_simulate_slice => {
                        ana_kind = AnalysisKind::Simulate(true);
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", ana_kind_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_LOCANA_yes => {
                local_analysis = UseLocalAnalysis::Yes;
            },
            Rule::OPTION_LOCANA_no => {
                local_analysis = UseLocalAnalysis::No;
            },
            Rule::OPTION_GOAL_DECL => {
                let goal_pair =  option_decl_pair.into_inner().next().unwrap();
                match goal_pair.as_rule() {
                    Rule::OPTION_GOAL_pass => {
                        goal = Some( GlobalVerdict::Pass );
                    },
                    Rule::OPTION_GOAL_weakpass => {
                        goal = Some( GlobalVerdict::WeakPass );
                    },
                    Rule::OPTION_GOAL_none => {
                        goal = None;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", goal_pair.as_rule() );
                    }
                }
            },
            _ => {
                panic!("what rule then ? : {:?}", option_decl_pair.as_rule() );
            }
        }
    }
    // ***
    let hoptions = HibouAnalyzeOptions{loggers,strategy,filters,priorities,ana_kind,local_analysis,goal};
    return Ok(hoptions);
}