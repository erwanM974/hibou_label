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


use graph_process_manager_loggers::nfait::logger::GenericNFAITLogger;
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use pest::iterators::Pair;

use crate::core::general_context::GeneralContext;
use crate::io::input::error::HibouParsingError;



#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};

use crate::process::explo::conf::ExplorationConfig;
use crate::process::explo::loggers::nfait::printer::ActionNFAITPrinter;


pub fn parse_nfait_logger(logger_id : u32,
                          gen_ctx : &GeneralContext,
                          file_name : &str,
                          logger_kind_pair : Pair<Rule>)
        -> Result<GenericNFAITLogger<ExplorationConfig,usize,ActionNFAITPrinter>,HibouParsingError> {
    let printer = ActionNFAITPrinter::new(vec![],gen_ctx.clone());
    return Ok(GenericNFAITLogger::new(printer,
                                      format!("nfait_l{:?}", logger_id),
                                      Some((true,GraphVizOutputFormat::svg)),
                                         ".".to_string(),
                                         ));
}

