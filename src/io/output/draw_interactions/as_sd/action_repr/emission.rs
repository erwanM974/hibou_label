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
use crate::core::language::syntax::action::{EmissionAction, EmissionTargetRef};
use crate::io::output::draw_commons::font::{get_hibou_font, HIBOU_FONT_SCALE};
use crate::io::output::draw_commons::hibou_color_palette::{HC_Message, HCP_Black};
use crate::io::output::draw_commons::sd_drawing_conf::*;
use crate::io::output::draw_interactions::as_sd::action_repr::common::draw_line_for_message_exchange;
use crate::io::output::draw_interactions::as_sd::util::arrow_heads::{draw_arrowhead_leftward, draw_arrowhead_rightward};
use crate::io::output::draw_interactions::as_sd::util::dimensions_tools::get_y_pos_from_yshift;
use crate::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;

// **********

pub fn draw_emission( image : &mut RgbImage,
                    gen_ctx: &GeneralContext,
                    em_act : &EmissionAction,
                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                    yshift : u32) -> [usize;2] {
    // ***
    let mut min_lf_id : usize = em_act.origin_lf_id;
    let mut max_lf_id : usize = em_act.origin_lf_id;
    // ***
    let msg_to_print : Vec<TextToPrint>;
    {
        let msg_label = gen_ctx.get_ms_name(em_act.ms_id).unwrap();
        msg_to_print = vec![TextToPrint::new(msg_label,Rgb(HC_Message))];
    }
    // ***
    let text_y_pos = get_y_pos_from_yshift(yshift) + VERTICAL_SIZE/2.0;
    let arrow_y_pos = get_y_pos_from_yshift(yshift+2);
    let msg_to_print_width = TextToPrint::get_text_width(&msg_to_print,&get_hibou_font(), &HIBOU_FONT_SCALE);
    // ***
    let (img_width,_) = image.dimensions();
    // ***
    match em_act.targets.len() {
        0 => {
            let main_lf_coords = lf_x_widths.get(&em_act.origin_lf_id).unwrap();
            // ***
            let msg_x_left = main_lf_coords.x_middle;
            let msg_x_right= msg_x_left +(main_lf_coords.x_span_inner/2.0);
            draw_arrowhead_rightward(image,msg_x_right,arrow_y_pos,Rgb(HCP_Black));
            draw_line_for_message_exchange(image,&em_act.synchronicity,msg_x_left,msg_x_right,arrow_y_pos);
            let msg_x_middle = (msg_x_left + msg_x_right)/2.0;
            draw_line_of_colored_text(image,
                                      &DrawCoord::CenteredAround(msg_x_middle),
                                      &DrawCoord::CenteredAround(text_y_pos),
                                      &msg_to_print,
                                      &get_hibou_font(),
                                      &HIBOU_FONT_SCALE);
        },
        1 => {
            let origin_lf_id = *(&em_act.origin_lf_id);
            let origin_lf_coords = lf_x_widths.get(&origin_lf_id).unwrap();
            match em_act.targets.get(0).unwrap() {
                EmissionTargetRef::Lifeline(target_lf_id) => {
                    {
                        min_lf_id = min_lf_id.min(*target_lf_id);
                        max_lf_id = max_lf_id.max(*target_lf_id);
                    }
                    let target_lf_coords = lf_x_widths.get(&target_lf_id).unwrap();
                    // ***
                    if origin_lf_id < *target_lf_id {
                        draw_arrowhead_rightward(image,target_lf_coords.x_middle, arrow_y_pos,Rgb(HCP_Black));
                    } else {
                        draw_arrowhead_leftward(image,target_lf_coords.x_middle, arrow_y_pos,Rgb(HCP_Black));
                    }
                    draw_line_for_message_exchange(image,&em_act.synchronicity,target_lf_coords.x_middle,origin_lf_coords.x_middle,arrow_y_pos);
                    // ***
                    let mut anchor_lf_id : usize = *target_lf_id;
                    if target_lf_id == &origin_lf_id {
                        panic!("cannot draw emission then reception on the same lifeline");
                    } else if target_lf_id < &origin_lf_id {
                        let mut lf_id_shift : usize = 1;
                        while !lf_x_widths.contains_key(&(origin_lf_id - lf_id_shift)) {
                            lf_id_shift = lf_id_shift + 1 ;
                        }
                        anchor_lf_id = origin_lf_id - lf_id_shift;
                    } else if target_lf_id > &origin_lf_id {
                        let mut lf_id_shift : usize = 1;
                        while !lf_x_widths.contains_key(&(origin_lf_id + lf_id_shift)) {
                            lf_id_shift = lf_id_shift + 1 ;
                        }
                        anchor_lf_id = origin_lf_id + lf_id_shift;
                    }
                    let anchor_lf_coords = lf_x_widths.get(&anchor_lf_id).unwrap();
                    let msg_x_middle = (origin_lf_coords.x_middle + anchor_lf_coords.x_middle)/2.0;
                    draw_line_of_colored_text(image,
                                              &DrawCoord::CenteredAround(msg_x_middle),
                                              &DrawCoord::CenteredAround(text_y_pos),
                                              &msg_to_print,
                                              &get_hibou_font(),
                                              &HIBOU_FONT_SCALE);
                },
                EmissionTargetRef::Gate(target_gt_id) => {
                    draw_filled_rect_mut(image,
                                         Rect::at((img_width as f32 - GATE_SIZE) as i32,
                                                  (arrow_y_pos - GATE_SIZE/2.0) as i32).of_size(GATE_SIZE as u32, GATE_SIZE as u32),
                                         Rgb(HCP_Black));
                    // ***
                    let msg_x_left = origin_lf_coords.x_middle;
                    let msg_x_right= img_width as f32;
                    draw_arrowhead_rightward(image,msg_x_right,arrow_y_pos,Rgb(HCP_Black));
                    draw_line_for_message_exchange(image,&em_act.synchronicity,msg_x_left,msg_x_right,arrow_y_pos);
                    let msg_x_middle = (msg_x_left + msg_x_right)/2.0;
                    draw_line_of_colored_text(image,
                                              &DrawCoord::CenteredAround(msg_x_middle),
                                              &DrawCoord::CenteredAround(text_y_pos),
                                              &msg_to_print,
                                              &get_hibou_font(),
                                              &HIBOU_FONT_SCALE);
                    // ***
                }
            }
        },
        _ => {
            {
                let main_lf_coords = lf_x_widths.get(&em_act.origin_lf_id).unwrap();
                // ***
                let msg_x_left = main_lf_coords.x_middle;
                let msg_x_right= msg_x_left +(main_lf_coords.x_span_inner/2.0);
                draw_arrowhead_rightward(image,msg_x_right, arrow_y_pos,Rgb(HCP_Black));
                //draw_double_half_ellipsis_rightward(image,msg_x_right, arrow_y_pos,Rgb(HCP_Black));
                draw_line_for_message_exchange(image,&em_act.synchronicity,msg_x_left,msg_x_right,arrow_y_pos);
                let msg_x_middle = (msg_x_left + msg_x_right)/2.0;
                draw_line_of_colored_text(image,
                                          &DrawCoord::CenteredAround(msg_x_middle),
                                          &DrawCoord::CenteredAround(text_y_pos),
                                          &msg_to_print,
                                          &get_hibou_font(),
                                          &HIBOU_FONT_SCALE);
                // ***
            }
            for target_ref in &em_act.targets {
                match target_ref {
                    EmissionTargetRef::Lifeline(tar_lf_id) => {
                        {
                            min_lf_id = min_lf_id.min(*tar_lf_id);
                            max_lf_id = max_lf_id.max(*tar_lf_id);
                        }
                        let tar_lf_coords = lf_x_widths.get(tar_lf_id).unwrap();
                        // ***
                        let tar_x_right = tar_lf_coords.x_middle;
                        let tar_x_left= tar_x_right - (tar_lf_coords.x_span_inner/2.0);

                        //draw_filled_circle_mut(image, (tar_x_left as i32, arrow_y_pos as i32), 3, Rgb(HCP_Black));
                        draw_arrowhead_rightward(image, tar_x_right, arrow_y_pos,Rgb(HCP_Black));
                        draw_line_for_message_exchange(image,&em_act.synchronicity,tar_x_left,tar_x_right,arrow_y_pos);
                    },
                    EmissionTargetRef::Gate(tar_gt_id) => {
                        draw_filled_rect_mut(image,
                                             Rect::at((img_width as f32 - GATE_SIZE) as i32,
                                                      (arrow_y_pos - GATE_SIZE/2.0) as i32).of_size(GATE_SIZE as u32, GATE_SIZE as u32),
                                             Rgb(HCP_Black));
                        let tar_x_right = img_width as f32;
                        let tar_x_left = tar_x_right - ((HORIZONTAL_SIZE - 2.0*MARGIN)/3.0);

                        //draw_filled_circle_mut(image, (tar_x_left as i32, arrow_y_pos as i32), 3, Rgb(HCP_Black));
                        //draw_double_half_ellipsis_rightward(image, tar_x_left, arrow_y_pos,Rgb(HCP_Black));
                        draw_arrowhead_rightward(image, tar_x_right, arrow_y_pos,Rgb(HCP_Black));
                        draw_line_for_message_exchange(image,&em_act.synchronicity,tar_x_left,tar_x_right,arrow_y_pos);
                    }
                }
            }
        }
    }
    // ***
    return [min_lf_id,max_lf_id];
}


