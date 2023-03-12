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

use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::position::position::Position;
use crate::io::output::draw_interactions::as_term::action_repr::emission::{emission_as_gv_label};
use crate::io::output::draw_interactions::as_term::action_repr::reception::{reception_as_gv_label};
use crate::io::output::draw_interactions::as_term::action_repr::trace_action::trace_actions_as_gv_label;
use crate::io::output::graphviz::colors::DotTranslatable;
use crate::io::output::graphviz::edge::edge::GraphVizEdge;
use crate::io::output::graphviz::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadStyle};
use crate::io::output::graphviz::node::node::GraphVizNode;
use crate::io::output::graphviz::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use crate::io::textual_convention::{SYNTAX_ALT, SYNTAX_COREG, SYNTAX_LOOP_H, SYNTAX_LOOP_P, SYNTAX_LOOP_S, SYNTAX_LOOP_W, SYNTAX_PAR, SYNTAX_SEQ, SYNTAX_STRICT, SYNTAX_SYNC};


pub fn interaction_gv_repr(gen_ctx : &GeneralContext,
                        interaction : &Interaction) -> String {
    let mut repr : String = String::new();
    repr.push_str( "digraph G {\n" );
    interaction_gv_repr_rec(&mut repr, gen_ctx, interaction,Position::Epsilon(None));
    repr.push_str( "}\n" );
    return repr;
}


fn interaction_gv_repr_rec(to_write : &mut String,
                        gen_ctx : &GeneralContext,
                        interaction : &Interaction,
                        current_pos : Position) -> String {
    let node_name = format!("p{}",current_pos.to_string());
    match interaction {
        &Interaction::Empty => {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( "o".to_string() ) );
            let gv_node = GraphVizNode{id : node_name.clone(), style : node_gv_options};
            to_write.push_str( &gv_node.to_dot_string() );
            to_write.push_str("\n");
        },
        &Interaction::Emission(ref em_act) => {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( emission_as_gv_label(gen_ctx,em_act) ) );
            let gv_node = GraphVizNode{id : node_name.clone(), style : node_gv_options};
            to_write.push_str( &gv_node.to_dot_string() );
            to_write.push_str("\n");
        },
        &Interaction::Reception(ref rc_act) => {
            let mut node_gv_options : GraphvizNodeStyle = Vec::new();
            node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
            node_gv_options.push( GraphvizNodeStyleItem::Label( reception_as_gv_label(gen_ctx,rc_act) ) );
            let gv_node = GraphVizNode{id : node_name.clone(), style : node_gv_options};
            to_write.push_str( &gv_node.to_dot_string() );
            to_write.push_str("\n");
        },
        &Interaction::Strict(ref i1, ref i2) => {
            repr_binary_operator(to_write, gen_ctx,i1, i2, SYNTAX_STRICT, current_pos);
        },
        &Interaction::Seq(ref i1, ref i2) => {
            repr_binary_operator(to_write, gen_ctx, i1, i2, SYNTAX_SEQ, current_pos);
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            let mut op_label = format!("{}(", SYNTAX_COREG);
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
            repr_binary_operator(to_write, gen_ctx, i1, i2, &op_label, current_pos);
        },
        &Interaction::Sync(ref sync_acts, ref i1, ref i2) => {
            let acts_as_str = trace_actions_as_gv_label(gen_ctx,sync_acts.iter());
            let op_label = format!("{}{}", SYNTAX_SYNC,acts_as_str);
            repr_binary_operator(to_write, gen_ctx, i1, i2, &op_label, current_pos);
        },
        &Interaction::Par(ref i1, ref i2) => {
            repr_binary_operator(to_write, gen_ctx, i1, i2, SYNTAX_PAR, current_pos);
        },
        &Interaction::Alt(ref i1, ref i2) => {
            repr_binary_operator(to_write, gen_ctx, i1, i2, SYNTAX_ALT, current_pos);
        },
        &Interaction::Loop(ref lp_kind, ref i1) => {
            // the parent loop node
            {
                let mut strict_node_gv_options : GraphvizNodeStyle = Vec::new();
                strict_node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
                match lp_kind {
                    &LoopKind::SStrictSeq => {
                        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( SYNTAX_LOOP_S.to_string() ) );
                    },
                    &LoopKind::HHeadFirstWS => {
                        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( SYNTAX_LOOP_H.to_string() ) );
                    },
                    &LoopKind::WWeakSeq => {
                        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( SYNTAX_LOOP_W.to_string() ) );
                    },
                    &LoopKind::PInterleaving => {
                        strict_node_gv_options.push( GraphvizNodeStyleItem::Label( SYNTAX_LOOP_P.to_string() ) );
                    }
                }
                let strict_gv_node = GraphVizNode{id : node_name.clone(), style : strict_node_gv_options};
                to_write.push_str( &strict_gv_node.to_dot_string() );
                to_write.push_str("\n");
            }
            // then the left sub-interaction
            {
                let left_position = Position::Left(Box::new(current_pos.clone()));
                let child_node_name = interaction_gv_repr_rec(to_write,gen_ctx,i1,left_position);
                let gv_edge = GraphVizEdge{origin_id : node_name.clone(),
                    target_id : child_node_name,
                    style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
                to_write.push_str(&gv_edge.to_dot_string());
                to_write.push_str("\n");
            }
        },
        &Interaction::And(ref i1, ref i2) => {
            repr_binary_operator(to_write, gen_ctx, i1, i2, "and", current_pos);
        }
    }
    return node_name;
}

fn repr_binary_operator(to_write : &mut String,
                        gen_ctx : &GeneralContext,
                        i1 : &Interaction,
                        i2 : &Interaction,
                        operator_label : &str,
                        current_pos : Position) {
    let node_name = format!("p{}",current_pos.to_string());
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
        let child_node_name = interaction_gv_repr_rec(to_write,gen_ctx,i1,left_position.clone());
        let gv_edge = GraphVizEdge{origin_id : node_name.clone(),
            target_id : child_node_name,
            style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
        to_write.push_str(&gv_edge.to_dot_string());
        to_write.push_str("\n");
    }
    // then the right sub-interaction
    {
        let right_position = Position::Right(Box::new(current_pos.clone()));
        let child_node_name = interaction_gv_repr_rec(to_write,gen_ctx,i2,right_position.clone());
        let gv_edge = GraphVizEdge{origin_id : node_name,
            target_id : child_node_name,
            style :  vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]};
        to_write.push_str(&gv_edge.to_dot_string());
        to_write.push_str("\n");
    }
}


