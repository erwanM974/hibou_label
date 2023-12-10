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
use crate::process::explo::conf::{ExplorationConfig, ExplorationStaticLocalVerdictAnalysisProof};
use crate::process::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::process::explo::node::ExplorationNodeKind;
use crate::process::explo::step::ExplorationStepKind;
use crate::process::explo::verdict::local::ExplorationLocalVerdict;


impl GraphVizProcessDrawer<ExplorationConfig> for InteractionProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        false
    }

    fn get_temp_folder(&self) -> &str {
        self.get_temp_folder()
    }

    fn get_verdict_color(&self,
                         local_verdict: &ExplorationLocalVerdict) -> GraphvizColor {
        match local_verdict {
            ExplorationLocalVerdict::Accepting => {
                GraphvizColor::blue
            },
            ExplorationLocalVerdict::DeadLocked => {
                GraphvizColor::red
            }
        }
    }

    fn make_static_analysis_as_gvcluster(&self,
                                         _context: &ExplorationContext,
                                         _param : &ExplorationParameterization,
                                         parent_state_id: u32,
                                         verdict: &ExplorationLocalVerdict,
                                         _data_proof: &ExplorationStaticLocalVerdictAnalysisProof) -> GraphVizCluster {

        let verdict_color = <InteractionProcessDrawer as GraphVizProcessDrawer<ExplorationConfig>>::get_verdict_color(self, verdict);
        let (cluster_id,anchor_id) = self.get_static_analysis_ids(parent_state_id);
        let node = GraphVizNode::new(anchor_id,
                                     vec![
                                         GraphvizNodeStyleItem::Label("terminates".to_string()),
                                         GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
                                         GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                                         GraphvizNodeStyleItem::PenWidth(0)
                                     ]);
        let mut cluster = GraphVizCluster::new(cluster_id,
                                               vec![
                                                   GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                                                   GraphvizNodeStyleItem::PenWidth(3),
                                                   GraphvizNodeStyleItem::Color( verdict_color )
                                               ],
                                               vec![],
                                               vec![]);
        cluster.add_node(node);
        cluster
    }

    fn make_step_gvnode(&self,
                        context: &ExplorationContext,
                        _param : &ExplorationParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &ExplorationStepKind) -> GraphVizNode {

        match *step {
            ExplorationStepKind::Execute(ref frt_elt) => {
                let step_name = format!("s_{}_{}_{:?}", origin_state_id, target_state_id, frt_elt.position);
                self.make_graphic_logger_firing(&context.gen_ctx,
                                                &frt_elt.position,
                                                &frt_elt.target_actions,
                                                None,
                                                step_name)
            }
        }
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &ExplorationContext,
                                     _parem: &ExplorationParameterization,
                                     new_state_id: u32,
                                     new_node: &ExplorationNodeKind) -> GraphVizCluster {
        self.make_graphic_logger_state(&context.gen_ctx,
                                       new_state_id,
                                       &new_node.interaction,
                                       None)

    }

    fn make_node_gvitem_as_gvnode(&self,
                                  context: &ExplorationContext,
                                  _param: &ExplorationParameterization,
                                  new_state_id: u32,
                                  new_node: &ExplorationNodeKind) -> GraphVizNode {
        if self.int_repr_sd && !self.int_repr_tt {
            self.make_graphic_logger_sd(&context.gen_ctx,
                                        &new_node.interaction,
                                        self.get_node_id(new_state_id))
        } else if self.int_repr_tt && !self.int_repr_sd {
            self.make_graphic_logger_tt(&context.gen_ctx,
                                        &new_node.interaction,
                                        self.get_node_id(new_state_id))
        } else {
            GraphVizNode::new(self.get_node_id(new_state_id),vec![])
        }
    }

    fn get_node_format(&self) -> &GraphVizLoggerNodeFormat {
        if self.int_repr_sd && self.int_repr_tt {
            &GraphVizLoggerNodeFormat::AnchoredCluster
        } else {
            &GraphVizLoggerNodeFormat::SimpleNode
        }
    }

    fn get_anchor_id(&self, id: u32) -> String {
        self.get_anchor_id(id)
    }

    fn get_node_id(&self, id: u32) -> String {
        self.get_node_id(id)
    }

    fn get_verdict_id(&self, id: u32) -> String {
        self.get_verdict_id(id)
    }

    fn get_static_analysis_ids(&self, id: u32) -> (String, String) {
        self.get_static_analysis_ids(id)
    }

}