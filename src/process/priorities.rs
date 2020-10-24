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


pub struct ProcessPriorities {
    pub emission : i32,
    pub reception : i32,
    pub in_loop : i32,
    pub step : Option<i32>,
    pub hide : i32,
    pub simulate : i32
}

impl ProcessPriorities {
    pub fn new(emission : i32,
               reception : i32,
               in_loop : i32,
               step : Option<i32>,
               hide : i32,
               simulate : i32) -> ProcessPriorities {
        return ProcessPriorities{emission,reception,in_loop,step,hide,simulate};
    }
}

impl std::string::ToString for ProcessPriorities {
    fn to_string(&self) -> String {
        let mut my_str = String::new();
        match &self.step {
            None => {},
            Some(step) => {
                my_str.push_str( &format!("step={:},",step) );
            }
        }
        my_str.push_str( &format!("emission={:},",self.emission) );
        my_str.push_str( &format!("reception={:},",self.reception) );
        my_str.push_str( &format!("loop={:}",self.in_loop) );
        return my_str;
    }
}

