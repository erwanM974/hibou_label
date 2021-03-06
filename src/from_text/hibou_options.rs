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
use crate::rendering::process::graphic_logger::{GraphicProcessLoggerOutputKind,GraphicProcessLoggerLayout,GraphicProcessLogger};
use crate::process::hibou_process::*;
use crate::from_text::hsf_file::ProcessKind;

use crate::process::priorities::ProcessPriorities;
use crate::process::verdicts::GlobalVerdict;
use crate::process::anakind::AnalysisKind;

pub struct HibouOptions {
    pub loggers : Vec<Box<dyn ProcessLogger>>,
    pub strategy : HibouSearchStrategy,
    pub pre_filters : Vec<HibouPreFilter>,
    pub ana_kind : Option<AnalysisKind>,
    pub use_locfront : bool,
    pub goal : Option<GlobalVerdict>,
    pub frontier_priorities : ProcessPriorities
}



impl HibouOptions {
    pub fn new(loggers : Vec<Box<dyn ProcessLogger>>,
               strategy : HibouSearchStrategy,
               pre_filters : Vec<HibouPreFilter>,
               ana_kind : Option<AnalysisKind>,
               use_locfront : bool,
               goal:Option<GlobalVerdict>,
               frontier_priorities : ProcessPriorities) -> HibouOptions {
        return HibouOptions{loggers,strategy,pre_filters,ana_kind,use_locfront,goal,frontier_priorities};
    }

    pub fn default_explore() -> HibouOptions {
        return HibouOptions{loggers:Vec::new(),
            strategy:HibouSearchStrategy::BFS,
            pre_filters:vec![HibouPreFilter::MaxLoopInstanciation(1)],
            ana_kind:None,
            goal:None,
            use_locfront:false,
            frontier_priorities:ProcessPriorities::new(0,0,0, None, -2, -2)};
    }

    pub fn default_analyze() -> HibouOptions {
        return HibouOptions{loggers:Vec::new(),
            strategy:HibouSearchStrategy::BFS,
            pre_filters:Vec::new(),
            ana_kind:Some(AnalysisKind::Prefix),
            use_locfront:true,
            goal:Some(GlobalVerdict::Pass),
            frontier_priorities:ProcessPriorities::new(0,0,0, None, -2, -2)};
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum LoggerKinds {
    graphic
}

fn parse_priorities(priority_pair : Pair<Rule>,
                    pp : &mut ProcessPriorities,
                    frontier_pp : bool) -> Result<(),HibouParsingError> {
    let mut priority_contents = priority_pair.into_inner();
    let priority_kind_pair = priority_contents.next().unwrap();
    // ***
    let priority_level_pair = priority_contents.next().unwrap();
    let priority_level_str : String = priority_level_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    let priority_level : i32 = priority_level_str.parse::<i32>().unwrap();
    // ***
    match priority_kind_pair.as_rule() {
        Rule::OPTION_PRIORITY_emission => {
            pp.emission = priority_level;
        },
        Rule::OPTION_PRIORITY_reception => {
            pp.reception = priority_level;
        },
        Rule::OPTION_PRIORITY_loop => {
            pp.in_loop = priority_level;
        },
        Rule::OPTION_PRIORITY_hide => {
            pp.hide = priority_level;
        },
        Rule::OPTION_PRIORITY_simu => {
            pp.simulate = priority_level;
        },
        Rule::OPTION_PRIORITY_step => {
            if frontier_pp {
                return Err( HibouParsingError::ProcessPriorityError("cannot specify \"step\" in frontier priorities".to_string()) );
            } else {
                pp.step = Some( priority_level );
            }
        }
        _ => {
            panic!("what rule then ? : {:?}", priority_kind_pair.as_rule() );
        }
    }
    return Ok(());
}

pub fn parse_hibou_options(option_pair : Pair<Rule>, file_name : &str, process_kind : &ProcessKind) -> Result<HibouOptions,HibouParsingError> {
    let mut loggers : Vec<Box<dyn ProcessLogger>> = Vec::new();
    let mut strategy : HibouSearchStrategy = HibouSearchStrategy::BFS;
    let mut frontier_priorities = ProcessPriorities::new(0,0,0, None, -2, -2);
    let mut pre_filters : Vec<HibouPreFilter> = Vec::new();
    let mut ana_kind_opt : Option<AnalysisKind> = Some(AnalysisKind::Prefix);
    let mut use_locfront = true;
    let mut goal : Option<GlobalVerdict> = Some(GlobalVerdict::WeakPass);
    // ***
    let mut got_loggers   : bool = false;
    let mut got_strategy  : bool = false;
    let mut got_frontier_priorities : bool = false;
    let mut got_pre_filters : bool = false;
    let mut got_ana_kind : bool = false;
    let mut got_locfront : bool = false;
    let mut got_goal : bool = false;
    // ***
    let mut declared_loggers : HashSet<LoggerKinds> = HashSet::new();
    // ***
    for option_decl_pair in option_pair.into_inner() {
        match option_decl_pair.as_rule() {
            Rule::OPTION_LOCFRONT_true => {
                if got_locfront {
                    return Err( HibouParsingError::HsfSetupError("several 'use_locfront=X' declared in the same '@X_option' section".to_string()));
                }
                got_locfront = true;
                // ***
                use_locfront = true;
            },
            Rule::OPTION_LOCFRONT_false => {
                if got_locfront {
                    return Err( HibouParsingError::HsfSetupError("several 'use_locfront=X' declared in the same '@X_option' section".to_string()));
                }
                got_locfront = true;
                // ***
                use_locfront = false;
            },
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
                            let graphic_logger_opts_pair = logger_kind_pair.into_inner().next();
                            match graphic_logger_opts_pair {
                                None => {
                                    loggers.push(Box::new(GraphicProcessLogger::new(file_name.to_string(),
                                                                                    GraphicProcessLoggerOutputKind::png,
                                                                                    GraphicProcessLoggerLayout::vertical ) ) );
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
                                                panic!("what rule then ? : {:?}", opt_pair.as_rule() );
                                            }
                                        }
                                    }
                                    loggers.push(Box::new(GraphicProcessLogger::new(file_name.to_string(),output_kind,layout_kind ) ) );
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
                    Rule::OPTION_STRATEGY_GFS => {
                        let mut gfs_priorities = ProcessPriorities::new(0,0,0,Some(1), -2, -2);
                        match strategy_pair.into_inner().next() {
                            None => {},
                            Some(gfs_opts_pair) => {
                                match gfs_opts_pair.as_rule() {
                                    Rule::OPTION_STRATEGY_GFS_opts => {
                                        for priority_pair in gfs_opts_pair.into_inner() {
                                            parse_priorities(priority_pair,&mut gfs_priorities,false);
                                        }
                                    },
                                    _ => {
                                        panic!("what rule then ? : {:?}", gfs_opts_pair.as_rule() );
                                    }
                                }
                            }
                        }
                        strategy = HibouSearchStrategy::GFS(gfs_priorities);
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", strategy_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_FRONTIER_PRIORITIES_DECL => {
                if got_frontier_priorities {
                    return Err( HibouParsingError::HsfSetupError("several 'frontier_priorities=X' declared in the same '@X_option' section".to_string()));
                }
                got_frontier_priorities = true;
                // ***
                for priority_pair in option_decl_pair.into_inner() {
                    parse_priorities(priority_pair,&mut frontier_priorities,true);
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
            Rule::OPTION_ANALYSIS_KIND_DECL => {
                if got_ana_kind {
                    return Err( HibouParsingError::HsfSetupError("several 'ana_kind=X' declared in the same '@X_option' section".to_string()));
                }
                got_ana_kind = true;
                // ***
                let ana_kind_pair =  option_decl_pair.into_inner().next().unwrap();
                match ana_kind_pair.as_rule() {
                    Rule::OPTION_ANA_KIND_accept => {
                        ana_kind_opt = Some( AnalysisKind::Accept );
                    },
                    Rule::OPTION_ANA_KIND_prefix => {
                        ana_kind_opt = Some( AnalysisKind::Prefix );
                    },
                    Rule::OPTION_ANA_KIND_hide => {
                        ana_kind_opt = Some( AnalysisKind::Hide );
                    },
                    Rule::OPTION_ANA_KIND_simulate_prefix => {
                        ana_kind_opt = Some( AnalysisKind::Simulate(false) );
                    },
                    Rule::OPTION_ANA_KIND_simulate_slice => {
                        ana_kind_opt = Some( AnalysisKind::Simulate(true) );
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", ana_kind_pair.as_rule() );
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
    match process_kind {
        ProcessKind::Analyze => {
            return Ok( HibouOptions::new(loggers,strategy,pre_filters,ana_kind_opt, use_locfront, goal,frontier_priorities) );
        },
        _ => {
            return Ok( HibouOptions::new(loggers,strategy,pre_filters,None, false, None,frontier_priorities) );
        }
    }
}