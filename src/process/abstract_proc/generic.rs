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


use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;
use crate::core::general_context::GeneralContext;
use crate::process::abstract_proc::common::{FilterEliminationKind, HibouSearchStrategy};




pub trait AbstractConfiguration : Sized {
    type NodeKind : AbstractNodeKind;
    type StepKind : AbstractStepKind<Self>;
    type Priorities : AbstractPriorities;
    type Filter : AbstractFilter<Self>;
    type FilterCriterion;
    type Logger;
}

pub trait AbstractPriorities : Sized {

    fn print_as_string(&self) -> String;

}





pub trait AbstractStepKind<Config : AbstractConfiguration> : Sized {

    fn get_priority(&self, process_priorities : &Config::Priorities) -> i32;

}

pub trait AbstractFilter<Config : AbstractConfiguration> : Sized + std::string::ToString {

    fn apply_filter(&self, depth : u32, node_counter : u32, criterion : &Config::FilterCriterion) -> Option<FilterEliminationKind>;

}

pub trait AbstractNodeKind : Sized {

}

pub struct GenericNode<Config : AbstractConfiguration> {
    pub kind : Config::NodeKind,
    pub remaining_ids_to_process : HashSet<u32>,
    pub depth : u32
}

pub struct GenericStep<Config : AbstractConfiguration> {
    pub parent_id : u32,
    pub id_as_child : u32,
    pub kind : Config::StepKind
}

impl<Config : AbstractConfiguration> GenericStep<Config> {
    pub fn new(parent_id : u32,
               id_as_child : u32,
               kind : Config::StepKind) -> GenericStep<Config> {
        return GenericStep{parent_id,id_as_child,kind};
    }
}

pub struct GenericProcessManager<Config : AbstractConfiguration> {
    pub gen_ctx : GeneralContext,
    strategy : HibouSearchStrategy,
    filters : Vec<Config::Filter>,
    priorities : Config::Priorities,
    memorized_states : HashMap<u32,GenericNode<Config>>,
    process_queue : Vec<GenericStep<Config>>,
    pub loggers: Vec<Config::Logger>
}

impl<Config : AbstractConfiguration> GenericProcessManager<Config> {

    pub fn new(gen_ctx : GeneralContext,
               strategy : HibouSearchStrategy,
               filters : Vec<Config::Filter>,
               priorities : Config::Priorities,
               /*memorized_states : HashMap<u32,GenericNode<Config>>,
               process_queue : Vec<GenericStep<Config>>,*/
               loggers : Vec<Config::Logger>) -> GenericProcessManager<Config> {
        return GenericProcessManager{gen_ctx,strategy,filters,priorities,memorized_states:hashmap!{},process_queue:vec![],loggers};
    }

    pub fn pick_memorized_state(&mut self, id:u32) -> GenericNode<Config> {
        return self.memorized_states.remove(&id).unwrap();
    }

    pub fn remember_state(&mut self, id:u32, state : GenericNode<Config>) {
        assert!(!self.memorized_states.contains_key(&id));
        self.memorized_states.insert( id, state );
    }

    pub fn extract_from_queue(&mut self) -> Option<GenericStep<Config>> {
        if self.process_queue.len() > 0 {
            return Some( self.process_queue.remove(0) );
        } else {
            return None;
        }
    }

    pub fn apply_filters(&self, depth : u32, node_counter : u32, criterion : &Config::FilterCriterion) -> Option<FilterEliminationKind> {
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

    pub fn enqueue_new_steps(&mut self,
                             parent_id : u32,
                             to_enqueue : Vec<GenericStep<Config>>,
                             node_depth : u32) {
        let mut to_enqueue_reorganized : Vec<GenericStep<Config>> = Vec::new();
        {
            let mut to_enqueue_reorganize_by_priorities : HashMap<i32,Vec<GenericStep<Config>>> = HashMap::new();
            for child in to_enqueue {
                let priority = child.kind.get_priority(&self.priorities);
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
            keys.sort_by_key(|k| Reverse(*k));
            for k in keys {
                match to_enqueue_reorganize_by_priorities.get_mut(&k) {
                    None => {},
                    Some( queue ) => {
                        for child in queue.drain(..) {
                            to_enqueue_reorganized.push( child );
                        }
                    }
                }
            }
        }
        // ***
        match &self.strategy {
            &HibouSearchStrategy::DFS => {
                to_enqueue_reorganized.append(&mut self.process_queue);
                self.process_queue = to_enqueue_reorganized;
            },
            &HibouSearchStrategy::BFS => {
                self.process_queue.append( &mut to_enqueue_reorganized );
            }
        }
    }

    pub fn get_basic_options_as_strings(&self) -> Vec<String> {
        let mut options_str : Vec<String> = Vec::new();
        options_str.push( format!("strategy={}", &self.strategy.to_string()) );
        options_str.push( format!("priorities=[{}]", &self.priorities.print_as_string()) );
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





