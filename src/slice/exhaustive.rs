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
use std::fs;
use crate::core::general_context::GeneralContext;
use crate::core::trace::TraceAction;
use crate::process::ana_proc::multitrace::{AnalysableMultiTrace, AnalysableMultiTraceCanal};
use crate::to_hfiles::multitrace_to_htf::write_multi_trace_into_file;
use crate::util::slicer::Slicer;



pub fn get_all_slices_rec<'a>(gen_ctx : &GeneralContext,
                      dir_name : &str,
                      id : &mut u32,
                      wide : bool,
                      ok_canals : &Vec< Vec<HashSet<TraceAction>> >,
                      rem_canals : &mut (impl Iterator<Item = &'a AnalysableMultiTraceCanal> + Clone)) {
    match rem_canals.next() {
        None => {
            let file_path = format!("{:}/s{:}", dir_name, id);
            *id = *id + 1;
            write_multi_trace_into_file(&file_path, gen_ctx, Some(&gen_ctx.co_localizations),ok_canals);
        },
        Some( rem_canal ) => {
            let orig_length = rem_canal.trace.len();
            let mut slicer = Slicer::new(&rem_canal.trace);
            while let Some(got_slice) = slicer.next() {
                if wide {
                    if got_slice.len() < orig_length/3 {
                        continue;
                    }
                }
                let mut new_trace = got_slice.iter().cloned().collect::<Vec<HashSet<TraceAction>>>();
                let mut new_ok_canals = ok_canals.clone();
                new_ok_canals.push(new_trace);
                get_all_slices_rec(gen_ctx,dir_name,id,wide,&new_ok_canals,&mut rem_canals.clone());
            }
        }
    }
}