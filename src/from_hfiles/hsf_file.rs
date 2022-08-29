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
use std::collections::HashMap;
use std::collections::btree_map::BTreeMap;
use std::path::Path;

use pest::iterators::Pair;

use crate::pest::Parser;

use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;


use crate::from_hfiles::error::HibouParsingError;

use crate::from_hfiles::parser::*;
use crate::from_hfiles::interaction::parse_interaction;
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;
use crate::from_hfiles::proc_options::opt_explore::{HibouExploreOptions, parse_explore_options};
use crate::from_hfiles::proc_options::opt_analyze::{HibouAnalyzeOptions, parse_analyze_options};
use crate::ui::extensions::HIBOU_MODEL_FILE_EXTENSION;


pub struct HibouOptions {
    pub explore_options : HibouExploreOptions,
    pub analyze_options : HibouAnalyzeOptions
}

impl HibouOptions{

    pub fn new(explore_options : HibouExploreOptions,
               analyze_options : HibouAnalyzeOptions) -> HibouOptions {
        return HibouOptions{explore_options,analyze_options};
    }

    pub fn default() -> HibouOptions {
        return HibouOptions::new(HibouExploreOptions::default(),HibouAnalyzeOptions::default());
    }

}

pub fn parse_hsf_file(file_path : &str) -> Result<(GeneralContext,Interaction,HibouOptions),HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_MODEL_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_MODEL_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_hsf_str ) => {
            return parse_hsf_string(unparsed_hsf_str, file_name);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

fn parse_hsf_string(sd_string : String,
                        name : &str) -> Result<(GeneralContext,Interaction,HibouOptions),HibouParsingError> {
    match SDParser::parse(Rule::HSF_PEST_FILE, &sd_string) {
        Ok( ref mut sd_cfg_pair ) => {
            let mut content = sd_cfg_pair.next().unwrap().into_inner();
            let first_pair = content.next().unwrap();
            match first_pair.as_rule() {
                Rule::HIBOU_MODEL_SETUP => {
                    let second_pair = content.next().unwrap();
                    return parse_sd(second_pair,Some(first_pair),name);
                },
                Rule::SD_INTERACTION => {
                    return parse_sd(first_pair, None,name);
                },
                _ => {
                    unreachable!();
                }
            }
        },
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        }
    }
}


fn parse_message_decl(ms_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) {
    for ms_pair in ms_decl_pair.into_inner() {
        let ms_name : String = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_msg(ms_name);
    }
}

fn parse_lifeline_decl(lf_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) {
    for lf_pair in lf_decl_pair.into_inner() {
        let lf_name : String = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_lf(lf_name);
    }
}

fn parse_gate_decl(gt_decl_pair : Pair<Rule>, gen_ctx : &mut GeneralContext ) {
    for gt_pair in gt_decl_pair.into_inner() {
        let gt_name : String = gt_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        gen_ctx.add_gt(gt_name);
    }
}

fn parse_setup(setup_pair : Pair<Rule>,
               gen_ctx : &mut GeneralContext,
               file_name : &str) -> Result<HibouOptions,HibouParsingError> {
    // ***
    let mut got_section_explore_options   : bool = false;
    let mut got_section_analyze_options   : bool = false;
    let mut got_section_messages  : bool = false;
    let mut got_section_lifelines : bool = false;
    let mut got_section_gates : bool = false;
    // ***
    let mut contents = setup_pair.into_inner();
    // ***
    let mut explore_options = HibouExploreOptions::default();
    let mut analyze_options = HibouAnalyzeOptions::default();
    // ***
    while let Some(current_pair) = contents.next() {
        match current_pair.as_rule() {
            Rule::EXPLORE_OPTION_SECTION => {
                if got_section_explore_options {
                    return Err( HibouParsingError::HsfSetupError("several '@explore_option' sections declared".to_string()));
                }
                got_section_explore_options = true;
                // ***
                // todo( separate the general context declaration from other options )
                // todo( at first lfs msgs decls and only then process options )
                // todo( it may cause problems otherwise )
                match parse_explore_options(&gen_ctx,current_pair,file_name) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( exp_opts ) => {
                        explore_options = exp_opts;
                    }
                }
            },
            Rule::ANALYZE_OPTION_SECTION => {
                if got_section_analyze_options {
                    return Err( HibouParsingError::HsfSetupError("several '@analyze_option' sections declared".to_string()));
                }
                got_section_analyze_options = true;
                // ***
                match parse_analyze_options(current_pair,file_name) {
                    Err(e) => {
                        return Err(e);
                    },
                    Ok( ana_opts ) => {
                        analyze_options = ana_opts;
                    }
                }
            },
            Rule::HIBOU_MODEL_MS_DECL => {
                if got_section_messages {
                    return Err( HibouParsingError::HsfSetupError("several '@message' sections declared".to_string()));
                }
                got_section_messages = true;
                parse_message_decl(current_pair,gen_ctx);
            },
            Rule::HIBOU_MODEL_LF_DECL => {
                if got_section_lifelines {
                    return Err( HibouParsingError::HsfSetupError("several '@lifeline' sections declared".to_string()));
                }
                got_section_lifelines = true;
                parse_lifeline_decl(current_pair,gen_ctx);
            },
            Rule::HIBOU_MODEL_GT_DECL => {
                if got_section_gates {
                    return Err( HibouParsingError::HsfSetupError("several '@gate' sections declared".to_string()));
                }
                got_section_gates = true;
                parse_gate_decl(current_pair,gen_ctx);
            },
            _ => {
                panic!("what rule then ? : {:?}", current_pair.as_rule() );
            }
        }
    }
    // ***
    return Ok( HibouOptions{explore_options,analyze_options} );
}

fn parse_sd(interaction_pair : Pair<Rule>,
            setup_pair_opt : Option< Pair<Rule> >,
            name : &str) -> Result<(GeneralContext,Interaction,HibouOptions),HibouParsingError> {
    let mut gen_ctx = GeneralContext::new();
    let hibou_options : HibouOptions;
    match setup_pair_opt {
        None => {
            hibou_options = HibouOptions::default();
        },
        Some( setup_pair ) => {
            match parse_setup(setup_pair, &mut gen_ctx, name) {
                Err(e) => {
                    return Err(e);
                },
                Ok( hopts ) => {
                    hibou_options = hopts;
                }
            }
        }
    }
    match parse_interaction(&mut gen_ctx, interaction_pair) {
        Err(e) => {
            return Err(e);
        },
        Ok( interaction ) => {
            return Ok( (gen_ctx,interaction,hibou_options) );
        }
    }
}