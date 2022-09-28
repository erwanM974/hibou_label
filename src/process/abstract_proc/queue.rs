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
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::DerefMut;
use crate::core::general_context::GeneralContext;
use crate::process::abstract_proc::common::{FilterEliminationKind, HibouSearchStrategy};
use crate::process::abstract_proc::generic::*;


pub trait GenericProcessQueue<Config : AbstractConfiguration> {

    fn new() -> Self where Self : Sized;

    /** returns a next step to execute
    and if the parent state from which this step is fired
    has no other child left
    then return its ID
    so that we may then forget it / erase from memory
       **/
    fn extract_from_queue(&mut self) -> Option<(GenericStep<Config>,Option<u32>)>;

    fn enqueue_new_steps(&mut self,
                         parent_id : u32,
                         to_enqueue : Vec<GenericStep<Config>>);

    fn set_last_reached_has_no_child(&mut self);

}


pub struct BFS_ProcessQueue<Config : AbstractConfiguration> {
    queue : VecDeque< (u32,Vec<GenericStep<Config>>) >
}

impl<Config : AbstractConfiguration> GenericProcessQueue<Config> for BFS_ProcessQueue<Config> {

    fn new() -> BFS_ProcessQueue<Config> {
        return BFS_ProcessQueue{queue:VecDeque::new()};
    }

    fn extract_from_queue(&mut self) -> Option<(GenericStep<Config>,Option<u32>)> {
        match self.queue.pop_front() {
            None => {
                return None;
            },
            Some( (parent_id,mut rem) ) => {
                match rem.pop() {
                    None => {
                        panic!("should never have an empty vector here");
                    },
                    Some( got_step ) => {
                        if rem.len() > 0 {
                            self.queue.push_front((parent_id,rem) );
                            return Some( (got_step,None) );
                        } else {
                            return Some( (got_step,Some(parent_id)) );
                        }
                    }
                }
            }
        }
    }

    fn enqueue_new_steps(&mut self,
                         parent_id : u32,
                         to_enqueue : Vec<GenericStep<Config>>) {
        if to_enqueue.len() > 0 {
            self.queue.push_back( (parent_id,to_enqueue) );
        }
    }

    fn set_last_reached_has_no_child(&mut self) {}

}




pub struct DFS_ProcessQueue<Config : AbstractConfiguration> {
    queue : Vec< (u32,Vec<GenericStep<Config>>) >
}

impl<Config : AbstractConfiguration> GenericProcessQueue<Config> for DFS_ProcessQueue<Config> {

    fn new() -> DFS_ProcessQueue<Config> {
        return DFS_ProcessQueue{queue:Vec::new()};
    }

    fn extract_from_queue(&mut self) -> Option<(GenericStep<Config>,Option<u32>)> {
        match self.queue.pop() {
            None => {
                return None;
            },
            Some( (parent_id,mut rem) ) => {
                match rem.pop() {
                    None => {
                        panic!("should never have an empty vector here");
                    },
                    Some( got_step ) => {
                        if rem.len() > 0 {
                            self.queue.push((parent_id,rem) );
                            return Some( (got_step,None) );
                        } else {
                            return Some( (got_step,Some(parent_id)) );
                        }
                    }
                }
            }
        }
    }

    fn enqueue_new_steps(&mut self,
                         parent_id : u32,
                         to_enqueue : Vec<GenericStep<Config>>) {
        if to_enqueue.len() > 0 {
            self.queue.push( (parent_id,to_enqueue) );
        }
    }

    fn set_last_reached_has_no_child(&mut self) {}
}






pub struct HCS_ProcessQueue<Config : AbstractConfiguration> {
    queue : VecDeque< (u32,Vec<GenericStep<Config>>) >,
    last_reached_has_no_child : bool
}


impl<Config : AbstractConfiguration> HCS_ProcessQueue<Config> {

    fn extract_DFS(&mut self) -> Option<(GenericStep<Config>,Option<u32>)> {
        match self.queue.pop_back() {
            None => {
                return None;
            },
            Some( (parent_id,mut rem) ) => {
                match rem.pop() {
                    None => {
                        panic!("should never have an empty vector here");
                    },
                    Some( got_step ) => {
                        if rem.len() > 0 {
                            self.queue.push_back((parent_id,rem) );
                            return Some( (got_step,None) );
                        } else {
                            return Some( (got_step,Some(parent_id)) );
                        }
                    }
                }
            }
        }
    }

    fn extract_BFS(&mut self) -> Option<(GenericStep<Config>,Option<u32>)> {
        match self.queue.pop_front() {
            None => {
                return None;
            },
            Some( (parent_id,mut rem) ) => {
                match rem.pop() {
                    None => {
                        panic!("should never have an empty vector here");
                    },
                    Some( got_step ) => {
                        if rem.len() > 0 {
                            self.queue.push_front((parent_id,rem) );
                            return Some( (got_step,None) );
                        } else {
                            return Some( (got_step,Some(parent_id)) );
                        }
                    }
                }
            }
        }
    }

}

impl<Config : AbstractConfiguration> GenericProcessQueue<Config> for HCS_ProcessQueue<Config> {

    fn new() -> HCS_ProcessQueue<Config> {
        return HCS_ProcessQueue{queue:VecDeque::new(),
            last_reached_has_no_child:true};
    }

    fn extract_from_queue(&mut self) -> Option<(GenericStep<Config>,Option<u32>)> {
        match self.last_reached_has_no_child {
            true => {
                self.last_reached_has_no_child = false;
                return self.extract_BFS();
            },
            false => {
                return self.extract_DFS();
            }
        }
    }

    fn enqueue_new_steps(&mut self,
                         parent_id : u32,
                         to_enqueue : Vec<GenericStep<Config>>) {
        if to_enqueue.len() > 0 {
            self.queue.push_back( (parent_id,to_enqueue) );
        }
    }

    fn set_last_reached_has_no_child(&mut self) {
        self.last_reached_has_no_child = true;
    }

}

