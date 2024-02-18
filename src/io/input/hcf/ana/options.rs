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


use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::handler::filter::AbstractFilter;
use graph_process_manager_core::manager::logger::AbstractProcessLogger;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use pest::iterators::Pair;
use crate::core::general_context::GeneralContext;

use crate::io::input::error::HibouParsingError;
use crate::io::input::hcf::loggers::graphviz::parse_graphic_logger;
use crate::io::input::hcf::loggers::tracegen::parse_tracegen_logger;


#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};


use crate::process::ana::conf::AnalysisConfig;
use crate::process::ana::filter::elim::AnalysisFilterEliminationKind;
use crate::process::ana::filter::filter::{AnalysisFilter, AnalysisFilterCriterion};
use crate::process::ana::param::anakind::{AnalysisKind, SimulationActionCriterion, SimulationConfiguration, SimulationLoopCriterion};
use crate::process::ana::param::param::{AnalysisParameterization, LocalAnalysisLifelineSelectionPolicy, LocalAnalysisParameterization};
use crate::process::ana::priorities::AnalysisPriorities;
use crate::process::ana::verdict::global::AnalysisGlobalVerdict;


pub struct HibouAnalyzeOptions {
    pub loggers : Vec<Box< dyn AbstractProcessLogger<AnalysisConfig>>>,
    pub strategy : QueueSearchStrategy,
    pub filters : Vec<Box<dyn AbstractFilter<AnalysisFilterCriterion,AnalysisFilterEliminationKind>>>,
    pub priorities : GenericProcessPriorities<AnalysisPriorities>,
    pub use_memoization : bool,
    pub goal : Option<AnalysisGlobalVerdict>,
    pub ana_param : AnalysisParameterization
}

impl HibouAnalyzeOptions {
    pub fn new(loggers : Vec<Box< dyn AbstractProcessLogger<AnalysisConfig>>>,
               strategy : QueueSearchStrategy,
               filters : Vec<Box<dyn AbstractFilter<AnalysisFilterCriterion,AnalysisFilterEliminationKind>>>,
               priorities : GenericProcessPriorities<AnalysisPriorities>,
               ana_param : AnalysisParameterization,
               use_memoization : bool,
               goal : Option<AnalysisGlobalVerdict>) -> HibouAnalyzeOptions {
        HibouAnalyzeOptions{loggers,strategy,filters,priorities,use_memoization,goal,ana_param}
    }

    pub fn default() -> HibouAnalyzeOptions {
        let default_param = AnalysisParameterization::new(
            AnalysisKind::Prefix,
            Some(LocalAnalysisParameterization::new(LocalAnalysisLifelineSelectionPolicy::OnlyOnImpactedByLastStep,None)),
            false);
        HibouAnalyzeOptions::new(
            vec![],
            QueueSearchStrategy::DFS,
            vec![],
            GenericProcessPriorities::new(AnalysisPriorities::default(),false),
            default_param,
            true,
            Some(AnalysisGlobalVerdict::WeakPass)
        )
    }
}


pub fn parse_analyze_options(gen_ctx : &GeneralContext,
                             option_pair : Pair<Rule>,
                             file_name : &str) -> Result<HibouAnalyzeOptions,HibouParsingError> {
    let mut loggers : Vec<Box< dyn AbstractProcessLogger<AnalysisConfig>>> = Vec::new();
    let mut strategy : QueueSearchStrategy = QueueSearchStrategy::BFS;
    let mut filters : Vec<Box<dyn AbstractFilter<AnalysisFilterCriterion,AnalysisFilterEliminationKind>>> = Vec::new();
    let mut priorities : GenericProcessPriorities<AnalysisPriorities> = GenericProcessPriorities::new(AnalysisPriorities::default(),false);
    let mut ana_kind = AnalysisKind::Prefix;
    let mut use_locana = true;
    let mut locana_param = LocalAnalysisParameterization::new(LocalAnalysisLifelineSelectionPolicy::OnlyOnImpactedByLastStep,None);
    let mut use_partial_order_reduction = false;
    let mut use_memoization = true;
    let mut goal = Some(AnalysisGlobalVerdict::WeakPass);
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
                        Rule::OPTION_TRACEGEN_LOGGER => {
                            match parse_tracegen_logger(logger_id, gen_ctx,file_name,logger_kind_pair) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( tlogger ) => {
                                    loggers.push( Box::new(tlogger));
                                }
                            }
                        },
                        Rule::OPTION_NFAIT_LOGGER => {
                            return Err(HibouParsingError::HcfSetupError("cannot use NFAIT logger for trace analysis".to_string()));
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
                        strategy = QueueSearchStrategy::BFS;
                    },
                    Rule::OPTION_STRATEGY_DFS => {
                        strategy = QueueSearchStrategy::DFS;
                    },
                    Rule::OPTION_STRATEGY_HCS => {
                        strategy = QueueSearchStrategy::HCS;
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
                    Rule::OPTION_PRIORITY => {
                        match parse_priorities(inner) {
                            Ok( got_priorities ) => {
                                priorities = got_priorities;
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", inner.as_rule() );
                    }
                }
            },
            Rule::OPTION_MEMOIZE => {
                let as_bool_pair = option_decl_pair.into_inner().next().unwrap();
                match as_bool_pair.as_rule() {
                    Rule::HIBOU_true => {
                        use_memoization = true;
                    },
                    Rule::HIBOU_false => {
                        use_memoization = false;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", as_bool_pair.as_rule() );
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
                    Rule::OPTION_ANA_KIND_eliminate => {
                        ana_kind = AnalysisKind::Eliminate;
                    },
                    Rule::OPTION_ANA_KIND_simulate => {
                        let mut inner = ana_kind_pair.into_inner();
                        match inner.next() {
                            None => {
                                let sim_config = SimulationConfiguration::new(false,
                                                                              false,
                                                                              false,
                                                                              SimulationLoopCriterion::MaxDepth,
                                                                              SimulationActionCriterion::None);
                                ana_kind = AnalysisKind::Simulate(sim_config);
                            },
                            Some( sim_config_decl_pair) => {
                                match parse_simulation_config(sim_config_decl_pair) {
                                    Ok( sim_config) => {
                                        ana_kind = AnalysisKind::Simulate(sim_config);
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }
                            }
                        }
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", ana_kind_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_PARTIAL_ORDER => {
                let as_bool_pair = option_decl_pair.into_inner().next().unwrap();
                match as_bool_pair.as_rule() {
                    Rule::HIBOU_true => {
                        use_partial_order_reduction = true;
                    },
                    Rule::HIBOU_false => {
                        use_partial_order_reduction = false;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", as_bool_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_LOCANA => {
                match parse_local_analyses_config(option_decl_pair) {
                    Err(e) => {return Err(e);},
                    Ok(got) => {
                        if let Some(got_locana_param) = got {
                            use_locana = true;
                            locana_param = got_locana_param;
                        } else {
                            use_locana = false;
                        }
                    }
                }
            },
            Rule::OPTION_GOAL_DECL => {
                let goal_pair =  option_decl_pair.into_inner().next().unwrap();
                match goal_pair.as_rule() {
                    Rule::OPTION_GOAL_pass => {
                        goal = Some( AnalysisGlobalVerdict::Pass );
                    },
                    Rule::OPTION_GOAL_weakpass => {
                        goal = Some( AnalysisGlobalVerdict::WeakPass );
                    },
                    Rule::HIBOU_none => {
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
    let locana_param = if use_locana {
        Some(locana_param)
    } else {
        None
    };
    let param = AnalysisParameterization::new(ana_kind,locana_param, use_partial_order_reduction);
    let hoptions = HibouAnalyzeOptions::new(loggers,strategy,filters,priorities,param,use_memoization,goal);
    return Ok(hoptions);
}


fn parse_filters(filters_decl_pair : Pair<Rule>) -> Result<Vec<Box<dyn AbstractFilter<AnalysisFilterCriterion,AnalysisFilterEliminationKind>>>,HibouParsingError> {
    let mut filters : Vec<Box<dyn AbstractFilter<AnalysisFilterCriterion,AnalysisFilterEliminationKind>>> = Vec::new();
    for filter_pair in filters_decl_pair.into_inner() {
        match filter_pair.as_rule() {
            Rule::OPTION_FILTER_MAX_DEPTH => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(Box::new(AnalysisFilter::MaxProcessDepth(my_val)));
            },
            Rule::OPTION_FILTER_MAX_LOOP_DEPTH  => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(Box::new(AnalysisFilter::MaxLoopInstanciation(my_val)));
            },
            Rule::OPTION_FILTER_MAX_NODE_NUMBER  => {
                let content = filter_pair.into_inner().next().unwrap();
                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                let my_val : u32 = content_str.parse::<u32>().unwrap();
                filters.push(Box::new(AnalysisFilter::MaxNodeNumber(my_val)));
            },
            _ => {
                panic!("what rule then ? : {:?}", filter_pair.as_rule() );
            }
        }
    }
    return Ok(filters);
}

fn parse_priorities(priorities_decl_pair : Pair<Rule>) -> Result<GenericProcessPriorities<AnalysisPriorities>,HibouParsingError> {
    let mut randomize : bool = false;
    let mut emission : i32 = 0;
    let mut reception : i32 = 0;
    let mut multi_rdv : i32 = 0;
    let mut in_loop : i32 = 0;
    let mut elim : i32 = 0;
    let mut simu : i32 = 0;
    // ***
    for priority_pair in priorities_decl_pair.into_inner() {
        match priority_pair.as_rule() {
            Rule::OPTION_PRIORITY_RANDOMIZE => {
                let bool_pair = priority_pair.into_inner().next().unwrap();
                match bool_pair.as_rule() {
                    Rule::HIBOU_true => {
                        randomize = true;
                    },
                    Rule::HIBOU_false => {
                        randomize = false;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", bool_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_PRIORITY_SPECIFIC_elt => {
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
                    // ***
                    Rule::OPTION_PRIORITY_elim => {
                        elim = priority_level;
                    },
                    Rule::OPTION_PRIORITY_simu => {
                        simu = priority_level;
                    },
                    // ***
                    Rule::OPTION_PRIORITY_simpl => {
                        return Err(HibouParsingError::ProcessPriorityError("found simpl priority in Analysis".to_string()));
                    },
                    Rule::OPTION_PRIORITY_flush => {
                        return Err(HibouParsingError::ProcessPriorityError("found flush priority in Analysis".to_string()));
                    },
                    Rule::OPTION_PRIORITY_invert => {
                        return Err(HibouParsingError::ProcessPriorityError("found invert priority in Analysis".to_string()));
                    },
                    Rule::OPTION_PRIORITY_deduplicate => {
                        return Err(HibouParsingError::ProcessPriorityError("found deduplicate priority in Analysis".to_string()));
                    },
                    Rule::OPTION_PRIORITY_factorize => {
                        return Err(HibouParsingError::ProcessPriorityError("found factorize priority in Analysis".to_string()));
                    },
                    Rule::OPTION_PRIORITY_defactorize => {
                        return Err(HibouParsingError::ProcessPriorityError("found defactorize priority in Analysis".to_string()));
                    },
                    // ***
                    _ => {
                        panic!("what rule then ? : {:?}", priority_kind_pair.as_rule() );
                    }
                }
            },
            _ => {
                panic!("what rule then ? : {:?}", priority_pair.as_rule() );
            }
        }
    }
    // ***
    let specific = AnalysisPriorities::new(emission,reception,multi_rdv,in_loop,elim,simu);
    return Ok(GenericProcessPriorities::new(specific,randomize));
}

fn parse_local_analyses_config(locana_pair : Pair<Rule>) -> Result<Option<LocalAnalysisParameterization>,HibouParsingError> {
    let choice_pair = locana_pair.into_inner().next().unwrap();
    match choice_pair.as_rule() {
        Rule::HIBOU_none => {
            return Ok(None);
        },
        Rule::OPTION_LOCANA_CONFIG_decl => {
            let mut max_depth = None;
            let mut select = LocalAnalysisLifelineSelectionPolicy::OnlyOnImpactedByLastStep;
            for inner_pair in choice_pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::OPTION_LOCANA_select => {
                        let select_pair = inner_pair.into_inner().next().unwrap();
                        match select_pair.as_rule() {
                            Rule::OPTION_LOCANA_select_all => {
                                select = LocalAnalysisLifelineSelectionPolicy::SelectAll;
                            },
                            Rule::OPTION_LOCANA_select_dirty => {
                                select = LocalAnalysisLifelineSelectionPolicy::OnlyOnImpactedByLastStep;
                            },
                            _ => {
                                panic!("what rule then ? : {:?}", select_pair.as_rule() );
                            }
                        }
                    },
                    Rule::OPTION_LOCANA_depth => {
                        let depth_pair = inner_pair.into_inner().next().unwrap();
                        match depth_pair.as_rule() {
                            Rule::HIBOU_none => {
                                max_depth = None;
                            },
                            Rule::ARITH_INTEGER => {
                                let content = depth_pair.into_inner().next().unwrap();
                                let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                                let my_val : u32 = content_str.parse::<u32>().unwrap();
                                max_depth = Some(my_val);
                            },
                            _ => {
                                panic!("what rule then ? : {:?}", depth_pair.as_rule() );
                            }
                        }
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", inner_pair.as_rule() );
                    }
                }
            }
            let locana_param = LocalAnalysisParameterization::new(select,max_depth);
            return Ok(Some(locana_param));
        },
        // ***
        _ => {
            panic!("what rule then ? : {:?}", choice_pair.as_rule() );
        }
    }
}


fn parse_simulation_config(simu_config_decl_pair : Pair<Rule>) -> Result<SimulationConfiguration,HibouParsingError> {
    let mut sim_before = false;
    let mut reset_crit_after_exec = false;
    let mut multiply_by_multitrace_length = false;
    let mut loop_crit = SimulationLoopCriterion::MaxDepth;
    let mut act_crit = SimulationActionCriterion::None;
    // ***
    for config_opt_pair in simu_config_decl_pair.into_inner() {
        match config_opt_pair.as_rule() {
            Rule::OPTION_ANA_SIMULATE_CONFIG_simbefore => {
                let as_bool_pair = config_opt_pair.into_inner().next().unwrap();
                match as_bool_pair.as_rule() {
                    Rule::HIBOU_true => {
                        sim_before = true;
                    },
                    Rule::HIBOU_false => {
                        sim_before = false;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", as_bool_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_ANA_SIMULATE_CONFIG_multiply_by_mu_length => {
                let as_bool_pair = config_opt_pair.into_inner().next().unwrap();
                match as_bool_pair.as_rule() {
                    Rule::HIBOU_true => {
                        multiply_by_multitrace_length = true;
                    },
                    Rule::HIBOU_false => {
                        multiply_by_multitrace_length = false;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", as_bool_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_ANA_SIMULATE_CONFIG_reset => {
                let as_bool_pair = config_opt_pair.into_inner().next().unwrap();
                match as_bool_pair.as_rule() {
                    Rule::HIBOU_true => {
                        reset_crit_after_exec = true;
                    },
                    Rule::HIBOU_false => {
                        reset_crit_after_exec = false;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", as_bool_pair.as_rule() );
                    }
                }
            },
            Rule::OPTION_ANA_SIMULATE_CONFIG_act => {
                let inner : Pair<Rule> = config_opt_pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::OPTION_ANA_SIMULATE_CONFIG_crit_num => {
                        let content : Pair<Rule> = inner.into_inner().next().unwrap();
                        let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        let my_val : u32 = content_str.parse::<u32>().unwrap();
                        act_crit = SimulationActionCriterion::SpecificNum(my_val);
                    },
                    Rule::OPTION_ANA_SIMULATE_CONFIG_crit_maxnum => {
                        act_crit = SimulationActionCriterion::MaxNumOutsideLoops;
                    },
                    Rule::HIBOU_none => {
                        act_crit = SimulationActionCriterion::None;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", inner.as_rule() );
                    }
                }
            },
            Rule::OPTION_ANA_SIMULATE_CONFIG_loop => {
                let inner : Pair<Rule> = config_opt_pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::OPTION_ANA_SIMULATE_CONFIG_crit_num => {
                        let content : Pair<Rule> = inner.into_inner().next().unwrap();
                        let content_str : String = content.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        let my_val : u32 = content_str.parse::<u32>().unwrap();
                        loop_crit = SimulationLoopCriterion::SpecificNum(my_val);
                    },
                    Rule::OPTION_ANA_SIMULATE_CONFIG_crit_maxnum => {
                        loop_crit = SimulationLoopCriterion::MaxNum;
                    },
                    Rule::OPTION_ANA_SIMULATE_CONFIG_crit_maxdepth => {
                        loop_crit = SimulationLoopCriterion::MaxDepth;
                    },
                    Rule::HIBOU_none => {
                        loop_crit = SimulationLoopCriterion::None;
                    },
                    _ => {
                        panic!("what rule then ? : {:?}", inner.as_rule() );
                    }
                }
            },
            _ => {
                panic!("what rule then ? : {:?}", config_opt_pair.as_rule() );
            }
        }
    }
    // ***
    let config = SimulationConfiguration::new(sim_before,reset_crit_after_exec,multiply_by_multitrace_length,loop_crit,act_crit);
    return Ok(config);
}