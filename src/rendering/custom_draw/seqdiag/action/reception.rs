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



use std::cmp;
use std::env;
use std::collections::{HashMap,HashSet};

// **********

use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
use imageproc::drawing::{
    draw_cross_mut,
    draw_line_segment_mut,
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    draw_filled_circle_mut,
    draw_text_mut
};

// **********

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::rendering::custom_draw::seqdiag::action::common::draw_line_for_message_exchange;


use crate::rendering::sd_drawing_conf::*;
use crate::rendering::hibou_color_palette::*;

use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::textual::colored::colored_text::*;

use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;
use crate::rendering::custom_draw::utils::arrow_heads::*;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;

// **********

pub fn draw_reception( image : &mut RgbImage,
                    gen_ctx: &GeneralContext,
                    rc_act : &ReceptionAction,
                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                    yshift : u32) -> [usize;2] {
    // ***
    let mut min_lf_id : usize = gen_ctx.get_lf_num();
    let mut max_lf_id : usize = 0;
    // ***
    let msg_to_print : Vec<TextToPrint>;
    {
        let msg_label = gen_ctx.get_ms_name(rc_act.ms_id).unwrap();
        msg_to_print = vec![TextToPrint{text:msg_label,color:Rgb(HC_Message)}];
    }
    // ***
    let text_y_pos = get_y_pos_from_yshift(yshift);
    let arrow_y_pos = get_y_pos_from_yshift(yshift+2);
    let msg_to_print_width : f32 = (TextToPrint::char_count(&msg_to_print) as f32)*FONT_WIDTH/2.0;
    // ***
    let (img_width,_) = image.dimensions();
    // ***
    match rc_act.origin_gt_id {
        None => {
            for rcv_lf_id in &rc_act.recipients {
                {
                    min_lf_id = min_lf_id.min(*rcv_lf_id);
                    max_lf_id = max_lf_id.max(*rcv_lf_id);
                }
                let tar_lf_coords = lf_x_widths.get(rcv_lf_id).unwrap();
                // ***
                let tar_x_right = tar_lf_coords.x_middle;
                let tar_x_left= tar_x_right - (tar_lf_coords.x_span_inner/2.0);
                draw_arrowhead_rightward(image, tar_x_right, arrow_y_pos,Rgb(HCP_Black));
                draw_line_for_message_exchange(image,&rc_act.synchronicity,tar_x_left,tar_x_right,arrow_y_pos);
                let msg_x_middle = (tar_x_left + tar_x_right)/2.0;
                draw_colored_text(image,&msg_to_print,msg_x_middle - msg_to_print_width/2.0,text_y_pos);
            }
        },
        Some( orig_gt_id ) => {
            {
                let orig_x_left = 0.0;
                let orig_x_right = orig_x_left + HORIZONTAL_SIZE/3.5;
                draw_filled_rect_mut(image,
                                     Rect::at(orig_x_left as i32,
                                              (arrow_y_pos - GATE_SIZE/2.0) as i32).of_size(GATE_SIZE as u32, GATE_SIZE as u32),
                                     Rgb(HCP_Black));
                draw_line_for_message_exchange(image,&rc_act.synchronicity,orig_x_left,orig_x_right,arrow_y_pos);
                draw_arrowhead_rightward(image, orig_x_right, arrow_y_pos,Rgb(HCP_Black));
                let msg_x_middle = (orig_x_left + orig_x_right)/2.0;
                draw_colored_text(image,&msg_to_print,msg_x_middle - msg_to_print_width/2.0,text_y_pos);
            }
            for rcv_lf_id in &rc_act.recipients {
                {
                    min_lf_id = min_lf_id.min(*rcv_lf_id);
                    max_lf_id = max_lf_id.max(*rcv_lf_id);
                }
                let tar_lf_coords = lf_x_widths.get(rcv_lf_id).unwrap();
                // ***
                let tar_x_right = tar_lf_coords.x_middle;
                let tar_x_left= tar_x_right - (tar_lf_coords.x_span_inner/2.0);
                draw_arrowhead_rightward(image, tar_x_right, arrow_y_pos,Rgb(HCP_Black));
                draw_line_for_message_exchange(image,&rc_act.synchronicity,tar_x_left,tar_x_right,arrow_y_pos);
                let msg_x_middle = (tar_x_left + tar_x_right)/2.0;
                draw_colored_text(image,&msg_to_print,msg_x_middle - msg_to_print_width/2.0,text_y_pos);
            }
        }
    }
    // ***
    return [min_lf_id,max_lf_id];
}


