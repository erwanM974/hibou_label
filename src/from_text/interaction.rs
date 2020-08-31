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


use crate::from_text::parser::*;


use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;
use crate::core::syntax::interaction::{Interaction,ScheduleOperatorKind};

use crate::from_text::error::HibouParsingError;
use crate::from_text::action::{parse_emission,parse_reception};


pub fn parse_interaction(gen_ctx : &mut GeneralContext, sd_interaction_pair : Pair<Rule>) -> Result<Interaction,HibouParsingError> {
    let sd_content_pair = sd_interaction_pair.into_inner().next().unwrap();
    match sd_content_pair.as_rule() {
        Rule::SD_EMPTY_INTERACTION => {
            return Ok( Interaction::Empty );
        },
        Rule::SD_ACTION_RECEPTION => {
            match parse_reception(gen_ctx,&mut sd_content_pair.into_inner()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( observable_action ) => {
                    return Ok( Interaction::Action(observable_action) );
                }
            }
        },
        Rule::SD_ACTION_EMISSION => {
            match parse_emission(gen_ctx,&mut sd_content_pair.into_inner()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( observable_action ) => {
                    return Ok( Interaction::Action(observable_action) );
                }
            }
        },
        Rule::SD_STRICT_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Strict,&mut sub_ints) );
                }
            }
        },
        Rule::SD_SEQ_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Seq,&mut sub_ints) );
                }
            }
        },
        Rule::SD_ALT_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Alt,&mut sub_ints) );
                }
            }
        },
        Rule::SD_PAR_INT => {
            match get_nary_sub_interactions(gen_ctx, sd_content_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( mut sub_ints ) => {
                    return Ok( fold_interactions_in_binary_operator(BinaryOperatorKind::Par,&mut sub_ints) );
                }
            }
        },
        Rule::SD_LOOP_INT => {
            let mut loop_content = sd_content_pair.into_inner();
            let loop_kind_pair = loop_content.next().unwrap().into_inner().next().unwrap();
            match parse_interaction(gen_ctx,loop_content.next().unwrap()) {
                Err(e) => {
                    return Err(e);
                },
                Ok( sub_int ) => {
                    match loop_kind_pair.as_rule() {
                        Rule::SD_STRICT => {
                            return Ok( Interaction::Loop(ScheduleOperatorKind::Strict,Box::new(sub_int)) );
                        },
                        Rule::SD_SEQ => {
                            return Ok( Interaction::Loop(ScheduleOperatorKind::Seq,Box::new(sub_int)) );
                        },
                        Rule::SD_PAR => {
                            return Ok( Interaction::Loop(ScheduleOperatorKind::Par,Box::new(sub_int)) );
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", sd_content_pair.as_rule());
        }
    }
}

fn get_nary_sub_interactions(gen_ctx : &mut GeneralContext, sd_content_pair : Pair<Rule>) -> Result<Vec<Interaction>,HibouParsingError> {
    let mut strict_content = sd_content_pair.into_inner();
    strict_content.next(); // get rid of the operator name
    let mut sub_ints : Vec<Interaction> = Vec::new();
    for sub_interaction in strict_content {
        match parse_interaction(gen_ctx,sub_interaction) {
            Err(e) => {
                return Err(e);
            },
            Ok( parsed_sub_int ) => {
                sub_ints.push( parsed_sub_int );
            }
        }
    }
    return Ok( sub_ints );
}

enum BinaryOperatorKind {
    Strict,
    Seq,
    Par,
    Alt
}

fn fold_interactions_in_binary_operator(op_kind : BinaryOperatorKind, sub_ints : &mut Vec<Interaction>) -> Interaction {
    assert!(sub_ints.len() > 0);
    if sub_ints.len() == 1 {
        return sub_ints.remove(0);
    } else {
        let first_int = sub_ints.remove(0);
        match op_kind {
            BinaryOperatorKind::Strict => {
                return Interaction::Strict( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Seq => {
                return Interaction::Seq( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Alt => {
                return Interaction::Alt( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            },
            BinaryOperatorKind::Par => {
                return Interaction::Par( Box::new(first_int), Box::new(fold_interactions_in_binary_operator(op_kind,sub_ints)));
            }
        }
    }
}





