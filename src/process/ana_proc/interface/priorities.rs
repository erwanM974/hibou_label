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



use crate::process::abstract_proc::generic::AbstractPriorities;

pub struct AnalysisPriorities {
    pub emission : i32,
    pub reception : i32,
    pub multi_rdv : i32,
    pub in_loop : i32,
    pub hide : i32,
    pub simu : i32
}

impl AnalysisPriorities {

    pub fn new(emission : i32,
               reception : i32,
               multi_rdv : i32,
               in_loop : i32,
               hide : i32,
               simu : i32) -> AnalysisPriorities {
        return AnalysisPriorities{emission,reception,multi_rdv,in_loop,hide,simu};
    }

    pub fn default() -> AnalysisPriorities {
        return AnalysisPriorities::new(0,0,0,0,1,-1);
    }
}

impl AbstractPriorities for AnalysisPriorities {

    fn print_as_string(&self) -> String {
        let mut my_str = String::new();
        my_str.push_str( &format!("emission={:},",self.emission) );
        my_str.push_str( &format!("reception={:},",self.reception) );
        my_str.push_str( &format!("multi-rdv={:},",self.multi_rdv) );
        my_str.push_str( &format!("loop={:},",self.in_loop) );
        my_str.push_str( &format!("hide={:},",self.hide) );
        my_str.push_str( &format!("simu={:}",self.simu) );
        return my_str;
    }

}