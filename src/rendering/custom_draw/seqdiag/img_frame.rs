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
use std::env;
use std::collections::HashMap;
use std::path::Path;

// **********

use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
use imageproc::drawing::{
    Point,
    draw_cross_mut,
    draw_line_segment_mut,
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    draw_filled_circle_mut,
    draw_convex_polygon_mut,
    draw_text_mut
};
use rusttype::{FontCollection, Scale};

// **********

use crate::core::general_context::GeneralContext;

use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::sd_drawing_conf::*;
use crate::rendering::hibou_color_palette::*;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;




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
        let mut square_span_with_margin = lf_name_span + 2.0*MARGIN;

        let font = FontCollection::from_bytes(HIBOU_GRAPHIC_FONT).unwrap().into_font().unwrap();

        let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };

        let label = vec![TextToPrint{text:lf_name,color:Rgb(HC_Lifeline)}];
        let lf_label_centered_pos = lf_coords.x_middle - lf_name_span/2.0;
        draw_colored_text(image,&label,lf_label_centered_pos,lifeline_y_start + VERTICAL_SIZE/2.0);
        // ***
        let mut yshift : usize = 2;
        // ***
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







