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
use crate::core::general_context::GeneralContext;
use crate::core::trace::TraceAction;
use crate::loggers::tracegen::conf::TracegenProcessLoggerGeneration;
use crate::process::ana_proc::multitrace::{AnalysableMultiTrace, AnalysableMultiTraceCanal};
use crate::to_hfiles::multitrace_to_htf::write_multi_trace_into_file;

pub struct TraceGenProcessLogger {
    int_name : String,
    pub generation : TracegenProcessLoggerGeneration,
    // ***
    pub trace_map : HashMap<u32,Vec<Vec<HashSet<TraceAction>>>>,
    pub co_localizations : Vec<HashSet<usize>>
}

impl TraceGenProcessLogger {
    pub fn new(name: String,
               generation: TracegenProcessLoggerGeneration,
               co_localizations : Vec<HashSet<usize>>) -> TraceGenProcessLogger {
        // ***
        let mut empty_mutrace : Vec<Vec<HashSet<TraceAction>>> = vec![];
        for cl in &co_localizations {
            empty_mutrace.push( vec![] );
        }
        let mut trace_map : HashMap<u32,Vec<Vec<HashSet<TraceAction>>>> = hashmap!{};
        trace_map.insert( 1, empty_mutrace);
        // ***
        return TraceGenProcessLogger {
            int_name: name,
            generation,
            trace_map,
            co_localizations
        }
    }

    fn proc_name(&self) -> String {
        return format!("tracegen_{}", self.int_name);
    }

    fn get_lf_coloc_id(&self, lf_id : usize) -> Option<usize> {
        for (coloc_id,coloc) in self.co_localizations.iter().enumerate() {
            if coloc.contains(&lf_id) {
                return Some(coloc_id);
            }
        }
        return None;
    }

    pub fn initiate(&mut self) {
        // empties temp directory if exists
        match fs::remove_dir_all(format!("./{:}", self.proc_name())) {
            Ok(_) => {
                // do nothing
            },
            Err(e) => {
                // do nothing
            }
        }
        // creates temp directory
        fs::create_dir_all(format!("./{:}", self.proc_name())).unwrap();
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
                               gen_ctx : &GeneralContext, state_id : u32) {
        let mutrace_as_vec = self.trace_map.get(&state_id).unwrap();
        let file_path = format!("./{:}/{:}_t{:}", self.proc_name(), self.int_name, state_id);
        write_multi_trace_into_file(&file_path, gen_ctx,Some(&self.co_localizations),mutrace_as_vec);
    }
}