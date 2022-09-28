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



use std::cmp::{min,max};
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

use std::collections::HashSet;
use std::fs;
use crate::core::general_context::GeneralContext;
use crate::core::trace::TraceAction;
use crate::process::ana_proc::multitrace::{AnalysableMultiTrace, AnalysableMultiTraceCanal};
use crate::to_hfiles::multitrace_to_htf::write_multi_trace_into_file;
use crate::util::slicer::Slicer;




pub fn get_random_slices(gen_ctx : &GeneralContext,
                              dir_name : &str,
                              num_slices : &mut u32,
                              multi_trace : &AnalysableMultiTrace) {
    let mut slices : HashSet< Vec<(usize,usize)> > = hashset!{};
    let mut rng = rand::thread_rng();
    while *num_slices > 0 {
        let mut new_canals_ids : Vec<(usize,usize)> = vec![];
        let mut new_canals : Vec<Vec<HashSet<TraceAction>>> = vec![];
        // ***
        for canal in &multi_trace.canals {
            let rng_indices = Uniform::from(0..canal.trace.len());
            let id1 = rng_indices.sample(&mut rng);
            let id2 = rng_indices.sample(&mut rng);
            let min_id = min(id1,id2);
            let max_id = max(id1,id2);
            let new_trace : Vec<HashSet<TraceAction>> = canal.trace[min_id..max_id].iter().cloned().collect();
            new_canals.push(new_trace);
            new_canals_ids.push( (min_id,max_id) );
        }
        // ***
        let file_path = format!("{:}/s{:}", dir_name, num_slices);
        *num_slices = *num_slices - 1;
        // ***
        if !slices.contains( &new_canals_ids ) {
            write_multi_trace_into_file(&file_path,
                                        gen_ctx,
                                        Some(&gen_ctx.co_localizations),
                                        &new_canals);
            slices.insert(new_canals_ids);
        }
    }
}