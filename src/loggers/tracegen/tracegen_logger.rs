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





use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;

use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::TraceAction;
use crate::loggers::tracegen::conf::TracegenProcessLoggerGeneration;
use crate::io::file_extensions::HIBOU_TRACE_FILE_EXTENSION;
use crate::io::output::to_hfiles::trace::to_htf::write_multi_trace_into_file;


pub struct TraceGenProcessLogger {
    pub generation : TracegenProcessLoggerGeneration,
    // ***
    pub trace_map : HashMap<u32,MultiTrace>,
    pub co_localizations : CoLocalizations,
    // ***
    pub parent_folder_name : String,
    pub trace_prefix : String
    // ***
}

impl TraceGenProcessLogger {
    pub fn new(generation: TracegenProcessLoggerGeneration,
               co_localizations : CoLocalizations,
               parent_folder_name : String,
               trace_prefix : String) -> TraceGenProcessLogger {
        // ***
        let mut empty_mutrace : MultiTrace = vec![];
        for cl in &co_localizations.locs_lf_ids {
            empty_mutrace.push( vec![] );
        }
        let mut trace_map : HashMap<u32,MultiTrace> = hashmap!{};
        trace_map.insert( 1, empty_mutrace);
        // ***
        return TraceGenProcessLogger {
            generation,
            trace_map,
            co_localizations,
            parent_folder_name,
            trace_prefix
        }
    }

    fn get_lf_coloc_id(&self, lf_id : usize) -> Option<usize> {
        for (coloc_id,coloc) in self.co_localizations.locs_lf_ids.iter().enumerate() {
            if coloc.contains(&lf_id) {
                return Some(coloc_id);
            }
        }
        return None;
    }

    pub fn initiate(&mut self) {
        // empties temp directory if exists
        match fs::remove_dir_all(format!("./{:}", self.parent_folder_name)) {
            Ok(_) => {
                // do nothing
            },
            Err(e) => {
                // do nothing
            }
        }
        // creates temp directory
        fs::create_dir_all(format!("./{:}", self.parent_folder_name)).unwrap();
    }

    pub fn add_actions(&mut self, parent_id : u32, new_id : u32, new_actions : &HashSet<TraceAction>) {
        // ***
        let mut to_add : HashMap<usize,HashSet<TraceAction>> = hashmap!{};
        // ***
        for new_action in new_actions {
            let coloc_id = self.get_lf_coloc_id(new_action.lf_id).unwrap();
            if to_add.contains_key(&coloc_id) {
                to_add.get_mut(&coloc_id).unwrap().insert(new_action.clone());
            } else {
                to_add.insert( coloc_id, hashset! {new_action.clone()} );
            }
        }
        // ***
        let mut mutrace = self.trace_map.get(&parent_id).unwrap().clone();
        for (canal_id,multiaction) in to_add {
            mutrace.get_mut(canal_id).unwrap().push(multiaction);
        }
        self.trace_map.insert( new_id, mutrace );
    }

    pub fn generate_trace_file(&mut self,
                               gen_ctx : &GeneralContext,
                               state_id : u32) {
        let mutrace_as_vec = self.trace_map.get(&state_id).unwrap();
        let file_name = format!("{:}t{:}.{:}", self.trace_prefix, state_id, HIBOU_TRACE_FILE_EXTENSION);
        let path : PathBuf = [&self.parent_folder_name, &file_name].iter().collect();
        write_multi_trace_into_file(path.as_path(), gen_ctx, &self.co_localizations,mutrace_as_vec);
    }
}