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
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use crate::output::rendering::colored_text::ttp::TextToPrint;

use crate::output::rendering::hibou_color_palette::HCP_White;
use crate::output::rendering::sd_drawing_conf::*;

pub enum DrawnColoredTextAlignment {
    Left,
    Center,
    Right
}

pub fn new_image_with_colored_text(path : &Path,
                                   alignment : &DrawnColoredTextAlignment,
                                   img_width : f32,
                                   img_height : f32,
                                   text_lines : Vec<Vec<TextToPrint>>) {
    // Draw Frame
    let mut image = RgbImage::new( img_width as u32, img_height as u32);
    draw_filled_rect_mut(&mut image, Rect::at(0,0).of_size(img_width as u32,img_height as u32), Rgb(HCP_White));
    // Draw content text
    let mut yshift : u32 = 0;
    for text in text_lines {
        let msg_x_pos : f32;
        match alignment {
            &DrawnColoredTextAlignment::Center => {
                msg_x_pos = img_width/2.0 - (TextToPrint::char_count(&text) as f32)*FONT_WIDTH/4.0;
            },
            &DrawnColoredTextAlignment::Left => {
                msg_x_pos = MARGIN;
            },
            &DrawnColoredTextAlignment::Right => {
                msg_x_pos = (img_width - MARGIN) - (TextToPrint::char_count(&text) as f32)*FONT_WIDTH/2.0;
            }
        }
        let msg_y_pos = MARGIN + (yshift as f32)*VERTICAL_SIZE;
        draw_colored_text(&mut image,&text,msg_x_pos,msg_y_pos);
        yshift = yshift + 2;
    }
    // ***
    image.save(path).unwrap();
}



pub fn draw_colored_text(image : &mut RgbImage,
                         to_print : &Vec<TextToPrint>,
                         msg_x_pos : f32,
                         msg_y_pos : f32) {
    let font = Font::try_from_bytes(HIBOU_GRAPHIC_FONT).unwrap();
    let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };

    // ***
    let mut char_count : u32 = 0;
    for txt_to_print in to_print {
        let mut my_text : String = (0..char_count).map(|_| " ").collect::<String>();
        my_text.push_str( &txt_to_print.text );
        draw_text_mut(image,
                      txt_to_print.color,
                      msg_x_pos as i32,
                      (msg_y_pos - (VERTICAL_SIZE as f32)/2.0) as i32,
                      scale,
                      &font,
                      &my_text
        );
        char_count = char_count + txt_to_print.text.chars().count() as u32;
    }
}