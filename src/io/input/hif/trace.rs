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
use pest::iterators::Pair;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::general_context::GeneralContext;
use crate::io::input::error::HibouParsingError;

#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::io::input::hif::parser::{HifParser,Rule};

pub fn sync_acts_from_pair(gen_ctx : &GeneralContext,
                               sync_acts_pair : Pair<Rule>)
                               -> Result<HashSet<TraceAction>,HibouParsingError> {
    match sync_acts_pair.as_rule() {
        Rule::TRACE_ACTION => {
            match trace_action_from_text(gen_ctx,sync_acts_pair) {
                Err(e) => {return Err(e);},
                Ok( action ) => {
                    return Ok(hashset!{action});
                }
            }
        },
        Rule::TRACE_ACTION_SET => {
            match get_trace_multi_action(gen_ctx,sync_acts_pair) {
                Err(e) => {return Err(e);},
                Ok( multi_action ) => {
                    return Ok(multi_action);
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", sync_acts_pair.as_rule() );
        }
    }
}


fn get_trace_multi_action(gen_ctx : &GeneralContext,
                          multi_act_pair : Pair<Rule>) -> Result<HashSet<TraceAction>,HibouParsingError> {
    let mut multi_action = hashset!{};
    for action_pair in multi_act_pair.into_inner() {
        match trace_action_from_text(gen_ctx,action_pair) {
            Err(e) => {return Err(e);},
            Ok( action ) => {
                multi_action.insert(action);
            }
        }
    }
    return Ok(multi_action);
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
