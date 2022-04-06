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
use crate::core::syntax::interaction::*;
use crate::core::general_context::GeneralContext;

use crate::from_hfiles::parser::*;
use crate::from_hfiles::error::HibouParsingError;


pub fn parse_comm_act_targets_as_lifelines(gen_ctx : &GeneralContext, target_pair : Pair<Rule>) -> Result<Vec<usize>,HibouParsingError> {
    let inner_pair = target_pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::HIBOU_LABEL => {
            let lf_name : String = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_lf_id( &lf_name ) {
                None => {
                    return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
                },
                Some( lf_id ) => {
                    return Ok( vec![lf_id] );
                }
            }
        },
        Rule::HIBOU_LABEL_LIST => {
            let mut target_lf_ids : Vec<usize> = vec![];
            for label_pair in inner_pair.into_inner() {
                let lf_name : String = label_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                match gen_ctx.get_lf_id( &lf_name ) {
                    None => {
                        return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
                    },
                    Some( lf_id ) => {
                        if target_lf_ids.contains(&lf_id) {
                            return Err( HibouParsingError::EmissionDefinitionError( format!("duplicate target {:}",lf_name) ) );
                        } else {
                            target_lf_ids.push( lf_id );
                        }
                    }
                }
            }
            return Ok( target_lf_ids );
        },
        Rule::ENVIRONMENT_TARGET => {
            return Ok( vec![] );
        },
        _ => {
            panic!("what rule then ? : {:?}", inner_pair.as_rule() );
        }
    }
}

pub fn parse_comm_act_targets_as_generic_targets(gen_ctx : &GeneralContext, target_pair : Pair<Rule>) -> Result<Vec<EmissionTargetRef>,HibouParsingError> {
    let inner_pair = target_pair.into_inner().next().unwrap();
    match inner_pair.as_rule() {
        Rule::HIBOU_LABEL => {
            let target_name : String = inner_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            // ***
            match gen_ctx.get_lf_id( &target_name ) {
                None => {
                    match gen_ctx.get_gt_id(&target_name) {
                        None => {
                            return Err( HibouParsingError::MissingLifelineOrGateDeclarationError( target_name ) );
                        },
                        Some( tar_gt_id ) => {
                            return Ok( vec![ EmissionTargetRef::Gate( tar_gt_id ) ] );
                        }
                    }
                },
                Some( tar_lf_id ) => {
                    return Ok( vec![ EmissionTargetRef::Lifeline( tar_lf_id ) ] );
                }
            }
            // ***
        },
        Rule::HIBOU_LABEL_LIST => {
            let mut inner_contents = inner_pair.into_inner();
            let mut target_refs : Vec<EmissionTargetRef> = Vec::new();
            for target_pair in inner_contents {
                let target_name : String = target_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                // ***
                match gen_ctx.get_lf_id( &target_name ) {
                    None => {
                        match gen_ctx.get_gt_id(&target_name) {
                            None => {
                                return Err( HibouParsingError::MissingLifelineOrGateDeclarationError( target_name ) );
                            },
                            Some( tar_gt_id ) => {
                                let new_ref = EmissionTargetRef::Gate( tar_gt_id );
                                if target_refs.contains(&new_ref) {
                                    return Err( HibouParsingError::EmissionDefinitionError( "duplicate target in emission".to_string() ) );
                                } else {
                                    target_refs.push(new_ref);
                                }
                            }
                        }
                    },
                    Some( tar_lf_id ) => {
                        let new_ref = EmissionTargetRef::Lifeline( tar_lf_id );
                        if target_refs.contains(&new_ref) {
                            return Err( HibouParsingError::EmissionDefinitionError( "duplicate target in emission".to_string() ) );
                        } else {
                            target_refs.push(new_ref);
                        }
                    }
                }
            }
            return Ok( target_refs );
        },
        Rule::ENVIRONMENT_TARGET => {
            return Ok( vec![] );
        },
        _ => {
            panic!("what rule then ? : {:?}", inner_pair.as_rule() );
        }
    }
}





