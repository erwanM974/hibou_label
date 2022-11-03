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
use crate::core::colocalizations::CoLocalizations;


use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::input::error::HibouParsingError;
use crate::input::htf::multi_trace::trace_canal_from_pair;
use crate::input::htf::trace::trace_sequence_from_pair;


#[allow(unused_imports)]
use crate::pest::Parser;
#[allow(unused_imports)]
use crate::input::htf::parser::{HtfParser,Rule};



pub fn multitrace_from_text(gen_ctx : &GeneralContext,
                            multitrace_str : &String) -> Result<(CoLocalizations,MultiTrace),HibouParsingError> {
    match HtfParser::parse(Rule::HTF_PEST_FILE, multitrace_str) {
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        },
        Ok( ref mut htf_pair ) => {
            let mut content = htf_pair.next().unwrap().into_inner();
            let first_pair : Pair<Rule> = content.next().unwrap();
            match first_pair.as_rule() {
                Rule::TRACE_SEQUENCE => {
                    let mut lifelines : HashSet<usize> = hashset!{};
                    match trace_sequence_from_pair(gen_ctx,first_pair,&hashset!{},&mut lifelines,true) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( got_trace ) => {
                            let co_localization = CoLocalizations::get_trivial_partition(lifelines.len());
                            let multi_trace : MultiTrace = vec![got_trace];
                            return Ok( (co_localization,multi_trace) );
                        }
                    }
                },
                Rule::MULTI_TRACE => {
                    let mut unavailable_lifelines : HashSet<usize> = HashSet::new();
                    let mut multi_trace : MultiTrace = vec![];
                    let mut colocs : Vec<HashSet<usize>> = vec![];
                    for canal_trace_pair in first_pair.into_inner() {
                        match trace_canal_from_pair(gen_ctx,
                                                    canal_trace_pair,
                                                    &mut colocs,
                                                    &mut multi_trace,
                                                    &mut unavailable_lifelines) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( () ) => {
                                // do nothing
                            }
                        }
                    }
                    complete_canals_up_to_defined_lifelines(gen_ctx, &mut colocs, &mut multi_trace );
                    return Ok( (CoLocalizations::new(colocs),multi_trace) );
                },
                _ => {
                    panic!("what rule then ? : {:?}", first_pair.as_rule() );
                }
            }
        }
    }
}


fn complete_canals_up_to_defined_lifelines(gen_ctx : &GeneralContext,
                                           colocs : &mut Vec<HashSet<usize>>,
                                           multi_trace : &mut MultiTrace) {
    let mut rem_lifelines : HashSet<usize> = gen_ctx.get_all_lfs_ids();
    for coloc in colocs.iter() {
        rem_lifelines = &rem_lifelines - coloc;
    }
    // ***
    for lf_id in rem_lifelines {
        colocs.push( hashset!{lf_id} );
        multi_trace.push(vec![]);
    }
    // ***
}

