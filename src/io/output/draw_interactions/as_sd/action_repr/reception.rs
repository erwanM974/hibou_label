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



use std::collections::HashMap;

use image::{Rgb, RgbImage};
use image_colored_text::draw::single_line::{draw_line_of_colored_text, DrawCoord};
use image_colored_text::ttp::TextToPrint;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::action::ReceptionAction;
use crate::io::output::draw_commons::font::{get_hibou_font, HIBOU_FONT_SCALE};
use crate::io::output::draw_commons::hibou_color_palette::{HC_Message, HCP_Black};
use crate::io::output::draw_commons::sd_drawing_conf::*;
use crate::io::output::draw_interactions::as_sd::action_repr::common::draw_line_for_message_exchange;
use crate::io::output::draw_interactions::as_sd::util::arrow_heads::draw_arrowhead_rightward;
use crate::io::output::draw_interactions::as_sd::util::dimensions_tools::get_y_pos_from_yshift;
use crate::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;

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
        msg_to_print = vec![TextToPrint::new(msg_label,Rgb(HC_Message))];
    }
    // ***
    let text_y_pos = get_y_pos_from_yshift(yshift) + VERTICAL_SIZE/2.0;
    let arrow_y_pos = get_y_pos_from_yshift(yshift+2);
    let msg_to_print_width = TextToPrint::get_text_width(&msg_to_print,
                                                         &get_hibou_font(),
                                                         &HIBOU_FONT_SCALE);
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
                draw_line_of_colored_text(image,
                                          &DrawCoord::CenteredAround(msg_x_middle),
                                          &DrawCoord::CenteredAround(text_y_pos),
                                          &msg_to_print,
                                          &get_hibou_font(),
                                          &HIBOU_FONT_SCALE);
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
                draw_line_of_colored_text(image,
                                          &DrawCoord::CenteredAround(msg_x_middle),
                                          &DrawCoord::CenteredAround(text_y_pos),
                                          &msg_to_print,
                                          &get_hibou_font(),
                                          &HIBOU_FONT_SCALE);
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
                draw_line_of_colored_text(image,
                                          &DrawCoord::CenteredAround(msg_x_middle),
                                          &DrawCoord::CenteredAround(text_y_pos),
                                          &msg_to_print,
                                          &get_hibou_font(),
                                          &HIBOU_FONT_SCALE);
            }
        }
    }
    // ***
    return [min_lf_id,max_lf_id];
}


