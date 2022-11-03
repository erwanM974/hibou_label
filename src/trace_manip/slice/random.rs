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



use std::cmp::{max, min};
use std::collections::HashSet;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};

use crate::core::general_context::GeneralContext;
use crate::output::to_hfiles::multitrace_to_htf::write_multi_trace_into_file;
use crate::trace_manip::slice::conf::SliceKind;



pub fn get_random_slicing(gen_ctx : &GeneralContext,
                          co_localizations : &CoLocalizations,
                         dir_name : &str,
                         num_slices : &mut u32,
                         multi_trace : &MultiTrace,
                         kind : &SliceKind,
                         wide : bool) {
    let mut slices : HashSet< Vec<(usize,usize)> > = hashset!{};
    let mut rng = rand::thread_rng();
    while *num_slices > 0 {
        let mut new_canals_ids : Vec<(usize,usize)> = vec![];
        let mut new_multi_trace : MultiTrace = vec![];
        // ***
        for trace in multi_trace {
            // ***
            let ids = get_indexes_cuts(&mut rng,trace.len(), kind, wide);
            // ***
            let new_trace : Trace = trace[ids.0..ids.1].iter().cloned().collect();
            new_multi_trace.push(new_trace);
            new_canals_ids.push( ids );
        }
        // ***
        let file_path = format!("{:}/s{:}", dir_name, num_slices);
        *num_slices = *num_slices - 1;
        // ***
        if !slices.contains( &new_canals_ids ) {
            write_multi_trace_into_file(&file_path,
                                        gen_ctx,co_localizations,
                                        &new_multi_trace);
            slices.insert(new_canals_ids);
        }
    }
}


fn get_indexes_cuts(rng : &mut ThreadRng, length : usize,kind : &SliceKind, wide : bool) -> (usize,usize) {
    if wide {
        return get_wider_cut(rng,length,kind);
    } else {
        return get_any_cut(rng,length,kind);
    }
}



fn get_any_cut(rng : &mut ThreadRng, length : usize, kind : &SliceKind) -> (usize,usize) {
    if length == 0 {
        return (0,0);
    }
    let rng_indices = Uniform::from(0..length );
    // ***
    let (id1,id2) : (usize,usize);
    match kind {
        &SliceKind::Slice => {
            id1 = rng_indices.sample(rng);
            id2 = rng_indices.sample(rng);
        },
        &SliceKind::Prefix => {
            id1 = 0;
            id2 = rng_indices.sample(rng);
        },
        &SliceKind::Suffix => {
            id1 = rng_indices.sample(rng);
            id2 = length;
        }
    }
    // ***
    let min_id = min(id1,id2);
    let max_id = max(id1,id2);
    return (min_id,max_id);
}

fn get_wider_cut(rng : &mut ThreadRng, length : usize, kind : &SliceKind) -> (usize,usize) {
    if length <= 4 {
        return get_any_cut(rng,length,kind);
    }
    let (id1,id2) : (usize,usize);
    match kind {
        &SliceKind::Slice => {
            let rng_indices = Uniform::from(0..((length/3) + 1) );
            id1 = rng_indices.sample(rng);
            id2 = length - rng_indices.sample(rng);
        },
        &SliceKind::Prefix => {
            let rng_indices = Uniform::from(0..((length/2)+ 1) );
            id1 = 0;
            id2 = min((length/2) + rng_indices.sample(rng), length );
        },
        &SliceKind::Suffix => {
            let rng_indices = Uniform::from(0..((length/2) + 1) );
            id1 = length - rng_indices.sample(rng);
            id2 = length;
        }
    }
    assert!(id2 >= id1);
    return (id1,id2);
}


