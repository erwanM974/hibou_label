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
use imageproc::drawing::{
    draw_filled_rect_mut,
    draw_hollow_rect_mut,
    draw_line_segment_mut
};
use imageproc::rect::Rect;


use crate::core::general_context::GeneralContext;
use crate::io::output::draw_commons::font::{get_hibou_font, HIBOU_FONT_SCALE};
use crate::io::output::draw_commons::hibou_color_palette::*;
use crate::io::output::draw_commons::sd_drawing_conf::*;
use crate::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;

// **********

pub fn draw_frame(image : &mut RgbImage, img_width : &f32, img_height : &f32, max_y_shift : usize) {
    draw_filled_rect_mut(image, Rect::at(0,0).of_size(*img_width as u32,*img_height as u32), Rgb(HCP_White));
}

pub fn draw_lifelines(image : &mut RgbImage,
                      lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                      inner_height : f32,
                      gen_ctx:&GeneralContext) {
    // Draw Lifelines
    let lifeline_y_start :f32 = MARGIN;
    let lifeline_y_end :f32 = MARGIN+inner_height;
    for (lf_id,lf_coords) in lf_x_widths.iter() {
        // ***
        let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
        let lf_name_span = FONT_WIDTH*(lf_name.chars().count() as f32)/2.0;
        // ***
        let label = vec![TextToPrint::new(lf_name,Rgb(HC_Lifeline))];
        draw_line_of_colored_text(image,
                                  &DrawCoord::CenteredAround(lf_coords.x_middle),
                                  &DrawCoord::CenteredAround(lifeline_y_start + VERTICAL_SIZE),
                                  &label,
                                  &get_hibou_font(),
                                  &HIBOU_FONT_SCALE);
        // ***
        let yshift : usize = 2;
        // ***
        let square_span_with_margin = lf_name_span + 2.0*MARGIN;
        let actor_x_start : f32 = lf_coords.x_middle - (square_span_with_margin/2.0);
        draw_hollow_rect_mut(image,
                             Rect::at(actor_x_start as i32, lifeline_y_start as i32).of_size(square_span_with_margin as u32, ((yshift as f32)*VERTICAL_SIZE) as u32),
                             Rgb(HC_Grammar_Symbol));
        // ***
        draw_line_segment_mut(image,
                              (lf_coords.x_middle, lifeline_y_start + (yshift as f32)*VERTICAL_SIZE),
                              (lf_coords.x_middle, lifeline_y_end),
                              Rgb(HC_Grammar_Symbol));
    }
}







