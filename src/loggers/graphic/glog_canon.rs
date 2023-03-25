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


use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyle, GraphvizEdgeStyleItem, GvArrowHeadSide, GvArrowHeadStyle};
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem};
use graphviz_dot_builder::traits::DotBuildable;

use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::position::position::Position;

use crate::core::transformation::transfokind::InteractionTransformationKind;

use crate::loggers::graphic::get_graph::filter::make_graphic_logger_filter;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::loggers::graphic::get_graph::state::make_graphic_logger_state;
use crate::loggers::graphic::get_graph::transition::{make_graphic_logger_string_label, make_graphic_logger_transformation};
use crate::loggers::graphic::graphic_logger::GraphicProcessLogger;
use crate::process::canon_proc::interface::logger::CanonizationLogger;
use crate::process::canon_proc::manager::IntRefOrOldIdRef;
use crate::process::canon_proc::transformations::phases::CanonizationPhase;


impl CanonizationLogger for GraphicProcessLogger {

    fn log_init(&mut self, interaction: &Interaction, gen_ctx: &GeneralContext) {
        {
            let mut cluster_gv_options : GraphvizNodeStyle = Vec::new();
            cluster_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::lightblue1 ));
            cluster_gv_options.push(GraphvizNodeStyleItem::Label("phase 1".to_string()));
            let cluster_phase_1 = GraphVizCluster::new("phase1".to_string(),cluster_gv_options,vec![],vec![]);
            self.graph.add_cluster(cluster_phase_1);
        }
        {
            let mut cluster_gv_options : GraphvizNodeStyle = Vec::new();
            cluster_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::palegreen ));
            cluster_gv_options.push(GraphvizNodeStyleItem::Label("phase 2".to_string()));
            let cluster_phase_2 = GraphVizCluster::new("phase2".to_string(),cluster_gv_options,vec![],vec![]);
            self.graph.add_cluster(cluster_phase_2);
        }
        let init_node = make_graphic_logger_state(&self.temp_folder,gen_ctx,1,interaction,self.int_repr_sd,self.int_repr_tt,None);
        let phase_cluster = self.graph.get_specific_cluster(0).unwrap();
        phase_cluster.add_cluster(init_node);
    }

    fn log_term(&mut self, options_as_str: &Vec<String>) {
        self.terminate(options_as_str);
    }

    fn log_phase_change(&mut self,
                        gen_ctx : &GeneralContext,
                        parent_state_id: u32,
                        new_state_id: u32,
                        target_interaction : &IntRefOrOldIdRef) {
        // ***
        let state_transfo = make_graphic_logger_string_label(&self.temp_folder,new_state_id,"Phase Change".to_string());
        // *** Transition To Transformation
        let tran_to_transfo : GraphVizEdge;
        {
            let tran_gv_options = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) ];
            tran_to_transfo = GraphVizEdge::new(format!("a{:}", parent_state_id),
                                                Some(format!("n{}",parent_state_id)),
                                                state_transfo.id.clone(),
                                                None,
                                                tran_gv_options);
        }
        // *** Resulting New Node
        let target_int_id : u32;
        match target_interaction {
            IntRefOrOldIdRef::OldIDRef(old_target_id) => {
                target_int_id = *old_target_id;
            },
            IntRefOrOldIdRef::IntRef(interaction) => {
                target_int_id = new_state_id;
                let new_node = make_graphic_logger_state(&self.temp_folder,gen_ctx,new_state_id,interaction,self.int_repr_sd,self.int_repr_tt,None);
                let phase_cluster = self.graph.get_specific_cluster(1).unwrap();
                phase_cluster.add_cluster(new_node);
            }
        }
        // *** Transition To New Node
        let tran_to_new : GraphVizEdge;
        {
            let tran_gv_options = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) ];
            tran_to_new = GraphVizEdge::new(state_transfo.id.clone(),
                                            None,
                                            format!("a{:}", target_int_id),
                                            Some(format!("n{}",target_int_id)),
                                            tran_gv_options);
        }
        // ***
        self.graph.add_node(state_transfo);
        self.graph.add_edge(tran_to_transfo);
        self.graph.add_edge(tran_to_new);
    }

    fn log_transformation(&mut self,
                          gen_ctx : &GeneralContext,
                          phase : &CanonizationPhase,
                          parent_state_id : u32,
                          new_state_id : u32,
                          transfo_kind : &InteractionTransformationKind,
                          position : &Position,
                          target_interaction : &IntRefOrOldIdRef) {
        // ***
        let state_transfo = make_graphic_logger_transformation(&self.temp_folder,new_state_id,transfo_kind,position);
        // *** Transition To Transformation
        let tran_to_transfo : GraphVizEdge;
        {
            let tran_gv_options = vec![GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) )];
            tran_to_transfo = GraphVizEdge::new(format!("a{:}", parent_state_id),
                                                Some(format!("n{}",parent_state_id)),
                                                state_transfo.id.clone(),
                                                None,
                                                tran_gv_options);
        }
        // *** Resulting New Node
        let target_int_id : u32;
        match target_interaction {
            IntRefOrOldIdRef::OldIDRef(old_target_id) => {
                target_int_id = *old_target_id;
                match phase {
                    CanonizationPhase::FirstDefactorize => {
                        let phase_cluster = self.graph.get_specific_cluster(0).unwrap();
                        phase_cluster.add_node(state_transfo);
                    },
                    CanonizationPhase::SecondFactorize => {
                        let phase_cluster = self.graph.get_specific_cluster(1).unwrap();
                        phase_cluster.add_node(state_transfo);
                    }
                }
            },
            IntRefOrOldIdRef::IntRef(interaction) => {
                target_int_id = new_state_id;
                let new_node = make_graphic_logger_state(&self.temp_folder,gen_ctx,new_state_id,interaction,self.int_repr_sd,self.int_repr_tt,None);
                match phase {
                    CanonizationPhase::FirstDefactorize => {
                        let phase_cluster = self.graph.get_specific_cluster(0).unwrap();
                        phase_cluster.add_node(state_transfo);
                        phase_cluster.add_cluster(new_node);
                    },
                    CanonizationPhase::SecondFactorize => {
                        let phase_cluster = self.graph.get_specific_cluster(1).unwrap();
                        phase_cluster.add_node(state_transfo);
                        phase_cluster.add_cluster(new_node);
                    }
                }
            }
        }
        // *** Transition To New Node
        let tran_to_new : GraphVizEdge;
        {
            let tran_gv_options = vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::Vee(GvArrowHeadSide::Both) ) ];
            tran_to_new = GraphVizEdge::new(format!("t{:}", new_state_id),
                                            None,
                                            format!("a{:}", target_int_id),
                                            Some(format!("n{}",target_int_id)),
                                            tran_gv_options);
        }
        // ***
        self.graph.add_edge(tran_to_transfo);
        self.graph.add_edge(tran_to_new);
    }

    fn log_filtered(&mut self,
                    phase : &CanonizationPhase,
                    parent_state_id: u32,
                    new_state_id: u32,
                    elim_kind: &FilterEliminationKind) {
        let (elim_node,elim_edge) = make_graphic_logger_filter(parent_state_id,new_state_id,elim_kind);
        match phase {
            CanonizationPhase::FirstDefactorize => {
                let phase_cluater = self.graph.get_specific_cluster(0).unwrap();
                phase_cluater.add_node(elim_node);
            },
            CanonizationPhase::SecondFactorize => {
                let phase_cluater = self.graph.get_specific_cluster(1).unwrap();
                phase_cluater.add_node(elim_node);
            }
        }
        self.graph.add_edge(elim_edge);
    }

}