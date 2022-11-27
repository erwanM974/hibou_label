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



use crate::io::output::graphviz::colors::DotTranslatable;
use crate::io::output::graphviz::edge::style::GraphvizEdgeStyle;

pub struct GraphVizEdge {
    pub origin_id : String,
    pub target_id : String,
    pub style : GraphvizEdgeStyle
}

impl GraphVizEdge {
    pub fn new(origin_id : String,
               target_id : String,
               style : GraphvizEdgeStyle) -> GraphVizEdge {
        return GraphVizEdge{origin_id,target_id,style};
    }
}

impl DotTranslatable for GraphVizEdge {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str(&(self.origin_id));
        res.push_str("->");
        res.push_str(&(self.target_id));
        res.push_str(& self.style.to_dot_string() );
        res.push_str(";");
        return res;
    }
}


