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

use crate::proc_refactoring::abstract_proc::AbstractPrioritiesConf;


pub struct CanonizationPriorities {
    pub simpl : i32,
    pub flush : i32,
    pub invert : i32,
    pub deduplicate : i32,
    pub factorize : i32,
    pub defactorize : i32
}

impl CanonizationPriorities {

    pub fn new(simpl : i32,
               flush : i32,
               invert : i32,
               deduplicate : i32,
               factorize : i32,
               defactorize : i32) -> CanonizationPriorities {
        return CanonizationPriorities{simpl,flush,invert,deduplicate,factorize,defactorize};
    }
}

impl AbstractPrioritiesConf for CanonizationPriorities {

    fn print_as_string(&self) -> String {
        let mut my_str = String::new();
        my_str.push_str( &format!("simpl={:},",self.simpl) );
        my_str.push_str( &format!("flush={:},",self.flush) );
        my_str.push_str( &format!("invert={:},",self.invert) );
        my_str.push_str( &format!("deduplicate={:}",self.deduplicate) );
        my_str.push_str( &format!("factorize={:}",self.factorize) );
        return my_str;
    }

}