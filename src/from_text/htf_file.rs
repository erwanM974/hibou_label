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

use crate::from_text::error::HibouParsingError;

use crate::from_text::parser::*;

pub static HIBOU_TRACE_FILE_EXTENSION : &'static str = "htf";

pub fn parse_htf_file(file_path : &str,
                      gen_ctx : &GeneralContext) -> Result<AnalysableMultiTrace,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_TRACE_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_TRACE_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_htf_str ) => {
            return multitrace_from_text(&unparsed_htf_str, gen_ctx);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

fn complete_canals_up_to_defined_lifelines(canals : &mut Vec<MultiTraceCanal>, gen_ctx : &GeneralContext) {
    let mut rem_lifelines : HashSet<usize> = HashSet::from_iter((0..gen_ctx.get_lf_num()).collect::<Vec<usize>>().iter().cloned());
    for canal in canals.iter() {
        for lf_id in &canal.lifelines {
            rem_lifelines.remove(lf_id);
        }
    }
    // ***
    for lf_id in rem_lifelines {
        let lifelines : HashSet<usize> = HashSet::from_iter( vec![lf_id].iter().cloned() );
        let trace : Vec<TraceAction> = Vec::new();
        canals.push( MultiTraceCanal::new(lifelines,trace,false,0,0,0) );
    }
    // ***
}

pub fn multitrace_from_text(multitrace_str : &String,
                            gen_ctx : &GeneralContext) -> Result<AnalysableMultiTrace,HibouParsingError> {
    match SDParser::parse(Rule::HTF_PEST_FILE, multitrace_str) {
        Err(e) => {
            return Err( HibouParsingError::MatchError(e.to_string()) );
        },
        Ok( ref mut htf_pair ) => {
            let mut content = htf_pair.next().unwrap().into_inner();
            let first_pair : Pair<Rule> = content.next().unwrap();
            match first_pair.as_rule() {
                Rule::TRACE => {
                    let mut canals : Vec<MultiTraceCanal> = Vec::new();
                    match trace_canal_from_pair(first_pair,gen_ctx,&HashSet::new()) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( trace_canal ) => {
                            canals.push( trace_canal );
                        }
                    }
                    complete_canals_up_to_defined_lifelines(&mut canals,gen_ctx);
                    return Ok( AnalysableMultiTrace::new(canals,0) );
                },
                Rule::MULTI_TRACE => {
                    let mut unavailable_lifelines : HashSet<usize> = HashSet::new();
                    let mut canals : Vec<MultiTraceCanal> = Vec::new();
                    for trace_pair in first_pair.into_inner() {
                        match trace_canal_from_pair(trace_pair,gen_ctx,&unavailable_lifelines) {
                            Err(e) => {
                                return Err(e);
                            },
                            Ok( trace_canal ) => {
                                unavailable_lifelines = unavailable_lifelines.union( &trace_canal.lifelines ).cloned().collect();
                                canals.push( trace_canal );
                            }
                        }
                    }
                    complete_canals_up_to_defined_lifelines(&mut canals,gen_ctx);
                    return Ok( AnalysableMultiTrace::new(canals,0) );
                },
                _ => {
                    panic!("what rule then ? : {:?}", first_pair.as_rule() );
                }
            }
        }
    }
}

pub fn trace_canal_from_pair(trace_pair : Pair<Rule>,
                             gen_ctx : &GeneralContext,
                             unavailable_lifelines : &HashSet<usize>) -> Result<MultiTraceCanal,HibouParsingError> {
    let mut trace : Vec<TraceAction> = Vec::new();
    let mut lifelines : HashSet<usize> = HashSet::new();
    // ***
    let mut content = trace_pair.into_inner();
    // ***
    match content.next() {
        None => {},
        Some( first_pair ) => {
            match first_pair.as_rule() {
                Rule::CANAL_ANY => {
                    match inner_trace_from_pairs(&mut content,gen_ctx,unavailable_lifelines,&mut trace, &mut lifelines, true) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok(_) => {}
                    }
                },
                Rule::CANAL_ALL => {
                    let all_lfs : HashSet<usize> = HashSet::from_iter((0..gen_ctx.get_lf_num()).collect::<Vec<usize>>().iter().cloned());
                    lifelines = all_lfs;
                    match inner_trace_from_pairs(&mut content,gen_ctx,unavailable_lifelines,&mut trace, &mut lifelines, true) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok(_) => {}
                    }
                },
                Rule::CANAL_LIFELINES => {
                    for trace_lf_pair in first_pair.into_inner() {
                        let lf_name : String  = trace_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                        match gen_ctx.get_lf_id(&lf_name) {
                            None => {
                                return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name));
                            },
                            Some( lf_id ) => {
                                lifelines.insert(lf_id);
                            }
                        }
                    }
                    match inner_trace_from_pairs(&mut content,gen_ctx,unavailable_lifelines,&mut trace, &mut lifelines, false) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok(_) => {}
                    }
                },
                _ => {
                    panic!("what rule then ? : {:?}", first_pair.as_rule() );
                }
            }
        }
    }
    // ***
    return Ok( MultiTraceCanal::new(lifelines,trace,false,0,0,0) );
}

pub fn inner_trace_from_pairs(content : &mut Pairs<Rule>,
                              gen_ctx : &GeneralContext,
                              unavailable_lifelines : &HashSet<usize>,
                              trace : &mut Vec<TraceAction>,
                              lifelines : &mut HashSet<usize>,
                              add_lfs : bool) -> Result<(),HibouParsingError> {
    for action_pair in content {
        match trace_action_from_text(action_pair,gen_ctx) {
            Err(e) => {
                return Err(e);
            },
            Ok( action ) => {
                if unavailable_lifelines.contains(&action.lf_id) {
                    return Err( HibouParsingError::NonDisjointTraceComponents );
                } else {
                    if add_lfs {
                        lifelines.insert( action.lf_id);
                    }
                }
                trace.push( action );
            }
        }
    }
    return Ok( () );
}


fn trace_action_from_text(action_pair : Pair<Rule>, gen_ctx : &GeneralContext) -> Result<TraceAction,HibouParsingError> {
    let mut contents = action_pair.into_inner();
    // ***
    let lf_pair : Pair<Rule> = contents.next().unwrap();
    let lf_name : String  = lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
    match gen_ctx.get_lf_id(&lf_name) {
        None => {
            return Err( HibouParsingError::MissingLifelineDeclarationError(lf_name) );
        },
        Some( lf_id ) => {
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
            match gen_ctx.get_ms_id(&ms_name) {
                None => {
                    return Err( HibouParsingError::MissingMessageDeclarationError(ms_name) );
                },
                Some( ms_id ) => {
                    return Ok( TraceAction{lf_id,act_kind,ms_id} );
                }
            }
        }
    }
}








