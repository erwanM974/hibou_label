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


use crate::io::output::graphviz::cluster::cluster::GraphVizCluster;
use crate::io::output::graphviz::colors::DotTranslatable;
use crate::io::output::graphviz::edge::edge::GraphVizEdge;
use crate::io::output::graphviz::node::style::GraphvizNodeStyle;


pub struct GraphVizDiGraph {
    pub clusters : Vec<GraphVizCluster>,
    pub nodes : Vec<Box<dyn DotTranslatable>>,
    pub edges : Vec<GraphVizEdge>
}

impl GraphVizDiGraph {
    pub fn new() -> GraphVizDiGraph {
        return GraphVizDiGraph{clusters:vec![],nodes:vec![],edges:vec![]};
    }

    pub fn get_specific_cluster(&mut self, cluster_id : usize) -> Option<&mut GraphVizCluster> {
        return self.clusters.get_mut(cluster_id);
    }
}

impl DotTranslatable for GraphVizDiGraph {
    fn to_dot_string(&self) -> String {
        let mut res = String::new();
        res.push_str("digraph G {");
        res.push_str("\ncompound=true;" );
        for cluster in &self.clusters {
            res.push_str("\n\t");
            res.push_str(& cluster.to_dot_string() );
        }
        for node in &self.nodes {
            res.push_str("\n\t");
            res.push_str(& node.to_dot_string() );
        }
        for edge in &self.edges {
            res.push_str("\n\t");
            res.push_str(& edge.to_dot_string() );
        }
        res.push_str("\n}");
        return res;
    }
}

