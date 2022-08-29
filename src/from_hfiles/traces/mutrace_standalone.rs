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
use crate::from_hfiles::traces::trace_actions::trace_sequence_from_pair;
use crate::process::ana_proc::multitrace::{AnalysableMultiTrace, AnalysableMultiTraceCanal};
use crate::ui::extensions::HIBOU_TRACE_FILE_EXTENSION;



pub fn trace_canal_from_pair_standalone(trace_pair : Pair<Rule>,
                                          gen_ctx : &mut GeneralContext,
                                          canals : &mut Vec<AnalysableMultiTraceCanal>,
                                          unavailable_lifelines : &mut HashSet<usize>) -> Result<(),HibouParsingError> {
    // ***
    let mut lifelines : HashSet<usize> = HashSet::new();
    // ***
    let mut content = trace_pair.into_inner();
    let canal_lfs_pair = content.next().unwrap();
    let trace_sequence_pair = content.next().unwrap();
    // ***
    let add_lfs : bool;
    // ***
    match canal_lfs_pair.as_rule() {
        Rule::CANAL_LIFELINES_any => {
            add_lfs = true;
        },
        Rule::CANAL_LIFELINES_all => {
            return Err( HibouParsingError::IllDefinedTraceComponents("cannot use #all when parsing multi-trace without signature from .hsf file".to_string()) )
        },
        Rule::CANAL_LIFELINES_spec => {
            add_lfs = false;
            // ***
            for trace_lf_pair in canal_lfs_pair.into_inner() {
                let got_lf_id : usize;
                let lf_name : String  = trace_lf_pair.as_str().chars().filter(|c| !c.is_whitespace()).collect();
                match gen_ctx.get_lf_id(&lf_name) {
                    None => {
                        got_lf_id = gen_ctx.add_lf(lf_name);
                    },
                    Some( lf_id ) => {
                        got_lf_id = lf_id;
                    }
                }
                lifelines.insert(got_lf_id);
            }
        },
        _ => {
            panic!("what rule then ? : {:?}", canal_lfs_pair.as_rule() );
        }
    }
    // ***
    match trace_sequence_from_pair(trace_sequence_pair,gen_ctx,unavailable_lifelines,&mut lifelines, add_lfs, true) {
        Err(e) => {
            return Err(e);
        },
        Ok( trace ) => {
            let new_canal = AnalysableMultiTraceCanal::new(trace,false,true,0,0,0);
            canals.push(new_canal);
            unavailable_lifelines.extend(lifelines.clone());
            gen_ctx.co_localizations.push( lifelines );
        }
    }
    // ***
    return Ok( () );
}






