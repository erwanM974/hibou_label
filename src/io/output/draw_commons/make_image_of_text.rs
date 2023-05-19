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

use image::{Rgb, RgbImage};
use image_colored_text::draw::multi_line::{draw_multiline_colored_text, MultiLineTextAlignment};
use image_colored_text::draw::single_line::DrawCoord;
use image_colored_text::ttp::TextToPrint;
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use crate::io::output::draw_commons::font::{get_hibou_font, HIBOU_FONT_SCALE};

use crate::io::output::draw_commons::hibou_color_palette::HCP_White;
use crate::io::output::draw_commons::sd_drawing_conf::{FONT_HEIGHT, FONT_WIDTH, MARGIN, VERTICAL_SIZE};


pub fn new_image_with_colored_text(path : &Path,
                                   alignment : &MultiLineTextAlignment,
                                   text_lines : &Vec<Vec<TextToPrint>>) {
    //
    let lines_widths : Vec<f32> = text_lines.iter()
        .map(|x| TextToPrint::get_text_width(x, &get_hibou_font(), &HIBOU_FONT_SCALE) ).collect();
    let max_line_width = lines_widths.into_iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap() )
        .unwrap();
    // ***
    let img_width : f32 = 2.0*MARGIN + max_line_width;
    let img_height : f32 = 2.0*MARGIN + (text_lines.len() as f32)*(FONT_HEIGHT);
    // Draw Frame
    let mut image = RgbImage::new( img_width as u32, img_height as u32);
    draw_filled_rect_mut(&mut image,
                         Rect::at(0,0).of_size(img_width as u32,img_height as u32),
                         Rgb(HCP_White));
    // Draw content text
    draw_multiline_colored_text(&mut image,
                                &DrawCoord::StartingAt(MARGIN),
                                &DrawCoord::StartingAt(MARGIN),
                                alignment,
                                text_lines,
                                &get_hibou_font(),
                                &HIBOU_FONT_SCALE);
    // ***
    image.save(path).unwrap();
}







