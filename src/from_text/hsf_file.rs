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


use crate::from_text::error::HibouParsingError;
use crate::process::log::ProcessLogger;

use crate::from_text::parser::*;
use crate::from_text::interaction::parse_interaction;
use crate::rendering::process::graphic_logger::GraphicProcessLogger;
use crate::from_text::hibou_options::*;


pub static HIBOU_MODEL_FILE_EXTENSION : &'static str = "hsf";

pub enum ProcessKind {
    Explore,
    Analyze,
    None
}

pub fn parse_hsf_file(file_path : &str, process_kind : &ProcessKind) -> Result<(GeneralContext,Interaction,HibouOptions),HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_MODEL_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_MODEL_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_hsf_str ) => {
            return parse_hsf_string(unparsed_hsf_str, file_name, process_kind);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

pub fn parse_hsf_string(sd_string : String,
                        name : &str,
                        process_kind : &ProcessKind) -> Result<(GeneralContext,Interaction,HibouOptions),HibouParsingError> {
    match SDParser::parse(Rule::HSF_PEST_FILE, &sd_string) {
        Ok( ref mut sd_cfg_pair ) => {
            let mut content = sd_cfg_pair.next().unwrap().into_inner();
            let first_pair = content.next().unwrap();
            match first_pair.as_rule() {
                Rule::HIBOU_MODEL_SETUP => {
                    let second_pair = content.next().unwrap();
                    return parse_sd(second_pair,Some(first_pair),name,process_kind);
                },
                Rule::SD_INTERACTION => {
                    return parse_sd(first_pair, None,name,process_kind);
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

fn parse_setup(setup_pair : Pair<Rule>,
               gen_ctx : &mut GeneralContext,
               file_name : &str,
               process_kind : &ProcessKind) -> Result<HibouOptions,HibouParsingError> {
    // ***
    let mut got_section_explore_options   : bool = false;
    let mut got_section_analyze_options   : bool = false;
    let mut got_section_messages  : bool = false;
    let mut got_section_lifelines : bool = false;
    // ***
    let mut contents = setup_pair.into_inner();
    let mut hibou_options_opt : Option<HibouOptions> = None;
    while let Some(current_pair) = contents.next() {
        match current_pair.as_rule() {
            Rule::EXPLORE_OPTION_SECTION => {
                if got_section_explore_options {
                    return Err( HibouParsingError::HsfSetupError("several '@explore_option' sections declared".to_string()));
                }
                got_section_explore_options = true;
                match process_kind {
                    &ProcessKind::Explore => {
                        match parse_hibou_options(current_pair,file_name, process_kind) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( hoptions ) => {
                                hibou_options_opt = Some(hoptions);
                            }
                        }
                    },
                    _ => {}
                }
            },
            Rule::ANALYZE_OPTION_SECTION => {
                if got_section_analyze_options {
                    return Err( HibouParsingError::HsfSetupError("several '@analyze_option' sections declared".to_string()));
                }
                got_section_analyze_options = true;
                match process_kind {
                    &ProcessKind::Analyze => {
                        match parse_hibou_options(current_pair,file_name, process_kind) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( hoptions ) => {
                                hibou_options_opt = Some(hoptions);
                            }
                        }
                    },
                    _ => {}
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
            _ => {
                panic!("what rule then ? : {:?}", current_pair.as_rule() );
            }
        }
    }
    match hibou_options_opt {
        None => {
            match process_kind {
                ProcessKind::Analyze => {
                    return Ok( HibouOptions::default_analyze() );
                },
                _ => {
                    return Ok( HibouOptions::default_explore() );
                }
            }
        },
        Some(hibou_options) => {
            return Ok( hibou_options );
        }
    }
}

fn parse_sd(interaction_pair : Pair<Rule>,
            setup_pair_opt : Option< Pair<Rule> >,
            name : &str,
            process_kind : &ProcessKind) -> Result<(GeneralContext,Interaction,HibouOptions),HibouParsingError> {
    let mut gen_ctx = GeneralContext::new();
    let hibou_options : HibouOptions;
    match setup_pair_opt {
        None => {
            match process_kind {
                ProcessKind::Analyze => {
                    hibou_options = HibouOptions::default_analyze();
                },
                _ => {
                    hibou_options = HibouOptions::default_explore();
                }
            }
        },
        Some( setup_pair ) => {
            match parse_setup(setup_pair, &mut gen_ctx, name, process_kind) {
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