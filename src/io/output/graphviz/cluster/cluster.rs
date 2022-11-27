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
use crate::io::output::graphviz::edge::edge::GraphVizEdge;
use crate::io::output::graphviz::node::node::GraphVizNode;
use crate::io::output::graphviz::node::style::GraphvizNodeStyle;

pub struct GraphVizCluster {
    pub id : String,
    pub style : GraphvizNodeStyle,
    pub nodes : Vec<Box<dyn DotTranslatable>>,
    pub edges : Vec<GraphVizEdge>
}

impl GraphVizCluster {
    pub fn new(id : String,
               style : GraphvizNodeStyle,
               nodes : Vec<Box<dyn DotTranslatable>>,
               edges : Vec<GraphVizEdge>) -> GraphVizCluster {
        return GraphVizCluster{id,style,nodes,edges};
    }
}

impl DotTranslatable for GraphVizCluster {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str(&format!("subgraph cluster_{:} {{\n",self.id));
        // ***
        for item in &self.style {
            res.push_str(&format!("{};\n",item.to_dot_string()) );
        }
        // ***
        for node in &self.nodes {
            res.push_str("\t");
            res.push_str(& node.to_dot_string() );
            res.push_str("\n");
        }
        for edge in &self.edges {
            res.push_str("\t");
            res.push_str(& edge.to_dot_string() );
            res.push_str("\n");
        }
        res.push_str("}");
        return res;
    }
}

