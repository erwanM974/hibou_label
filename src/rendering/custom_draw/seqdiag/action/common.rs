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

use std::cmp;
use std::env;
use std::collections::{HashMap,HashSet};

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

use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;


use crate::rendering::sd_drawing_conf::*;
use crate::rendering::hibou_color_palette::*;

use crate::rendering::custom_draw::seqdiag::dimensions_tools::*;
use crate::rendering::textual::colored::colored_text::*;

use crate::rendering::custom_draw::utils::colored_text::draw_colored_text;
use crate::rendering::custom_draw::utils::arrow_heads::*;
use crate::rendering::custom_draw::seqdiag::lf_coords::DrawingLifelineCoords;
use crate::core::syntax::action::CommunicationSynchronicity;




pub fn draw_line_for_message_exchange(image : &mut RgbImage, synchronicity : &CommunicationSynchronicity, x_left : f32, x_right : f32, y_pos : f32) {
    match synchronicity {
        CommunicationSynchronicity::Asynchronous => {
            draw_line_segment_mut(image,
                                  (x_left, y_pos),
                                  (x_right, y_pos),
                                  Rgb(HCP_Black));
        },
        CommunicationSynchronicity::Synchronous => {
            draw_line_segment_mut(image,
                                  (x_left, y_pos - 1.5),
                                  (x_right, y_pos - 1.5),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_left, y_pos + 1.5),
                                  (x_right, y_pos + 1.5),
                                  Rgb(HCP_Black));
        }
    }

}