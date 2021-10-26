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
                    return Err( HibouParsingError::MissingLifelineOrGateDeclarationError( target_lf_name ) );
                },
                Some( lf_id ) => {
                    return Ok( ObservableAction{lf_id,act_kind:ObservableActionKind::Reception(None),ms_id} );
                }
            }
            // ***
        }
    }
}

fn parse_emission_inner(gen_ctx : &mut GeneralContext, contents : &mut Pairs<Rule>) -> Result<(usize,Vec<EmissionTargetRef>),HibouParsingError> {
    let ms_pair = contents.next().unwrap();
    let ms_name : String = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_ms_id( &ms_name ) {
        None => {
            return Err( HibouParsingError::MissingMessageDeclarationError( ms_name ) );
        },
        Some( ms_id ) => {
            let next_pair =  contents.next().unwrap();
            let mut target_refs : Vec<EmissionTargetRef> = Vec::new();
            match next_pair.as_rule() {
                Rule::HIBOU_LABEL => {
                    let target_name : String = next_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    // ***
                    match gen_ctx.get_lf_id( &target_name ) {
                        None => {
                            match gen_ctx.get_gt_id(&target_name) {
                                None => {
                                    return Err( HibouParsingError::MissingLifelineOrGateDeclarationError( target_name ) );
                                },
                                Some( tar_gt_id ) => {
                                    target_refs.push( EmissionTargetRef::Gate( tar_gt_id ));
                                }
                            }
                        },
                        Some( tar_lf_id ) => {
                            target_refs.push( EmissionTargetRef::Lifeline( tar_lf_id ));
                        }
                    }
                    // ***
                },
                Rule::HIBOU_LABEL_LIST => {
                    let mut inner_contents = next_pair.into_inner();
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
                },
                Rule::ENVIRONMENT_TARGET => {
                    // nothing
                },
                _ => {
                    panic!();
                }
            }
            return Ok( (ms_id,target_refs) )
        }
    }
}

pub fn parse_emission(gen_ctx : &mut GeneralContext, contents : &mut Pairs<Rule>) -> Result<Interaction,HibouParsingError> {
    let orig_pair = contents.next().unwrap();
    let orig_name : String = orig_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    // ***
    match gen_ctx.get_lf_id( &orig_name ) {
        None => {
            match gen_ctx.get_gt_id( &orig_name ) {
                None => {
                    return Err( HibouParsingError::MissingLifelineOrGateDeclarationError( orig_name ) );
                },
                Some(gt_id) => {
                    match parse_emission_inner(gen_ctx, contents) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( (ms_id,target_refs) ) => {
                            let mut reception_actions : Vec<ObservableAction> = Vec::new();
                            if target_refs.len() == 0 {
                                return Err( HibouParsingError::WrongGateUsage("reception from gate must have at least one target".to_string()) );
                            } else {
                                for target_ref in target_refs {
                                    match target_ref {
                                        EmissionTargetRef::Lifeline( lf_id ) => {
                                            reception_actions.push( ObservableAction{lf_id,act_kind:ObservableActionKind::Reception(Some(gt_id)),ms_id} );
                                        },
                                        _ => {
                                            return Err( HibouParsingError::WrongGateUsage("reception from gate cannot have another gate as target".to_string()) );
                                        }
                                    }
                                }
                            }
                            return Ok( fold_reception_actions_with_seq(&mut reception_actions) );
                        }
                    }
                }
            }
        },
        Some( lf_id ) => {
            match parse_emission_inner(gen_ctx, contents) {
                Err(e) => {
                    return Err(e);
                },
                Ok( (ms_id,target_refs) ) => {
                    let emission_action = ObservableAction{lf_id:lf_id,act_kind:ObservableActionKind::Emission(target_refs),ms_id:ms_id};
                    return Ok( Interaction::Action(emission_action) );
                }
            }
        }
    }
}


