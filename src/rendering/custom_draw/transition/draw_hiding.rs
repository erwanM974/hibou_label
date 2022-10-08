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

use std::collections::HashSet;
use std::cmp;
use std::path::Path;
use std::collections::HashMap;

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
use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;

use crate::rendering::sd_drawing_conf::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::textual::colored::short_action::diagram_repr_trace_action;

use crate::rendering::hibou_color_palette::*;
use crate::rendering::textual::monochrome::position::position_to_text;
use crate::rendering::custom_draw::utils::colored_text::{draw_colored_text, DrawnColoredTextAlignment, new_image_with_colored_text};

use crate::core::trace::{TraceAction,TraceActionKind};
use crate::process::ana_proc::interface::step::SimulationStepKind;

// ***





pub fn draw_hiding(path_str : &String,
                   lfs_to_hide : &HashSet<usize>,
                   gen_ctx : &GeneralContext) {
    let path = Path::new( path_str );
    // ***
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    {
        let mut ttp = Vec::new();
        // ***
        ttp.push( TextToPrint{text:"hide [".to_string(),color:Rgb(HCP_Black)} );
        let mut are_left = lfs_to_hide.len();
        for lf_id in lfs_to_hide {
            let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
            ttp.push( TextToPrint{text:lf_name,color:Rgb(HC_Lifeline)} );
            are_left = are_left -1;
            if are_left > 0 {
                ttp.push( TextToPrint{text:",".to_string(),color:Rgb(HCP_Black)} );
            }
        }
        ttp.push( TextToPrint{text:"]".to_string(),color:Rgb(HCP_Black)} );
        text_lines.push( ttp );
    }
    // ***
    let line_lens : Vec<usize> = text_lines.iter().map(|x| TextToPrint::char_count(x) ).collect();
    let max_x_shift = *line_lens.iter().max().unwrap();
    // ***
    let img_width : f32 = 2.0*MARGIN + (max_x_shift as f32)*FONT_WIDTH/2.0;
    let img_height : f32 = 2.0*MARGIN + (text_lines.len() as f32)*FONT_HEIGHT/2.0;
    // ***
    new_image_with_colored_text(path,&DrawnColoredTextAlignment::Center,img_width,img_height,text_lines)
}


