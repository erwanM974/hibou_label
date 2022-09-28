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

use rand::thread_rng;
use std::cmp::Reverse;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::core::general_context::GeneralContext;
use crate::process::abstract_proc::common::{FilterEliminationKind, HibouSearchStrategy};
use crate::process::abstract_proc::generic::*;
use crate::process::abstract_proc::queue::*;


pub enum GenericProcessPriorities<Config : AbstractConfiguration> {
    Random,
    Specific(Config::Priorities)
}

impl<Config : AbstractConfiguration> GenericProcessPriorities<Config> {
    pub fn print_as_string(&self) -> String {
        match self {
            GenericProcessPriorities::Random => {
                return "random".to_string();
            },
            GenericProcessPriorities::Specific(spec_prio) => {
                return spec_prio.print_as_string();
            }
        }
    }
}

pub struct GenericProcessManager<Config : AbstractConfiguration> {
    pub gen_ctx : GeneralContext,
    strategy : HibouSearchStrategy,
    filters : Vec<Config::Filter>,
    priorities : GenericProcessPriorities<Config>,
    memorized_states : HashMap<u32,GenericNode<Config>>,
    process_queue : Box< dyn GenericProcessQueue<Config> >,
    pub loggers: Vec<Config::Logger>
}

impl<Config : 'static + AbstractConfiguration> GenericProcessManager<Config> {

    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               filters : Vec<Config::Filter>,
               priorities : GenericProcessPriorities<Config>,
               loggers : Vec<Config::Logger>) -> GenericProcessManager<Config> {
        let process_queue : Box< dyn GenericProcessQueue<Config> >;
        match &strategy {
            HibouSearchStrategy::BFS => {
                process_queue = Box::new(BFS_ProcessQueue::<Config>::new() );
            },
            HibouSearchStrategy::DFS => {
                process_queue = Box::new(DFS_ProcessQueue::<Config>::new() );
            },
            HibouSearchStrategy::HCS => {
                process_queue = Box::new(HCS_ProcessQueue::<Config>::new() );
            }
        }
        return GenericProcessManager{gen_ctx,
            strategy,
            filters,
            priorities,
            memorized_states:hashmap!{},
            process_queue,
            loggers};
    }

    pub fn pick_memorized_state(&mut self, id:u32) -> GenericNode<Config> {
        return self.memorized_states.remove(&id).unwrap();
    }

    pub fn remember_state(&mut self, id:u32, state : GenericNode<Config>) {
        assert!(!self.memorized_states.contains_key(&id));
        self.memorized_states.insert( id, state );
    }

    pub fn extract_from_queue(&mut self) -> Option<GenericStep<Config>> {
        match self.process_queue.extract_from_queue() {
            None => {
                return None;
            },
            Some( (step,_) ) => {
                return Some(step);
            }
        }
    }

    pub fn apply_filters(&self,
                         depth : u32,
                         node_counter : u32,
                         criterion : &Config::FilterCriterion) -> Option<FilterEliminationKind> {
        for filter in &self.filters {
            match filter.apply_filter(depth,node_counter,criterion) {
                None => {},
                Some( elim_kind) => {
                    return Some(elim_kind);
                }
            }
        }
        return None;
    }

    fn reorganize_by_priority(priorities : &Config::Priorities, to_enqueue : Vec<GenericStep<Config>>) -> Vec<GenericStep<Config>> {
        let mut to_enqueue_reorganized : Vec<GenericStep<Config>> = Vec::new();
        {
            let mut to_enqueue_reorganize_by_priorities : HashMap<i32,Vec<GenericStep<Config>>> = HashMap::new();
            for child in to_enqueue {
                let priority = child.kind.get_priority(priorities);
                // ***
                match to_enqueue_reorganize_by_priorities.get_mut(&priority) {
                    None => {
                        to_enqueue_reorganize_by_priorities.insert(priority,vec![ child ]);
                    },
                    Some( queue ) => {
                        queue.push(child );
                    }
                }
            }
            // ***
            let mut keys : Vec<i32> = to_enqueue_reorganize_by_priorities.keys().cloned().collect();
            keys.sort();
            for k in keys {
                match to_enqueue_reorganize_by_priorities.get_mut(&k) {
                    None => {},
                    Some( queue ) => {
                        to_enqueue_reorganized.append( queue );
                    }
                }
            }
        }
        return to_enqueue_reorganized;
    }

    pub fn queue_set_last_reached_has_no_child(&mut self) {
        self.process_queue.set_last_reached_has_no_child();
    }

    pub fn enqueue_new_steps(&mut self,
                             parent_id : u32,
                             to_enqueue : Vec<GenericStep<Config>>,
                             node_depth : u32) {
        match &self.priorities {
            GenericProcessPriorities::Random => {
                let mut new_steps = to_enqueue;
                new_steps.shuffle(&mut thread_rng());
                self.process_queue.enqueue_new_steps(parent_id,new_steps);
            },
            GenericProcessPriorities::Specific(priorities) => {
                let to_enqueue_reorganized = GenericProcessManager::reorganize_by_priority(priorities,to_enqueue);
                self.process_queue.enqueue_new_steps(parent_id,to_enqueue_reorganized);
            }
        }
    }

    pub fn get_basic_options_as_strings(&self) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        options_str.push( format!("strategy={}", &self.strategy.to_string()) );
        options_str.push( format!("priorities={}", &self.priorities.print_as_string()) );
        {
            let mut rem_filter = self.filters.len();
            let mut filters_str = "filters=[".to_string();
            for filter in &self.filters {
                filters_str.push_str( &filter.to_string() );
                rem_filter = rem_filter - 1;
                if rem_filter > 0 {
                    filters_str.push_str( "," );
                }
            }
            filters_str.push_str( "]" );
            options_str.push( filters_str );
        }
        return options_str;
    }
}





