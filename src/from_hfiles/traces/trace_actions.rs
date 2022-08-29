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

use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::Path;
use std::fs;
use pest::Parser;
use pest::iterators::{Pair,Pairs};

use crate::core::trace::*;
use crate::core::general_context::GeneralContext;

use crate::from_hfiles::error::HibouParsingError;

use crate::from_hfiles::parser::*;
use crate::process::ana_proc::multitrace::{AnalysableMultiTrace, AnalysableMultiTraceCanal};


pub fn trace_sequence_from_pair(trace_sequence_pair : Pair<Rule>,
                            gen_ctx : &mut GeneralContext,
                            unavailable_lifelines : &HashSet<usize>,
                            lifelines : &mut HashSet<usize>,
                            add_lfs : bool,
                                enrich_signature : bool) -> Result<Vec<HashSet<TraceAction>>,HibouParsingError> {
    let mut trace : Vec<HashSet<TraceAction>> = vec![];
    for trace_sequence_elt_pair in trace_sequence_pair.into_inner() {
        match trace_sequence_elt_pair.as_rule() {
            Rule::TRACE_ACTION => {
                match get_trace_action(trace_sequence_elt_pair,gen_ctx,unavailable_lifelines,lifelines,add_lfs,enrich_signature) {
                    Err(e) => {return Err(e);},
                    Ok( action ) => {
                        trace.push( hashset!{action} );
                    }
                }
            },
            Rule::TRACE_ACTION_SET => {
                let mut multi_action = hashset!{};
                for action_pair in trace_sequence_elt_pair.into_inner() {
                    match get_trace_action(action_pair,gen_ctx,unavailable_lifelines,lifelines,add_lfs,enrich_signature) {
                        Err(e) => {return Err(e);},
                        Ok( action ) => {
                            multi_action.insert(action);
                        }
                    }
                }
                trace.push( multi_action );
            },
            _ => {
                panic!("what rule then ? : {:?}", trace_sequence_elt_pair.as_rule() );
            }
        }
    }
    return Ok( trace );
}

fn get_trace_action(action_pair : Pair<Rule>,
                    gen_ctx : &mut GeneralContext,
                    unavailable_lifelines : &HashSet<usize>,
                    lifelines : &mut HashSet<usize>,
                    add_lfs : bool,
                    enrich_signature : bool)-> Result<TraceAction,HibouParsingError>  {
    match trace_action_from_text(action_pair,gen_ctx,enrich_signature) {
        Err(e) => {
            return Err(e);
        },
        Ok( action ) => {
            if unavailable_lifelines.contains(&action.lf_id) {
                return Err( HibouParsingError::NonDisjointTraceComponents );
            } else {
                if add_lfs {
                    lifelines.insert( action.lf_id);
                } else {
                    if !lifelines.contains( &action.lf_id ) {
                        return Err( HibouParsingError::IllDefinedTraceComponents(format!("lifeline of action {:?} not in predefined co-localisation {:?}",
                                                                                         action,
                                                                                         lifelines)) );
                    }
                }
            }
            return Ok( action );
        }
    }
}


fn trace_action_from_text(action_pair : Pair<Rule>,
                          gen_ctx : &mut GeneralContext,
                          enrich_signature : bool) -> Result<TraceAction,HibouParsingError> {
    let mut contents = action_pair.into_inner();
    // ***
    let lf_pair : Pair<Rule> = contents.next().unwrap();
    let lf_name : String  = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    // ***
    let got_lf_id : usize;
    match gen_ctx.get_lf_id(&lf_name) {
        None => {
            if enrich_signature {
                got_lf_id = gen_ctx.add_lf(lf_name);
            } else {
                return Err( HibouParsingError::MissingLifelineOrGateDeclarationError(lf_name) );
            }
        },
        Some( lf_id ) => {
            got_lf_id = lf_id;
        }
    }
    // ***
    let act_kind_pair : Pair<Rule> = contents.next().unwrap();
    let act_kind : TraceActionKind;
    match act_kind_pair.as_rule() {
        Rule::TRACE_EMISSION_SYMBOL => {
            act_kind = TraceActionKind::Emission;
        },
        Rule::TRACE_RECEPTION_SYMBOL => {
            act_kind = TraceActionKind::Reception;
        },
        _ => {
            panic!("what rule then ? : {:?}", act_kind_pair.as_rule() );
        }
    }
    // ***
    let ms_pair : Pair<Rule> = contents.next().unwrap();
    let ms_name : String  = ms_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    // ***
    let got_ms_id : usize;
    match gen_ctx.get_ms_id(&ms_name) {
        None => {
            if enrich_signature {
                got_ms_id = gen_ctx.add_msg(ms_name);
            } else {
                return Err( HibouParsingError::MissingMessageDeclarationError(ms_name) );
            }
        },
        Some( ms_id ) => {
            got_ms_id = ms_id;
        }
    }
    // ***
    return Ok( TraceAction{lf_id:got_lf_id,act_kind,ms_id:got_ms_id} );
}
