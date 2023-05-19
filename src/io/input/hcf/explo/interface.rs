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
use std::path::Path;
use pest::iterators::Pair;


#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};


use crate::core::general_context::GeneralContext;
use crate::io::file_extensions::HIBOU_CONFIGURATION_FILE_EXTENSION;
use crate::io::input::error::HibouParsingError;
use crate::io::input::hcf::explo::options::{HibouExploreOptions, parse_explore_options};


pub fn parse_hcf_file_for_explore(gen_ctx : &GeneralContext,
                                  file_path : &str) -> Result<HibouExploreOptions,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_CONFIGURATION_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_CONFIGURATION_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_hcf_str ) => {
            return parse_hcf_string_for_explore(gen_ctx,unparsed_hcf_str, file_name);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}


fn parse_hcf_string_for_explore(gen_ctx : &GeneralContext,
                                    hcf_string : String, file_name : &str)
                                    -> Result<HibouExploreOptions,HibouParsingError> {
    match HcfParser::parse(Rule::HCF_PEST_FILE, &hcf_string) {
        Ok( ref mut got_pair ) => {
            let conf_pair = got_pair.next().unwrap();
            match conf_pair.as_rule() {
                Rule::HIBOU_CONFIGURATION => {
                    return parse_conf_pair_for_explore(gen_ctx,conf_pair, file_name);
                },
                _ => {
                    panic!("what rule then ? : {:?}", conf_pair.as_rule() );
                }
            }
        },
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        }
    }
}


fn parse_conf_pair_for_explore(gen_ctx : &GeneralContext,
                               conf_pair : Pair<Rule>,
                               file_name : &str)
                               -> Result<HibouExploreOptions,HibouParsingError> {
    let mut got_section_explore_options   : bool = false;
    let mut explore_options = HibouExploreOptions::default();

    let mut contents = conf_pair.into_inner();

    while let Some(current_pair) = contents.next() {
        match current_pair.as_rule() {
            Rule::EXPLORE_OPTION_SECTION => {
                if got_section_explore_options {
                    return Err( HibouParsingError::HsfSetupError("several '@explore_option' sections declared".to_string()));
                }
                got_section_explore_options = true;
                // ***
                match parse_explore_options(gen_ctx,current_pair,file_name) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( exp_opts ) => {
                        explore_options = exp_opts;
                    }
                }
            },
            Rule::ANALYZE_OPTION_SECTION => {
                // nothing
            },
            Rule::CANONIZE_OPTION_SECTION => {
                // nothing
            },
            _ => {
                panic!("what rule then ? : {:?}", current_pair.as_rule() );
            }
        }
    }

    return Ok(explore_options);
}
