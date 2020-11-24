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
use pest::iterators::{Pair,Pairs};



use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;

use crate::from_text::parser::*;
use crate::from_text::error::HibouParsingError;



pub fn parse_reception(gen_ctx : &mut GeneralContext, contents : &mut Pairs<Rule>) -> Result<ObservableAction,HibouParsingError> {
    let ms_pair = contents.next().unwrap();
    let ms_name : String = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_ms_id( &ms_name ) {
        None => {
            return Err( HibouParsingError::MissingMessageDeclarationError( ms_name ) );
        },
        Some( ms_id ) => {
            let target_lf_pair =  contents.next().unwrap();
            let target_lf_name : String = target_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            // ***
            match gen_ctx.get_lf_id( &target_lf_name ) {
                None => {
                    return Err( HibouParsingError::MissingLifelineDeclarationError( target_lf_name ) );
                },
                Some( lf_id ) => {
                    return Ok( ObservableAction{lf_id,act_kind:ObservableActionKind::Reception,ms_id} );
                }
            }
            // ***
        }
    }
}

pub fn parse_emission(gen_ctx : &mut GeneralContext, contents : &mut Pairs<Rule>) -> Result<ObservableAction,HibouParsingError> {
    let orig_lf_pair = contents.next().unwrap();
    let orig_lf_name : String = orig_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    // ***
    match gen_ctx.get_lf_id( &orig_lf_name ) {
        None => {
            return Err( HibouParsingError::MissingLifelineDeclarationError( orig_lf_name ) );
        },
        Some( lf_id ) => {
            let ms_pair = contents.next().unwrap();
            let ms_name : String = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_ms_id( &ms_name ) {
                None => {
                    return Err( HibouParsingError::MissingMessageDeclarationError( ms_name ) );
                },
                Some( ms_id ) => {
                    let next_pair =  contents.next().unwrap();
                    match next_pair.as_rule() {
                        Rule::SD_LIFELINE => {
                            let target_lf_name : String = next_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                            // ***
                            match gen_ctx.get_lf_id( &target_lf_name ) {
                                None => {
                                    return Err( HibouParsingError::MissingLifelineDeclarationError( target_lf_name ) );
                                },
                                Some( tar_lf_id ) => {
                                    return Ok( ObservableAction{lf_id,act_kind:ObservableActionKind::Emission(vec![tar_lf_id]),ms_id} );
                                }
                            }
                            // ***
                        },
                        Rule::TARGET_LF_LIST => {
                            match parse_lifelines_list(gen_ctx, next_pair) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( target_lfs ) => {
                                    if target_lfs.contains(&lf_id) {
                                        return Err( HibouParsingError::EmissionDefinitionError( "emitting to sender not supported (in graphical representations)".to_string() ) );
                                    } else {
                                        return Ok( ObservableAction{lf_id,act_kind:ObservableActionKind::Emission(target_lfs),ms_id} );
                                    }
                                }
                            }
                        },
                        Rule::ENVIRONMENT_TARGET => {
                            return Ok( ObservableAction{lf_id,act_kind:ObservableActionKind::Emission(Vec::new()),ms_id} );
                        },
                        _ => {
                            panic!();
                        }
                    }
                }
            }
        }
    }
}


pub fn parse_lifelines_list(gen_ctx : &mut GeneralContext, next_pair : Pair<Rule>) -> Result<Vec<usize>,HibouParsingError> {
    let mut target_lfs : Vec<usize> = Vec::new();
    let mut inner_contents = next_pair.into_inner();
    for tar_lf_pair in inner_contents {
        let target_lf_name : String = tar_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
        // ***
        match gen_ctx.get_lf_id( &target_lf_name ) {
            None => {
                return Err( HibouParsingError::MissingLifelineDeclarationError( target_lf_name ) );
            },
            Some( tar_lf_id ) => {
                if target_lfs.contains(&tar_lf_id) {
                    return Err( HibouParsingError::EmissionDefinitionError( "duplicate lifeline in lifeline list".to_string() ) );
                } else {
                    target_lfs.push(tar_lf_id);
                }
            }
        }
    }
    return Ok( target_lfs );
}