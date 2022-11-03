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
use crate::core::execution::trace::multitrace::MultiTrace;

use crate::core::general_context::GeneralContext;
use crate::input::error::HibouParsingError;
use crate::input::htf::trace::trace_sequence_from_pair;



#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::input::htf::parser::{HtfParser,Rule};



pub fn trace_canal_from_pair(gen_ctx : &GeneralContext,
                                       trace_pair : Pair<Rule>,
                                       co_localizations : &mut Vec<HashSet<usize>>,
                                       multi_trace : &mut MultiTrace,
                             unavailable_lifelines : &mut HashSet<usize>) -> Result<(),HibouParsingError> {
    // ***
    let mut lifelines : HashSet<usize> = HashSet::new();
    // ***
    let mut content = trace_pair.into_inner();
    let canal_lfs_pair = content.next().unwrap();
    let trace_sequence_pair = content.next().unwrap();
    // ***
    match canal_lfs_pair.as_rule() {
        Rule::CANAL_LIFELINES_any => {
            match trace_sequence_from_pair(gen_ctx,trace_sequence_pair,unavailable_lifelines,&mut lifelines, true) {
                Err(e) => {
                    return Err(e);
                },
                Ok( trace ) => {
                    unavailable_lifelines.extend(lifelines.clone());
                    co_localizations.push(lifelines);
                    multi_trace.push(trace);
                }
            }
        },
        Rule::CANAL_LIFELINES_all => {
            let mut remaining_lfs : HashSet<usize> = gen_ctx.get_all_lfs_ids();
            remaining_lfs = &remaining_lfs - unavailable_lifelines;
            match trace_sequence_from_pair(gen_ctx,trace_sequence_pair,unavailable_lifelines,&mut remaining_lfs, false) {
                Err(e) => {
                    return Err(e);
                },
                Ok( trace ) => {
                    unavailable_lifelines.extend(remaining_lfs.clone());
                    co_localizations.push(remaining_lfs);
                    multi_trace.push(trace);
                }
            }
        },
        Rule::CANAL_LIFELINES_spec => {
            let mut lifelines : HashSet<usize> = hashset!{};
            for trace_lf_pair in canal_lfs_pair.into_inner() {
                let lf_name : String  = trace_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                match gen_ctx.get_lf_id(&lf_name) {
                    None => {
                        return Err( HibouParsingError::MissingLifelineOrGateDeclarationError(lf_name));
                    },
                    Some( lf_id ) => {
                        lifelines.insert(lf_id);
                    }
                }
            }
            // ***
            match trace_sequence_from_pair(gen_ctx,trace_sequence_pair,unavailable_lifelines,&mut lifelines, false) {
                Err(e) => {
                    return Err(e);
                },
                Ok( trace ) => {
                    unavailable_lifelines.extend(lifelines.clone());
                    co_localizations.push(lifelines);
                    multi_trace.push(trace);
                }
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", canal_lfs_pair.as_rule() );
        }
    }
    // ***
    return Ok( () );
}








