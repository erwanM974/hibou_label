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

use std::fs;
use crate::core::general_context::GeneralContext;
use crate::process::ana_proc::multitrace::{AnalysableMultiTrace, AnalysableMultiTraceCanal};
use crate::slice::exhaustive::get_all_slices_rec;
use crate::slice::random::get_random_slices;


pub enum SliceGenerationConfiguration {
    Exhaustive, // all the slices
    Random(u32) // a number 'x' of random slices
}


pub fn generate_slices(gen_ctx : &GeneralContext,
                       mu_name : &str,
                       multi_trace : &AnalysableMultiTrace,
                       parent_folder : Option<&str>,
                       conf : SliceGenerationConfiguration) {
    let dir_name : String;
    match parent_folder {
        None => {
            dir_name = format!("./{:}_slices", mu_name);
        },
        Some( parent ) => {
            dir_name = format!("{:}/{:}_slices", parent, mu_name);
        }
    }
    // empties directory if exists
    match fs::remove_dir_all(&dir_name) {
        Ok(_) => {
            // do nothing
        },
        Err(e) => {
            // do nothing
        }
    }
    // creates directory
    fs::create_dir_all(&dir_name).unwrap();
    // ***
    match conf {
        SliceGenerationConfiguration::Exhaustive => {
            get_all_slices_rec(gen_ctx,&dir_name, &mut 1, &vec![], &mut multi_trace.canals.iter());
        },
        SliceGenerationConfiguration::Random( mut num_slices ) => {
            if num_slices >= get_total_num_slices(&multi_trace) {
                get_all_slices_rec(gen_ctx,&dir_name, &mut 1, &vec![], &mut multi_trace.canals.iter());
            } else {
                get_random_slices(gen_ctx,&dir_name,&mut num_slices,&multi_trace);
            }
        }
    }
}

fn get_total_num_slices(multi_trace : &AnalysableMultiTrace) -> u32 {
    let mut prod : u32 = 1;
    for canal in &multi_trace.canals {
        let canal_len = canal.trace.len();
        if canal_len > 0 {
            let num_slices_of_canal : u32 = (1..canal_len as u32).sum();
            prod = prod * num_slices_of_canal;
        }
    }
    return prod;
}
