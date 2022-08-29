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

use crate::core::syntax::interaction::{Interaction,LoopKind};
use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;
use crate::core::syntax::position::Position;

use crate::rendering::textual::convention::*;

use crate::rendering::custom_draw::term::util::position_to_id;


pub fn emission_repr(to_write : &mut String,
               em_act : &EmissionAction,
               gen_ctx : &GeneralContext,
               interaction_name : &String,
               current_pos : Position) {
    let ms_name = gen_ctx.get_ms_name(em_act.ms_id).unwrap();
    let lf_name = gen_ctx.get_lf_name(em_act.origin_lf_id).unwrap();
    // ***
    let mut node_gv_options : GraphvizNodeStyle = Vec::new();
    node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
    // ***
    let node_name = format!("{}p{}",interaction_name,position_to_id(&current_pos));
    // ***
    let mut targ_names : Vec<String> = Vec::new();
    for targ_ref in &em_act.targets {
        match targ_ref {
            EmissionTargetRef::Lifeline(tar_lf_id) => {
                targ_names.push( gen_ctx.get_lf_name(*tar_lf_id).unwrap() );
            },
            EmissionTargetRef::Gate(tar_gt_id) => {
                targ_names.push( gen_ctx.get_gt_name(*tar_gt_id).unwrap() );
            }
        }
    }
    // ***
    node_gv_options.push( GraphvizNodeStyleItem::Label( format!("{}-{}>({})", &lf_name, &ms_name, &targ_names.join(",")) ) );
    // ***
    let gv_node = GraphVizNode{id : node_name, style : node_gv_options};
    to_write.push_str( &gv_node.to_dot_string() );
    to_write.push_str("\n");
}


pub fn reception_repr(to_write : &mut String,
               rc_act : &ReceptionAction,
               gen_ctx : &GeneralContext,
               interaction_name : &String,
               current_pos : Position) {
    let ms_name = gen_ctx.get_ms_name(rc_act.ms_id).unwrap();
    // ***
    let mut node_gv_options : GraphvizNodeStyle = Vec::new();
    node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::PlainText) );
    // ***
    let node_name = format!("{}p{}",interaction_name,position_to_id(&current_pos));
    // ***
    let mut targ_names : Vec<String> = Vec::new();
    for rcp_lf_id in &rc_act.recipients {
        targ_names.push( gen_ctx.get_lf_name(*rcp_lf_id).unwrap() );
    }
    // ***
    match rc_act.origin_gt_id {
        None => {
            node_gv_options.push( GraphvizNodeStyleItem::Label( format!("|-{}>({})", &ms_name,  &targ_names.join(",")) ) );},
        Some(orig_gt_id) => {
            let gt_name = gen_ctx.get_gt_name(orig_gt_id).unwrap();
            node_gv_options.push( GraphvizNodeStyleItem::Label( format!("{}-{}>({})", &gt_name, &ms_name,  &targ_names.join(",")) ) );
        }
    }
    // ***
    let gv_node = GraphVizNode{id : node_name, style : node_gv_options};
    to_write.push_str( &gv_node.to_dot_string() );
    to_write.push_str("\n");
}