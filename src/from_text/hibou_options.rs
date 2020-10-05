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
use crate::rendering::process::graphic_logger::{GraphicProcessLoggerKind,GraphicProcessLogger};
use crate::process::hibou_process::*;
use crate::from_text::hsf_file::ProcessKind;

use crate::process::process_manager::ProcessPriorities;
use crate::process::verdicts::GlobalVerdict;

pub struct HibouOptions {
    pub loggers : Vec<Box<dyn ProcessLogger>>,
    pub strategy : HibouSearchStrategy,
    pub pre_filters : Vec<HibouPreFilter>,
    pub sem_kind : Option<SemanticKind>,
    pub goal : Option<GlobalVerdict>,
    pub priorities : ProcessPriorities
}



impl HibouOptions {
    pub fn new(loggers : Vec<Box<dyn ProcessLogger>>,
               strategy : HibouSearchStrategy,
               pre_filters : Vec<HibouPreFilter>,
               sem_kind : Option<SemanticKind>,
               goal:Option<GlobalVerdict>,
               priorities : ProcessPriorities) -> HibouOptions {
        return HibouOptions{loggers,strategy,pre_filters,sem_kind,goal,priorities};
    }

    pub fn default_explore() -> HibouOptions {
        return HibouOptions{loggers:Vec::new(),
            strategy:HibouSearchStrategy::BFS,
            pre_filters:vec![HibouPreFilter::MaxLoopInstanciation(1)],
            sem_kind:None,
            goal:None,
            priorities:ProcessPriorities::new(0,0,0)};
    }

    pub fn default_analyze() -> HibouOptions {
        return HibouOptions{loggers:Vec::new(),
            strategy:HibouSearchStrategy::BFS,
            pre_filters:Vec::new(),
            sem_kind:Some(SemanticKind::Prefix),
            goal:Some(GlobalVerdict::Pass),
            priorities:ProcessPriorities::new(0,0,0)};
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum LoggerKinds {
    graphic
}

pub fn parse_hibou_options(option_pair : Pair<Rule>, file_name : &str, process_kind : &ProcessKind) -> Result<HibouOptions,HibouParsingError> {
    let mut loggers : Vec<Box<dyn ProcessLogger>> = Vec::new();
    let mut strategy : HibouSearchStrategy = HibouSearchStrategy::BFS;
    let mut priorities = ProcessPriorities::new(0,0,0);
    let mut pre_filters : Vec<HibouPreFilter> = Vec::new();
    let mut semantics : Option<SemanticKind> = None;
    let mut goal : Option<GlobalVerdict> = None;
    // ***
    let mut got_loggers   : bool = false;
    let mut got_strategy  : bool = false;
    let mut got_priorities : bool = false;
    let mut got_pre_filters : bool = false;
    let mut got_semantics : bool = false;
    let mut got_goal : bool = false;
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
                            let graphic_logger_pair = logger_kind_pair.into_inner().next();
                            match graphic_logger_pair {
                                None => {
                                    loggers.push(Box::new(GraphicProcessLogger::new(file_name.to_string(),GraphicProcessLoggerKind::png ) ) );
                                },
                                Some(graphic_logger_kind_pair) => {
                                    match graphic_logger_kind_pair.as_rule() {
                                        Rule::GRAPHIC_LOGGER_KIND_png => {
                                            loggers.push(Box::new(GraphicProcessLogger::new(file_name.to_string(),GraphicProcessLoggerKind::png ) ) );
                                        },
                                        Rule::GRAPHIC_LOGGER_KIND_svg => {
                                            loggers.push(Box::new(GraphicProcessLogger::new(file_name.to_string(),GraphicProcessLoggerKind::svg ) ) );
                                        },
                                        _ => {
                                            panic!("what rule then ? : {:?}", graphic_logger_kind_pair.as_rule() );
                                        }
                                    }
                                }
                            }
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
            Rule::OPTION_PRIORITIES_DECL => {
                if got_priorities {
                    return Err( HibouParsingError::HsfSetupError("several 'priorities=X' declared in the same '@X_option' section".to_string()));
                }
                got_priorities = true;
                // ***
                for priority_pair in option_decl_pair.into_inner() {
                    let mut priority_contents = priority_pair.into_inner();
                    let priority_kind_pair = priority_contents.next().unwrap();
                    // ***
                    let priority_level_pair = priority_contents.next().unwrap();
                    let priority_level_str : String = priority_level_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    let priority_level : i32 = priority_level_str.parse::<i32>().unwrap();
                    // ***
                    match priority_kind_pair.as_rule() {
                        Rule::OPTION_PRIORITTY_emission => {
                            priorities.emission = priority_level;
                        },
                        Rule::OPTION_PRIORITTY_reception => {
                            priorities.reception = priority_level;
                        },
                        Rule::OPTION_PRIORITY_loop => {
                            priorities.in_loop = priority_level;
                        },
                        _ => {
                            panic!("what rule then ? : {:?}", priority_kind_pair.as_rule() );
                        }
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
                            let my_val : u32 = content_str.parse::<u32>().unwrap();
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
            Rule::OPTION_SEMANTICS_DECL => {
                if got_semantics {
                    return Err( HibouParsingError::HsfSetupError("several 'semantics=X' declared in the same '@X_option' section".to_string()));
                }
                got_semantics = true;
                // ***
                let semantics_pair =  option_decl_pair.into_inner().next().unwrap();
                match semantics_pair.as_rule() {
                    Rule::OPTION_SEMANTICS_accept => {
                        semantics = Some( SemanticKind::Accept );
                    },
                    Rule::OPTION_SEMANTICS_prefix => {
                        semantics = Some( SemanticKind::Prefix );
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", semantics_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_GOAL_DECL => {
                if got_goal {
                    return Err( HibouParsingError::HsfSetupError("several 'goal=X' declared in the same '@X_option' section".to_string()));
                }
                got_goal = true;
                // ***
                let goal_pair =  option_decl_pair.into_inner().next().unwrap();
                match goal_pair.as_rule() {
                    Rule::OPTION_GOAL_pass => {
                        goal = Some( GlobalVerdict::Pass );
                    },
                    Rule::OPTION_GOAL_weakpass => {
                        goal = Some( GlobalVerdict::WeakPass );
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
    match process_kind {
        ProcessKind::Analyze => {
            let ana_sem : SemanticKind;
            match semantics {
                None => {
                    ana_sem = SemanticKind::Prefix;
                },
                Some( sem_in ) => {
                    ana_sem = sem_in;
                }
            }
            let ana_goal : GlobalVerdict;
            match goal {
                None => {
                    ana_goal = GlobalVerdict::Pass;
                },
                Some( goal_in ) => {
                    ana_goal = goal_in;
                }
            }
            // ***
            return Ok( HibouOptions::new(loggers,strategy,pre_filters,Some(ana_sem), Some(ana_goal),priorities) );
        },
        _ => {
            return Ok( HibouOptions::new(loggers,strategy,pre_filters,None, None,priorities) );
        }
    }
}