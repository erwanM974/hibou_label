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


use std::path::Path;

use image::Rgb;
use crate::core::language::position::position::Position;
use crate::core::transformation::transfokind::InteractionTransformationKind;

use crate::io::output::draw_commons::colored_text::draw_ttp::{DrawnColoredTextAlignment, new_image_with_colored_text};
use crate::io::output::draw_commons::colored_text::ttp::TextToPrint;
use crate::io::output::draw_commons::hibou_color_palette::*;
use crate::io::output::draw_commons::sd_drawing_conf::*;


// **********



pub fn draw_string_label(path : &Path,string_label : String) {
    // ***
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    {
        let mut ttp = Vec::new();
        // ***
        ttp.push( TextToPrint{text:string_label,color:Rgb(HCP_Black)} );
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


