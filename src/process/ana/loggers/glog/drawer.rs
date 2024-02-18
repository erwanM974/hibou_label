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


use std::path::PathBuf;
use graph_process_manager_loggers::graphviz::drawer::GraphVizProcessDrawer;
use graph_process_manager_loggers::graphviz::format::{GraphVizLoggerNodeFormat, GraphVizProcessLoggerLayout};
use graph_process_manager_loggers::graphviz::logger::GenericGraphVizLogger;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape};
use graphviz_dot_builder::traits::{DotBuildable, GraphVizOutputFormat};

use crate::loggers::graphviz::drawer::InteractionProcessDrawer;
use crate::process::ana::conf::AnalysisConfig;
use crate::process::ana::context::AnalysisContext;
use crate::process::ana::node::node::AnalysisNodeKind;
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::step::AnalysisStepKind;
use crate::process::ana::verdict::local::AnalysisLocalVerdict;
use crate::process::ana::conf::AnalysisStaticLocalVerdictAnalysisProof;
use crate::process::ana::handling::local_analysis::perform_local_analysis;

impl GraphVizProcessDrawer<AnalysisConfig> for InteractionProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        true
    }

    fn get_temp_folder(&self) -> &str {
        self.get_temp_folder()
    }

    fn get_verdict_color(&self,
                         local_verdict: &AnalysisLocalVerdict) -> GraphvizColor {
        match local_verdict {
            AnalysisLocalVerdict::Cov => {
                GraphvizColor::blue3 // 0 0 205
            },
            AnalysisLocalVerdict::TooShort => {
                GraphvizColor::cyan3 // 0 205 205
            },
            AnalysisLocalVerdict::MultiPref => {
                GraphvizColor::slateblue3 // 105 89 205
            },
            AnalysisLocalVerdict::Slice => {
                GraphvizColor::darkorchid3 // 154 50 205
            },
            AnalysisLocalVerdict::Inconc(_) => {
                GraphvizColor::deeppink3 // 205 16 118
            },
            AnalysisLocalVerdict::Out(_) => {
                GraphvizColor::red3 // 205 0 0
            },
            AnalysisLocalVerdict::OutSim(_) => {
                GraphvizColor::crimson
            }
        }
    }

    fn make_static_analysis_as_gvcluster(&self,
                                         context: &AnalysisContext,
                                         param : &AnalysisParameterization,
                                         parent_state_id: u32,
                                         verdict: &AnalysisLocalVerdict,
                                         data_proof: &AnalysisStaticLocalVerdictAnalysisProof) -> GraphVizCluster {

        let verdict_color = <InteractionProcessDrawer as GraphVizProcessDrawer<AnalysisConfig>>::get_verdict_color(self,verdict);
        let (static_ana_id,static_anchor_id) = self.get_static_analysis_ids(parent_state_id);
        let drawer = InteractionProcessDrawer::new("./temp".to_string(),
                                                   self.int_repr_sd,
                                                   self.int_repr_tt);
        let sub_graphic_logger: GenericGraphVizLogger<AnalysisConfig> = GenericGraphVizLogger::new(Box::new(drawer),
                                                                                                   GraphVizOutputFormat::png,
                                                                                                   GraphVizProcessLoggerLayout::Vertical,
                                                                                                   false,
                                                                                                   ".".to_string(),
                                                                                                   static_ana_id.clone());
        let max_depth = match param.locana.as_ref().unwrap().max_depth {
            None => None,
            Some(x) => Some(x)
        };
        perform_local_analysis(&context.gen_ctx,
                               data_proof.local_coloc.clone(),
                               &param.ana_kind,
                               false, // Partial Order Reduction is not useful on a single coloc trace
                               &max_depth,
                               data_proof.local_interaction.clone(),
                               data_proof.local_multi_trace.clone(),
                               data_proof.local_flags.clone(),
                               vec![Box::new(sub_graphic_logger)]);
        // ***
        let subproc_image_file_path : PathBuf = [&".".to_string(), &format!("{}.png",static_ana_id)].iter().collect();
        // ***
        let node = GraphVizNode::new(static_anchor_id,
                                     vec![
                                         GraphvizNodeStyleItem::Image( subproc_image_file_path.into_os_string().to_str().unwrap().to_string() ),
                                         GraphvizNodeStyleItem::Label( "".to_string() ),
                                         GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
                                         GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
                                         GraphvizNodeStyleItem::PenWidth(0)
                                     ]);
        let mut cluster = GraphVizCluster::new(static_ana_id,
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
                        context: &AnalysisContext,
                        param : &AnalysisParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &AnalysisStepKind) -> GraphVizNode {

        let step_name = format!("s_{}_{}", origin_state_id, target_state_id);
        match *step {
            AnalysisStepKind::EliminateNoLongerObserved(ref lfs_to_hide) => {
                self.make_graphic_logger_hiding(&context.gen_ctx,lfs_to_hide,step_name)
            },
            AnalysisStepKind::Execute(ref frt_elt, ref consu_set, ref sim_map) => {
                self.make_graphic_logger_firing(&context.gen_ctx,
                                                &frt_elt.position,
                                                &frt_elt.target_actions,
                                                Some((&context.co_localizations,consu_set,sim_map)),
                                                step_name)
            }
        }
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &AnalysisContext,
                                     param: &AnalysisParameterization,
                                     new_state_id: u32,
                                     new_node: &AnalysisNodeKind) -> GraphVizCluster {
        let (has_simulation,sim_crit_loop,sim_crit_act) = param.ana_kind.get_sim_crits();
        self.make_graphic_logger_state(&context.gen_ctx,
                                       new_state_id,
                                       &new_node.interaction,
                                       Some((&context.co_localizations,&context.multi_trace,&new_node.flags,has_simulation,sim_crit_loop,sim_crit_act)))

    }

    fn make_node_gvitem_as_gvnode(&self,
                                  context: &AnalysisContext,
                                  param: &AnalysisParameterization,
                                  new_state_id: u32,
                                  new_node: &AnalysisNodeKind) -> GraphVizNode {
        let (has_simulation,sim_crit_loop,sim_crit_act) = param.ana_kind.get_sim_crits();
        self.make_graphic_logger_mu(&context.gen_ctx,
                                    &context.co_localizations,
                                    &context.multi_trace,
                                    &new_node.flags,
                                    has_simulation,
                                    sim_crit_loop,
                                    sim_crit_act,
                                    self.get_node_id(new_state_id))
    }

    fn get_node_format(&self) -> &GraphVizLoggerNodeFormat {
        if self.int_repr_sd || self.int_repr_tt {
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