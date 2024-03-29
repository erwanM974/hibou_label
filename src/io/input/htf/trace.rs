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

use std::collections::{BTreeSet, HashSet};

use pest::iterators::Pair;


use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::multitrace::Trace;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::io::input::error::HibouParsingError;


#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use crate::io::input::htf::parser::{HtfParser,Rule};



pub fn trace_sequence_from_pair(gen_ctx : &GeneralContext,
                                trace_sequence_pair : Pair<Rule>,
                                unavailable_lifelines : &BTreeSet<usize>,
                                lifelines : &mut BTreeSet<usize>,
                                add_lfs : bool) -> Result<Trace,HibouParsingError> {
    let mut trace : Trace = vec![];
    for trace_elt_pair in trace_sequence_pair.into_inner() {
        match trace_element_from_pair(gen_ctx,trace_elt_pair,unavailable_lifelines,lifelines,add_lfs) {
            Err(e) => {
                return Err(e);
            }
            Ok(trace_elt) => {
                trace.push(trace_elt);
            }
        }
    }
    return Ok( trace );
}


pub fn trace_element_from_pair(gen_ctx : &GeneralContext,
                               trace_elt_pair : Pair<Rule>,
                               unavailable_lifelines : &BTreeSet<usize>,
                               lifelines : &mut BTreeSet<usize>,
                               add_lfs : bool) -> Result<BTreeSet<TraceAction>,HibouParsingError> {
    match trace_elt_pair.as_rule() {
        Rule::TRACE_ACTION => {
            match get_trace_action(gen_ctx,trace_elt_pair,unavailable_lifelines,lifelines,add_lfs) {
                Err(e) => {return Err(e);},
                Ok( action ) => {
                    return Ok(btreeset!{action});
                }
            }
        },
        Rule::TRACE_ACTION_SET => {
            match get_trace_multi_action(gen_ctx,trace_elt_pair,unavailable_lifelines,lifelines,add_lfs) {
                Err(e) => {return Err(e);},
                Ok( multi_action ) => {
                    return Ok(multi_action);
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", trace_elt_pair.as_rule() );
        }
    }
}


fn get_trace_multi_action(gen_ctx : &GeneralContext,
                          multi_act_pair : Pair<Rule>,
                          unavailable_lifelines : &BTreeSet<usize>,
                          lifelines : &mut BTreeSet<usize>,
                          add_lfs : bool) -> Result<BTreeSet<TraceAction>,HibouParsingError> {
    let mut multi_action = btreeset!{};
    for action_pair in multi_act_pair.into_inner() {
        match get_trace_action(gen_ctx,action_pair,unavailable_lifelines,lifelines,add_lfs) {
            Err(e) => {return Err(e);},
            Ok( action ) => {
                multi_action.insert(action);
            }
        }
    }
    return Ok(multi_action);
}

fn get_trace_action(gen_ctx : &GeneralContext,
                    action_pair : Pair<Rule>,
                    unavailable_lifelines : &BTreeSet<usize>,
                    lifelines : &mut BTreeSet<usize>,
                    add_lfs : bool)-> Result<TraceAction,HibouParsingError>  {
    match trace_action_from_text(gen_ctx,action_pair) {
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


fn trace_action_from_text(gen_ctx : &GeneralContext,
                          action_pair : Pair<Rule>) -> Result<TraceAction,HibouParsingError> {
    let mut contents = action_pair.into_inner();
    // ***
    let lf_pair : Pair<Rule> = contents.next().unwrap();
    let lf_name : String  = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    // ***
    let got_lf_id : usize;
    match gen_ctx.get_lf_id(&lf_name) {
        None => {
            return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
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
            return Err( HibouParsingError::MissingMessageDeclarationError(ms_name) );
        },
        Some( ms_id ) => {
            got_ms_id = ms_id;
        }
    }
    // ***
    return Ok( TraceAction{lf_id:got_lf_id,act_kind,ms_id:got_ms_id} );
}
