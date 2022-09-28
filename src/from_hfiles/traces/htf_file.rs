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

use std::ops::Sub;
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
use crate::from_hfiles::traces::mutrace_for_analysis::trace_canal_from_pair_for_analysis;
use crate::from_hfiles::traces::mutrace_standalone::trace_canal_from_pair_standalone;
use crate::from_hfiles::traces::trace_actions::trace_sequence_from_pair;
use crate::process::ana_proc::multitrace::{AnalysableMultiTrace, AnalysableMultiTraceCanal};
use crate::ui::extensions::HIBOU_TRACE_FILE_EXTENSION;


pub fn parse_htf_file(file_path : &str,
                      gen_ctx : &mut GeneralContext,
                      enrich_signature : bool) -> Result<AnalysableMultiTrace,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_TRACE_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_TRACE_FILE_EXTENSION.to_string()));
    }
    // ***
    match fs::read_to_string(file_path) {
        Ok( unparsed_htf_str ) => {
            return multitrace_from_text(&unparsed_htf_str, gen_ctx, enrich_signature);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

fn complete_canals_up_to_defined_lifelines(canals : &mut Vec<AnalysableMultiTraceCanal>, gen_ctx : &mut GeneralContext) {
    let mut rem_lifelines : HashSet<usize> = gen_ctx.get_all_lfs_ids();
    for coloc in &gen_ctx.co_localizations {
        rem_lifelines = &rem_lifelines - coloc;
    }
    // ***
    for lf_id in rem_lifelines {
        gen_ctx.co_localizations.push( hashset!{lf_id} );
        canals.push( AnalysableMultiTraceCanal::new(vec![],false,false,0,0,0) );
    }
    // ***
}

pub fn multitrace_from_text(multitrace_str : &String,
                            gen_ctx : &mut GeneralContext,
                            enrich_signature : bool) -> Result<AnalysableMultiTrace,HibouParsingError> {
    match SDParser::parse(Rule::HTF_PEST_FILE, multitrace_str) {
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        },
        Ok( ref mut htf_pair ) => {
            let mut content = htf_pair.next().unwrap().into_inner();
            let first_pair : Pair<Rule> = content.next().unwrap();
            match first_pair.as_rule() {
                Rule::TRACE_SEQUENCE => {
                    let mut lifelines : HashSet<usize> = hashset!{};
                    match trace_sequence_from_pair(first_pair,gen_ctx,&hashset!{},&mut lifelines,true, enrich_signature) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( got_trace ) => {
                            gen_ctx.set_partition(vec![lifelines]);
                            let unique_canal = AnalysableMultiTraceCanal::new(got_trace,false,false,0,0,0);
                            let mut canals : Vec<AnalysableMultiTraceCanal> = vec![unique_canal];
                            return Ok( AnalysableMultiTrace::new(canals,0,0) );
                        }
                    }
                },
                Rule::MULTI_TRACE => {
                    let mut unavailable_lifelines : HashSet<usize> = HashSet::new();
                    let mut canals : Vec<AnalysableMultiTraceCanal> = Vec::new();
                    for canal_trace_pair in first_pair.into_inner() {
                        if enrich_signature {
                            match trace_canal_from_pair_standalone(canal_trace_pair,gen_ctx,&mut canals,&mut unavailable_lifelines) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( () ) => {
                                    // do nothing
                                }
                            }
                        } else {
                            match trace_canal_from_pair_for_analysis(canal_trace_pair,gen_ctx, &mut canals, &mut unavailable_lifelines) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( () ) => {
                                    // do nothing
                                }
                            }
                        }
                    }
                    complete_canals_up_to_defined_lifelines(&mut canals,gen_ctx);
                    return Ok( AnalysableMultiTrace::new(canals,0,0) );
                },
                _ => {
                    panic!("what rule then ? : {:?}", first_pair.as_rule() );
                }
            }
        }
    }
}





