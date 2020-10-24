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

use std::collections::HashMap;
use std::cmp::Reverse;

use crate::process::hibou_process::NextToProcess;



pub trait ProcessQueue {
    fn insert_item_left(&mut self,node:NextToProcess,priority:i32);
    fn insert_item_right(&mut self,node:NextToProcess,priority:i32);
    fn get_next(&mut self) -> Option<NextToProcess>;
}


pub struct SimpleProcessQueue {
    queue : Vec<NextToProcess>
}

impl SimpleProcessQueue {
    pub fn new() -> SimpleProcessQueue {
        return SimpleProcessQueue{queue:Vec::new()}
    }
}

impl ProcessQueue for SimpleProcessQueue {

    fn insert_item_left(&mut self,node:NextToProcess,priority:i32) {
        self.queue.insert(0,node);
    }

    fn insert_item_right(&mut self,node:NextToProcess,priority:i32) {
        self.queue.push(node);
    }

    fn get_next(&mut self) -> Option<NextToProcess> {
        if self.queue.len() > 0 {
            return Some( self.queue.remove(0) );
        } else {
            return None;
        }
    }

}



pub struct MultiProcessQueue {
    queues : HashMap<i32,Vec<NextToProcess>>
}

impl MultiProcessQueue {
    pub fn new() -> MultiProcessQueue {
        return MultiProcessQueue{queues:HashMap::new()}
    }
}

impl ProcessQueue for MultiProcessQueue {

    fn insert_item_left(&mut self,node:NextToProcess,priority:i32) {
        match self.queues.get_mut(&priority) {
            None => {
                self.queues.insert(priority,vec![node]);
            },
            Some( queue ) => {
                queue.insert(0,node);
            }
        }
    }

    fn insert_item_right(&mut self,node:NextToProcess,priority:i32) {
        match self.queues.get_mut(&priority) {
            None => {
                self.queues.insert(priority,vec![node]);
            },
            Some( queue ) => {
                queue.push(node);
            }
        }
    }

    fn get_next(&mut self) -> Option<NextToProcess> {
        let mut keys : Vec<i32> = self.queues.keys().cloned().collect();
        keys.sort_by_key(|k| Reverse(*k));
        for k in keys {
            match self.queues.get_mut(&k) {
                None => {},
                Some( queue ) => {
                    if queue.len() > 0 {
                        let next = queue.remove(0);
                        return Some(next);
                    }
                }
            }
        }
        return None;
    }

}

