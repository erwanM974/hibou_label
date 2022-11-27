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


use crate::io::input::hcf::proc_options::opt_analyze::{HibouAnalyzeOptions, parse_analyze_options};

#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hcf::parser::{HcfParser,Rule};


pub fn parse_hcf_string_for_ana(gen_ctx : &GeneralContext, hcf_string : String, file_name : &str) -> Result<HibouAnalyzeOptions,HibouParsingError> {
    match HcfParser::parse(Rule::HCF_PEST_FILE, &hcf_string) {
        Ok( ref mut got_pair ) => {
            let conf_pair = got_pair.next().unwrap();
            match conf_pair.as_rule() {
                Rule::HIBOU_CONFIGURATION => {
                    return parse_conf_pair_for_ana(gen_ctx,conf_pair, file_name);
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


fn parse_conf_pair_for_ana(gen_ctx : &GeneralContext,
                               conf_pair : Pair<Rule>,
                               file_name : &str)
                               -> Result<HibouAnalyzeOptions,HibouParsingError> {
    let mut got_section_ana_options   : bool = false;
    let mut ana_options = HibouAnalyzeOptions::default();

    let mut contents = conf_pair.into_inner();

    while let Some(current_pair) = contents.next() {
        match current_pair.as_rule() {
            Rule::ANALYZE_OPTION_SECTION => {
                if got_section_ana_options {
                    return Err( HibouParsingError::HsfSetupError("several '@analyze_option' sections declared".to_string()));
                }
                got_section_ana_options = true;
                // ***
                match parse_analyze_options(current_pair,file_name) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( ana_opts ) => {
                        ana_options = ana_opts;
                    }
                }
            },
            Rule::EXPLORE_OPTION_SECTION => {
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

    return Ok(ana_options);
}
