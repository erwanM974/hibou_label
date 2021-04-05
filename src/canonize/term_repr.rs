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

use crate::rendering::graphviz::graph::*;
use crate::rendering::graphviz::node_style::*;
use crate::rendering::graphviz::edge_style::*;
use crate::rendering::graphviz::common::*;

use crate::core::syntax::interaction::{Interaction,ScheduleOperatorKind};
use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;
use crate::core::syntax::position::Position;

use crate::core::semantics::execute::deploy_receptions;


pub fn interaction_repr(interaction : &Interaction, gen_ctx : &GeneralContext, name : &String, as_subgraph : bool) -> String {
    let mut repr : String = String::new();
    if as_subgraph {
        repr.push_str( &format!("subgraph cluster_{} {{\n", name) );
        let mut node_gv_options : GraphvizNodeStyle = Vec::new();
        node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
        node_gv_options.push( GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Filled]) );
        node_gv_options.push( GraphvizNodeStyleItem::Label("".to_string()) );
        let gv_node = GraphVizNode{id : format!("{}_anchor",name), style : node_gv_options};
        repr.push_str( &gv_node.to_dot_string() );
        repr.push_str("\n");
    } else {
        repr.push_str( &format!("digraph {} {{\n", name) );
    }
    interaction_repr_rec(&mut repr, interaction, gen_ctx, name, Position::Epsilon);
    repr.push_str( "}\n" );
    return repr;
}

fn action_repr(to_write : &mut String,
               act : &ObservableAction,
               gen_ctx : &GeneralContext,
               interaction_name : &String,
               current_pos : Position) {
    let ms_name = gen_ctx.get_ms_name(act.ms_id).unwrap();
    let lf_name = gen_ctx.get_lf_name(act.lf_id).unwrap();
    match &act.act_kind {
        &ObservableActionKind::Reception => {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( format!("{}?{}", &lf_name, &ms_name) ) );
            let gv_node = GraphVizNode{id : format!("{}p{}",interaction_name,position_to_id(&current_pos)), style : node_gv_options};
            to_write.push_str( &gv_node.to_dot_string() );
            to_write.push_str("\n");
        },
        &ObservableActionKind::Emission(ref targets) => {
            // ***
            let node_name = format!("{}p{}",interaction_name,position_to_id(&current_pos));
            // ***
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( format!("{}!{}", &lf_name, &ms_name) ) );
            // ***
            let tars_len = targets.len();
            if tars_len == 0 {
                let gv_node = GraphVizNode{id : node_name, style : node_gv_options};
                to_write.push_str( &gv_node.to_dot_string() );
                to_write.push_str("\n");
            } else {
                let deployed_recs = deploy_receptions(act.ms_id, &mut targets.clone());
                // the parent strict node
                {
                    let mut strict_node_gv_options : GraphvizNodeStyle = Vec::new();
                    strict_node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
                    strict_node_gv_options.push( GraphvizNodeStyleItem::Label( "strict".to_string() ) );
                    let strict_gv_node = GraphVizNode{id : node_name.clone(), style : strict_node_gv_options};
                    to_write.push_str( &strict_gv_node.to_dot_string() );
                    to_write.push_str("\n");
                }
                // then the emission node
                {
                    let new_position = Position::Left(Box::new(current_pos.clone()));
                    let child_node_name = format!("{}p{}",interaction_name,position_to_id(&new_position));
                    let gv_node = GraphVizNode{id : child_node_name.clone(), style : node_gv_options};
                    to_write.push_str( &gv_node.to_dot_string() );
                    to_write.push_str("\n");
                    let gv_edge = GraphVizEdge{origin_id : node_name.clone(),
                        target_id : child_node_name,
                        style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
                    to_write.push_str(&gv_edge.to_dot_string());
                    to_write.push_str("\n");
                }
                // then the sub-interaction for the reception actions
                {
                    let new_position = Position::Right(Box::new(current_pos.clone()));
                    let child_node_name = interaction_repr_rec(to_write,&deployed_recs,gen_ctx,interaction_name,new_position);
                    let gv_edge = GraphVizEdge{origin_id : node_name,
                        target_id : child_node_name,
                        style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
                    to_write.push_str(&gv_edge.to_dot_string());
                    to_write.push_str("\n");
                }
            }
        }
    }
}

fn interaction_repr_rec(to_write : &mut String,
                        interaction : &Interaction,
                        gen_ctx : &GeneralContext,
                        interaction_name : &String,
                        current_pos : Position) -> String {
    let node_name = format!("{}p{}",interaction_name,position_to_id(&current_pos));
    match interaction {
        &Interaction::Empty => {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( "o".to_string() ) );
            let gv_node = GraphVizNode{id : node_name.clone(), style : node_gv_options};
            to_write.push_str( &gv_node.to_dot_string() );
            to_write.push_str("\n");
        },
        &Interaction::Action(ref act) => {
            action_repr(to_write,act,gen_ctx,interaction_name,current_pos);
        },
        &Interaction::Strict(ref i1, ref i2) => {
            repr_binary_operator(to_write, i1, i2, "strict", gen_ctx, interaction_name, current_pos);
        },
        &Interaction::Seq(ref i1, ref i2) => {
            repr_binary_operator(to_write, i1, i2, "seq", gen_ctx, interaction_name, current_pos);
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let mut op_label = "coreg(".to_string();
            let mut rem  = cr.len();
            for lf_id in cr {
                let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
                op_label.push_str(&lf_name);
                rem = rem - 1;
                if rem > 0 {
                    op_label.push_str(",");
                }
            }
            op_label.push_str(")");
            repr_binary_operator(to_write, i1, i2, &op_label, gen_ctx, interaction_name, current_pos);
        },
        &Interaction::Par(ref i1, ref i2) => {
            repr_binary_operator(to_write, i1, i2, "par", gen_ctx, interaction_name, current_pos);
        },
        &Interaction::Alt(ref i1, ref i2) => {
            repr_binary_operator(to_write, i1, i2, "alt", gen_ctx, interaction_name, current_pos);
        },
        &Interaction::Loop(ref lp_kind, ref i1) => {
            // the parent loop node
            {
                let mut strict_node_gv_options : GraphvizNodeStyle = Vec::new();
                strict_node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
                match lp_kind {
                    &ScheduleOperatorKind::Strict => {
                        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( "loop_strict".to_string() ) );
                    },
                    &ScheduleOperatorKind::Seq => {
                        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( "loop_seq".to_string() ) );
                    },
                    &ScheduleOperatorKind::Par => {
                        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( "loop_par".to_string() ) );
                    }
                }
                let strict_gv_node = GraphVizNode{id : node_name.clone(), style : strict_node_gv_options};
                to_write.push_str( &strict_gv_node.to_dot_string() );
                to_write.push_str("\n");
            }
            // then the left sub-interaction
            {
                let left_position = Position::Left(Box::new(current_pos.clone()));
                let child_node_name = interaction_repr_rec(to_write,i1,gen_ctx,interaction_name,left_position);
                let gv_edge = GraphVizEdge{origin_id : node_name.clone(),
                    target_id : child_node_name,
                    style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
                to_write.push_str(&gv_edge.to_dot_string());
                to_write.push_str("\n");
            }
        }
    }
    return node_name;
}

fn repr_binary_operator(to_write : &mut String,
                        i1 : &Interaction,
                        i2 : &Interaction,
                        operator_label : &str,
                        gen_ctx : &GeneralContext,
                        interaction_name : &String,
                        current_pos : Position) {
    let node_name = format!("{}p{}",interaction_name,position_to_id(&current_pos));
    // the parent strict node
    {
        let mut strict_node_gv_options : GraphvizNodeStyle = Vec::new();
        strict_node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( operator_label.to_string() ) );
        let strict_gv_node = GraphVizNode{id : node_name.clone(), style : strict_node_gv_options};
        to_write.push_str( &strict_gv_node.to_dot_string() );
        to_write.push_str("\n");
    }
    // then the left sub-interaction
    {
        let left_position = Position::Left(Box::new(current_pos.clone()));
        let child_node_name = interaction_repr_rec(to_write,i1,gen_ctx,interaction_name,left_position.clone());
        let gv_edge = GraphVizEdge{origin_id : node_name.clone(),
            target_id : child_node_name,
            style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
        to_write.push_str(&gv_edge.to_dot_string());
        to_write.push_str("\n");
    }
    // then the right sub-interaction
    {
        let right_position = Position::Right(Box::new(current_pos.clone()));
        let child_node_name = interaction_repr_rec(to_write,i2,gen_ctx,interaction_name,right_position.clone());
        let gv_edge = GraphVizEdge{origin_id : node_name,
            target_id : child_node_name,
            style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
        to_write.push_str(&gv_edge.to_dot_string());
        to_write.push_str("\n");
    }
}



fn position_to_id(position : &Position) -> String {
    match position {
        Position::Left(ref in_self) => {
            let mut my_string = "1".to_string();
            let sub_pos = position_to_id( &(*in_self) );
            if sub_pos != "0".to_string() {
                my_string.push_str( &sub_pos );
            }
            return my_string;
        },
        Position::Right(ref in_self) => {
            let mut my_string = "2".to_string();
            let sub_pos = position_to_id( &(*in_self) );
            if sub_pos != "0".to_string() {
                my_string.push_str( &sub_pos );
            }
            return my_string;
        },
        Position::Epsilon => {
            return "0".to_string();
        }
    }
}

