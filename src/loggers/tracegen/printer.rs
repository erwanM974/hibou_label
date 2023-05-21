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



use std::collections::{BTreeSet, HashMap};

use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::syntax::interaction::Interaction;
use crate::loggers::tracegen::object::TraceGenLoggerObject;


pub enum TracegenProcessLoggerGeneration {
    terminal,   // at terminal nodes in the process
    accepted,   // generate a trace file only for exactly accepted traces
    //   i.e. at each new node verify express empty and if it is the case generate
    atExactDepth(u32), // generate a trace file for nodes at a specific depth
    atDepthModulo(u32) // same but for all depths equals 0 modulo smth
}


pub struct MultiTraceProcessPrinter {
    pub partition : CoLocalizations,
    pub tracegen : TracegenProcessLoggerGeneration
}

impl MultiTraceProcessPrinter {
    pub fn new(partition: CoLocalizations, tracegen: TracegenProcessLoggerGeneration) -> Self {
        MultiTraceProcessPrinter { partition, tracegen }
    }
}

impl MultiTraceProcessPrinter {

    pub(crate) fn should_generate_multi_trace_on_interaction_reached(&self,
                                                                     interaction : &Interaction,
                                                                     depth : u32) -> bool {
        match self.tracegen {
            TracegenProcessLoggerGeneration::accepted => {
                if interaction.express_empty() {
                    true
                } else {
                    false
                }
            },
            TracegenProcessLoggerGeneration::atExactDepth(exact_depth) => {
                depth == exact_depth
            },
            TracegenProcessLoggerGeneration::atDepthModulo(modulo_depth) => {
                depth % modulo_depth == 0
            },
            TracegenProcessLoggerGeneration::terminal => {
                unimplemented!();
            }
        }
    }

    pub(crate) fn get_initial_multi_trace(&self) -> TraceGenLoggerObject {
        let mut mu : MultiTrace = vec![];
        for cl in &self.partition.locs_lf_ids {
            mu.push( vec![] );
        }
        TraceGenLoggerObject::new(mu)
    }

    fn get_lf_coloc_id(&self,
                       lf_id : usize) -> Option<usize> {
        for (coloc_id,coloc) in self.partition.locs_lf_ids.iter().enumerate() {
            if coloc.contains(&lf_id) {
                return Some(coloc_id);
            }
        }
        return None;
    }

    pub(crate) fn add_actions_to_multi_trace(&self,
                                  mu : &TraceGenLoggerObject,
                                  new_actions : &BTreeSet<TraceAction>) -> TraceGenLoggerObject {
        let mut to_add : HashMap<usize,BTreeSet<TraceAction>> = hashmap!{};
        // ***
        for new_action in new_actions {
            let coloc_id = self.get_lf_coloc_id(new_action.lf_id).unwrap();
            if to_add.contains_key(&coloc_id) {
                to_add.get_mut(&coloc_id).unwrap().insert(new_action.clone());
            } else {
                to_add.insert( coloc_id, btreeset!{new_action.clone()} );
            }
        }
        // ***
        let mut new_mu = mu.mu.clone();
        for (canal_id,multiaction) in to_add {
            new_mu.get_mut(canal_id).unwrap().push(multiaction);
        }
        // ***
        TraceGenLoggerObject::new(new_mu)
    }

}

