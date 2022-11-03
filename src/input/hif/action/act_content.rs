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
use crate::core::language::syntax::action::CommunicationSynchronicity;
use crate::input::error::HibouParsingError;



#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::input::hif::parser::{HifParser,Rule};


pub enum ParsedReference {
    LifelineRef(usize),
    GateRef(usize)
}



pub fn parse_comm_act_origin(gen_ctx : &GeneralContext, origin_pair : Pair<Rule>) -> Result<ParsedReference,HibouParsingError> {
    let origin_name_pair = origin_pair.into_inner().next().unwrap();
    let origin_name : String = origin_name_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_lf_id( &origin_name ) {
        None => {
            match gen_ctx.get_gt_id( &origin_name ) {
                None => {
                    return Err( HibouParsingError::MissingLifelineOrGateDeclarationError(origin_name) );
                },
                Some( gt_id ) => {
                    return Ok( ParsedReference::GateRef(gt_id) );
                }
            }
        },
        Some( lf_id ) => {
            return Ok( ParsedReference::LifelineRef(lf_id) );
        }
    }
}

pub fn parse_comm_content(gen_ctx : &GeneralContext, comm_content_pair : Pair<Rule>) -> Result<(CommunicationSynchronicity,usize),HibouParsingError> {
    let mut contents = comm_content_pair.into_inner();
    let first_pair = contents.next().unwrap();
    let mut comm_type = CommunicationSynchronicity::Asynchronous;
    let mut got_ms_id : Option<usize> = None;
    match first_pair.as_rule() {
        Rule::COMM_ASYNCH => {
            comm_type = CommunicationSynchronicity::Asynchronous;
        },
        Rule::COMM_SYNCH => {
            comm_type = CommunicationSynchronicity::Synchronous;
        },
        Rule::HIBOU_LABEL => {
            let ms_name : String = first_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
            match gen_ctx.get_ms_id( &ms_name ) {
                None => {
                    return Err( HibouParsingError::MissingMessageDeclarationError( ms_name ) );
                },
                Some( ms_id ) => {
                    got_ms_id = Some(ms_id);
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", first_pair.as_rule() );
        }
    }
    // ***
    match got_ms_id {
        None => {
            let second_pair = contents.next().unwrap();
            match second_pair.as_rule() {
                Rule::HIBOU_LABEL => {
                    let ms_name : String = second_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                    match gen_ctx.get_ms_id( &ms_name ) {
                        None => {
                            return Err( HibouParsingError::MissingMessageDeclarationError( ms_name ) );
                        },
                        Some( ms_id ) => {
                            return Ok( (comm_type,ms_id) );
                        }
                    }
                },
                _ => {
                    panic!("what rule then ? : {:?}", second_pair.as_rule() );
                }
            }
        },
        Some(ms_id) => {
            return Ok( (comm_type,ms_id) );
        }
    }
}