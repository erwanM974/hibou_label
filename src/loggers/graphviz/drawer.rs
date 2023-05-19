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



pub struct InteractionProcessDrawer {
    pub temp_folder : String,
    pub int_repr_sd : bool,
    pub int_repr_tt : bool
}

impl InteractionProcessDrawer {
    pub fn new(temp_folder: String, int_repr_sd: bool, int_repr_tt: bool) -> Self {
        InteractionProcessDrawer { temp_folder, int_repr_sd, int_repr_tt }
    }
}


impl InteractionProcessDrawer {

    pub fn get_temp_folder(&self) -> &str {
        &self.temp_folder
    }

    pub fn get_anchor_id(&self, id: u32) -> String {
        format!("a{:}", id)
    }

    pub fn get_node_id(&self, id: u32) -> String {
        format!("n{:}", id)
    }

    pub fn get_verdict_id(&self, id: u32) -> String {
        format!("v{:}", id)
    }

    pub fn get_static_analysis_ids(&self, id: u32) -> (String, String) {
        (format!("stat{:}", id),format!("stat_anchor{:}", id))
    }

    pub fn get_step_id(&self,
                       origin_id: u32,
                       target_id: u32) -> String {
        format!("s_{:}_{:}", origin_id, target_id)
    }

}