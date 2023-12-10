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
use crate::io::input::hcf::loggers::nfait::parse_nfait_logger;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};
use crate::process::canon::conf::CanonizationConfig;
use crate::process::canon::filter::elim::CanonizationFilterEliminationKind;
use crate::process::canon::filter::filter::CanonizationFilterCriterion;
use crate::process::canon::priorities::CanonizationPriorities;


pub struct HibouCanonizeOptions {
    pub loggers : Vec<Box<dyn AbstractProcessLogger<CanonizationConfig>>>,
    pub strategy : QueueSearchStrategy,
    pub filters : Vec<Box<dyn AbstractFilter<CanonizationFilterCriterion,CanonizationFilterEliminationKind>>>,
    pub priorities : GenericProcessPriorities<CanonizationPriorities>,
    pub search_all : bool,
}



impl HibouCanonizeOptions {
    fn new(loggers : Vec<Box<dyn AbstractProcessLogger<CanonizationConfig>>>,
               strategy : QueueSearchStrategy,
               filters : Vec<Box<dyn AbstractFilter<CanonizationFilterCriterion,CanonizationFilterEliminationKind>>>,
               priorities : GenericProcessPriorities<CanonizationPriorities>,
           search_all : bool) -> HibouCanonizeOptions {
        return HibouCanonizeOptions{loggers,strategy,filters,priorities,search_all};
    }

    pub fn default() -> HibouCanonizeOptions {
        HibouCanonizeOptions::new(
            Vec::new(),
            QueueSearchStrategy::BFS,
            vec![],
            GenericProcessPriorities::new(CanonizationPriorities::default(),false),
            false
        )
    }

}




pub fn parse_canonize_options(gen_ctx: &GeneralContext,
                             option_pair : Pair<Rule>,
                             file_name : &str) -> Result<HibouCanonizeOptions,HibouParsingError> {
    let mut loggers : Vec<Box<dyn AbstractProcessLogger<CanonizationConfig>>> = Vec::new();
    let mut search_all = false;
    // ***
    for option_decl_pair in option_pair.into_inner() {
        match option_decl_pair.as_rule() {
            Rule::OPTION_CANON_searchall_yes => {
                search_all = true;
            },
            Rule::OPTION_CANON_searchall_no => {
                search_all = false;
            },
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
            _ => {
                panic!("what rule then ? : {:?}", option_decl_pair.as_rule() );
            }
        }
    }
    // ***
    let strategy : QueueSearchStrategy = QueueSearchStrategy::BFS;
    let filters : Vec<Box<dyn AbstractFilter<CanonizationFilterCriterion,CanonizationFilterEliminationKind>>> = Vec::new();
    let priorities : GenericProcessPriorities<CanonizationPriorities> = GenericProcessPriorities::new(CanonizationPriorities::default(),false);
    let hoptions = HibouCanonizeOptions::new(loggers,strategy,filters,priorities,search_all);
    return Ok(hoptions);
}





