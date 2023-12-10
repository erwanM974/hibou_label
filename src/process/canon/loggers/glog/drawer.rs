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


use graph_process_manager_loggers::graphviz::drawer::GraphVizProcessDrawer;
use graph_process_manager_loggers::graphviz::format::GraphVizLoggerNodeFormat;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::DotBuildable;

use crate::loggers::graphviz::drawer::InteractionProcessDrawer;
use crate::process::canon::conf::{CanonizationConfig, CanonizationStaticLocalVerdictAnalysisProof};
use crate::process::canon::context::CanonizationContext;
use crate::process::canon::node::CanonizationNodeKind;
use crate::process::canon::param::phase::CanonizationParameterization;
use crate::process::canon::step::CanonizationStepKind;
use crate::process::canon::verdict::local::CanonizationLocalVerdict;


impl GraphVizProcessDrawer<CanonizationConfig> for InteractionProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        false
    }

    fn get_temp_folder(&self) -> &str {
        self.get_temp_folder()
    }

    fn get_verdict_color(&self,
                         local_verdict: &CanonizationLocalVerdict) -> GraphvizColor {
        GraphvizColor::black
    }

    fn make_static_analysis_as_gvcluster(&self,
                                         _context: &CanonizationContext,
                                         _param : &CanonizationParameterization,
                                         parent_state_id: u32,
                                         verdict: &CanonizationLocalVerdict,
                                         _data_proof: &CanonizationStaticLocalVerdictAnalysisProof) -> GraphVizCluster {
        panic!("should not be called")
    }

    fn make_step_gvnode(&self,
                        context: &CanonizationContext,
                        _param : &CanonizationParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &CanonizationStepKind) -> GraphVizNode {
        let step_name = format!("s_{}_{}", origin_state_id, target_state_id);
        match *step {
            CanonizationStepKind::GoToNextPhase => {
                self.make_graphic_logger_string_label("go to next phase".to_string(),step_name)
            },
            CanonizationStepKind::Transform(ref transfo) => {
                self.make_graphic_logger_transformation(&transfo.kind,&transfo.position,step_name)
            }
        }
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &CanonizationContext,
                                     _parem: &CanonizationParameterization,
                                     new_state_id: u32,
                                     new_node: &CanonizationNodeKind) -> GraphVizCluster {
        let mut cluster = self.make_graphic_logger_state(&context.gen_ctx,
                                       new_state_id,
                                       &new_node.interaction,
                                       None);
        let colors = vec![
            GraphvizColor::lightskyblue,
            GraphvizColor::lightgoldenrod1,
            GraphvizColor::seagreen1,
            GraphvizColor::lightsalmon
        ];
        match cluster.style.iter().position(|x| match x {
            GraphvizNodeStyleItem::FillColor(_) => true,
            _ => false
        }) {
            Some(idx) => {
                cluster.style.remove(idx);
            },
            _ => {}
        };
        let color = colors.get((new_node.phase as usize) % colors.len()).unwrap();
        cluster.style.push(GraphvizNodeStyleItem::FillColor(color.clone()));
        cluster
    }

    fn make_node_gvitem_as_gvnode(&self,
                                  context: &CanonizationContext,
                                  _param: &CanonizationParameterization,
                                  new_state_id: u32,
                                  new_node: &CanonizationNodeKind) -> GraphVizNode {
        panic!("should not be called")
    }

    fn get_node_format(&self) -> &GraphVizLoggerNodeFormat {
        &GraphVizLoggerNodeFormat::AnchoredCluster
        /*if self.int_repr_sd && self.int_repr_tt {
            &GraphVizLoggerNodeFormat::AnchoredCluster
        } else {
            &GraphVizLoggerNodeFormat::SimpleNode
        }*/
    }

    fn get_anchor_id(&self, id: u32) -> String {
        self.get_anchor_id(id)
    }

    fn get_node_id(&self, id: u32) -> String {
        self.get_node_id(id)
    }

    fn get_verdict_id(&self, id: u32) -> String {
        panic!("should not be called")
    }

    fn get_static_analysis_ids(&self, id: u32) -> (String, String) {
        panic!("should not be called")
    }

}