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

use image::Rgb;
use image::RgbImage;

// **********

use crate::core::syntax::interaction::*;
use crate::core::general_context::GeneralContext;

use crate::process::ana_proc::multitrace::{AnalysableMultiTraceCanal,AnalysableMultiTrace};

use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::sd_drawing_conf::*;
use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::custom_draw::seqdiag::img_frame::*;
use crate::rendering::custom_draw::seqdiag::img_content::*;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;
use crate::rendering::hibou_color_palette::*;

use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;
use crate::rendering::custom_draw::seqdiag::ext_multi_trace::extract_texts_on_multi_trace;
// **********

pub fn draw_interaction(path_str : &String,
                        interaction : &Interaction,
                        gen_ctx : &GeneralContext,
                        remaining_multi_trace : &Option<&AnalysableMultiTrace>) {
    let path = Path::new( path_str );
    // ***
    let mut lf_x_widths : HashMap<usize,DrawingLifelineCoords> = HashMap::new();
    let mut current_x : f32 = MARGIN;
    for lf_id in 0..gen_ctx.get_lf_num() {
        if interaction.involves_any_of(&hashset!{lf_id}) {
            let lf_char_width = gen_ctx.get_lf_name(lf_id).unwrap().len();
            // ***
            let span_inner = (HORIZONTAL_SIZE - 2.0*MARGIN).max( 2.0*MARGIN + (lf_char_width as f32)*FONT_WIDTH/2.0 );
            let span_outer = span_inner + 2.0*MARGIN;
            let middle = current_x + (span_outer/2.0) + THICKNESS;
            lf_x_widths.insert(lf_id,DrawingLifelineCoords{x_start:current_x,
                x_span_inner:span_inner,
                x_span_outer:span_outer,
                x_middle:middle});
            current_x = current_x + span_outer + MARGIN;
        }
    }
    // ***
    let max_y_shift = get_interaction_max_yshift(interaction);
    let mut inner_height : f32 = (max_y_shift as f32)*VERTICAL_SIZE;

    let img_width : f32;
    let multi_trace_txttoprint : Option<(Vec<Vec<TextToPrint>>,f32)>;
    match remaining_multi_trace {
        None => {
            img_width = current_x;
            multi_trace_txttoprint = None;
        }
        Some( multi_trace ) => {
            let mt_ttp = extract_texts_on_multi_trace( gen_ctx, multi_trace);
            let mut max_char_count = 0;
            for ttp in &mt_ttp {
                max_char_count = max_char_count.max(TextToPrint::char_count(ttp) );
            }
            let mt_print_width = (max_char_count as f32)*FONT_WIDTH/2.0;
            img_width = current_x + mt_print_width;
            // ***
            inner_height = inner_height.max( ((2*mt_ttp.len()) as f32)*VERTICAL_SIZE );
            // ***
            multi_trace_txttoprint = Some( (mt_ttp,mt_print_width) );
        }
    }
    let img_height : f32 = inner_height + 2.0*MARGIN;

    // Draw Frame
    let mut image = RgbImage::new( img_width as u32, img_height as u32);
    draw_frame(&mut image, &img_width, &img_height, max_y_shift);

    // Draw Lifelines
    draw_lifelines(&mut image, &lf_x_widths, inner_height, gen_ctx);

    // Draw Fragments
    let mut nest_shift : u32 = 1; // shift to display nested fragments
    let mut yshift : u32 = 3;

    draw_interaction_rec(&mut image,  gen_ctx, interaction, &lf_x_widths, gen_ctx.get_lf_num(), &mut nest_shift, &mut yshift);

    match multi_trace_txttoprint {
        None => {},
        Some( (mt_ttp,mt_print_width) ) => {
            let mut yshift : u32 = 0;
            for text in mt_ttp {
                let msg_x_pos = img_width - mt_print_width - MARGIN/2.0;
                let msg_y_pos = MARGIN + (yshift as f32)*VERTICAL_SIZE;
                draw_colored_text(&mut image,&text,msg_x_pos,msg_y_pos);
                yshift = yshift +2;
            }
        }
    }

    image.save(path).unwrap();
}