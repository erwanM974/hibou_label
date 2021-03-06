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
use image::{Rgb, RgbImage};
use rusttype::{FontCollection, Scale};
use imageproc::drawing::draw_text_mut;

use crate::rendering::textual::convention::*;
use crate::rendering::sd_drawing_conf::*;
use crate::rendering::textual::colored::colored_text::*;

pub fn draw_colored_text(image : &mut RgbImage,
                         to_print : &Vec<TextToPrint>,
                         msg_x_pos : f32,
                         msg_y_pos : f32) {
    let font = FontCollection::from_bytes(HIBOU_GRAPHIC_FONT).unwrap().into_font().unwrap();
    let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };

    // ***
    let mut char_count : u32 = 0;
    for txt_to_print in to_print {
        let mut my_text : String = (0..char_count).map(|_| " ").collect::<String>();
        my_text.push_str( &txt_to_print.text );
        draw_text_mut(image,
                      txt_to_print.color,
                      msg_x_pos as u32,
                      (msg_y_pos - (VERTICAL_SIZE as f32)/2.0) as u32,
                      scale,
                      &font,
                      &my_text
        );
        char_count = char_count + txt_to_print.text.chars().count() as u32;
    }
}