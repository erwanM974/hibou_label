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
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use graphviz_dot_builder::traits::DotBuildable;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::io::output::draw_traces::interface::draw_multitrace;
use crate::loggers::graphviz::drawer::InteractionProcessDrawer;
use crate::process::ana::node::flags::MultiTraceAnalysisFlags;


impl InteractionProcessDrawer {

    pub(crate) fn make_anchor_node(&self, state_id : u32) -> GraphVizNode {
        GraphVizNode::new(self.get_anchor_id(state_id),
                          vec![GraphvizNodeStyleItem::Label("".to_string()),
                               GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis]),
                               GraphvizNodeStyleItem::Peripheries(0),
                               GraphvizNodeStyleItem::Height(0),GraphvizNodeStyleItem::Width(0)
                          ])
    }

    pub(crate) fn make_graphic_logger_state(&self,
                                     gen_ctx : &GeneralContext,
                                     state_id : u32,
                                     interaction : &Interaction,
                                     with_mu : Option<(&CoLocalizations,&MultiTrace,&MultiTraceAnalysisFlags,bool,bool,bool)>)
                                     -> GraphVizCluster {
        // ***
        let cluster_gv_options = vec![ GraphvizNodeStyleItem::FillColor( GraphvizColor::lightgrey ),
                                       GraphvizNodeStyleItem::Label( "".to_string() )];
        // ***
        let mut cluster = GraphVizCluster::new( self.get_node_id(state_id),
                                                cluster_gv_options,
                                                vec![],
                                                vec![]);
        // ***
        let mut anchored = false;
        let mut got = 0;
        if let Some((coloc,mu,flags,is_sim,crit_loop,crit_act)) = with_mu {
            got += 1;
            let node = self.make_graphic_logger_mu(gen_ctx,
                                              coloc,
                                              mu,flags,
                                              is_sim,
                                              crit_loop,
                                              crit_act,
                                              format!("mu{}", state_id));
            cluster.add_node(node);
        }
        if got > 0 {
            let node = self.make_anchor_node(state_id);
            cluster.add_node(node);
            anchored = true;
        }
        // ***
        if self.int_repr_sd {
            got += 1;
            let node = self.make_graphic_logger_sd(gen_ctx,interaction, format!("isd{}", state_id));
            cluster.add_node(node);
        }
        if !anchored && got > 0 {
            let node = self.make_anchor_node(state_id);
            cluster.add_node(node);
            anchored = true;
        }
        if self.int_repr_tt {
            let node = self.make_graphic_logger_tt(gen_ctx,interaction, format!("itt{}", state_id));
            cluster.add_node(node);
        }
        if !anchored {
            let node = self.make_anchor_node(state_id);
            cluster.add_node(node);
        }
        // ***
        return cluster;
    }



    pub(crate) fn make_graphic_logger_sd(&self,
                                         gen_ctx : &GeneralContext,
                                         interaction : &Interaction,
                                         name : String) -> GraphVizNode {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        if interaction != &Interaction::Empty {
            draw_interaction(gen_ctx,
                             interaction,
                             &InteractionGraphicalRepresentation::AsSequenceDiagram,
                             &"temp".to_string(),
                             &self.temp_folder,
                             &name);
            // ***
            let int_image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
            // ***
            node_gv_options.push( GraphvizNodeStyleItem::Image( int_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        } else {
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            node_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        }
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name, node_gv_options)
    }

    pub(crate)fn make_graphic_logger_tt(&self,
                              gen_ctx : &GeneralContext,
                              interaction : &Interaction,
                              name : String) -> GraphVizNode {
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        if interaction != &Interaction::Empty {
            draw_interaction(gen_ctx,
                             interaction,
                             &InteractionGraphicalRepresentation::AsTerm,
                             &"temp".to_string(),
                             &self.temp_folder,
                             &name);
            // ***
            let int_image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
            // ***
            node_gv_options.push( GraphvizNodeStyleItem::Image( int_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        } else {
            node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
            node_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
        }
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name, node_gv_options)
    }

    pub(crate) fn make_graphic_logger_mu(&self,
                              gen_ctx : &GeneralContext,
                              co_localizations : &CoLocalizations,
                              multi_trace : &MultiTrace,
                              flags : &MultiTraceAnalysisFlags,
                              is_simulation : bool,
                              sim_crit_loop : bool,
                              sim_crit_act : bool,
                              name : String) -> GraphVizNode {
        // ***
        draw_multitrace(gen_ctx,
                        co_localizations,
                        multi_trace,
                        flags,
                        is_simulation,
                        sim_crit_loop,
                        sim_crit_act,
                        &self.temp_folder,
                        &name);
        // ***
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        {
            let mu_image_file_path : PathBuf = [&self.temp_folder, &format!("{}.png",name)].iter().collect();
            node_gv_options.push( GraphvizNodeStyleItem::Image( mu_image_file_path.into_os_string().to_str().unwrap().to_string() ) );
        }
        node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        // ***
        GraphVizNode::new(name, node_gv_options)
    }
}