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
use imageproc::drawing::draw_line_segment_mut;
use imageproc::drawing::draw_cubic_bezier_curve_mut;

use crate::io::output::draw_commons::sd_drawing_conf::ARROW_HEAD_LENGTH;

// **********

pub fn draw_double_half_ellipsis_leftward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_cubic_bezier_curve_mut(image,
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
    draw_cubic_bezier_curve_mut(image,
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.0*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 1.0*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
}

pub fn draw_double_half_ellipsis_rightward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_cubic_bezier_curve_mut(image,
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.5*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.5*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
    draw_cubic_bezier_curve_mut(image,
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos + 0.5*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.0*(ARROW_HEAD_LENGTH as f32), y_pos - 0.25*(ARROW_HEAD_LENGTH as f32)),
                                (x_pos - 1.0*(ARROW_HEAD_LENGTH as f32), y_pos + 0.25*(ARROW_HEAD_LENGTH as f32)),
                                my_color);
}

pub fn draw_arrowhead_rightward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos - (ARROW_HEAD_LENGTH as f32), y_pos - (ARROW_HEAD_LENGTH as f32)),
                          my_color);
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos - (ARROW_HEAD_LENGTH as f32), y_pos + (ARROW_HEAD_LENGTH as f32)),
                          my_color);
}

pub fn draw_arrowhead_leftward(image : &mut RgbImage, x_pos : f32, y_pos : f32, my_color : Rgb<u8>) {
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos + (ARROW_HEAD_LENGTH as f32), y_pos - (ARROW_HEAD_LENGTH as f32)),
                          my_color);
    draw_line_segment_mut(image,
                          (x_pos, y_pos),
                          (x_pos + (ARROW_HEAD_LENGTH as f32), y_pos + (ARROW_HEAD_LENGTH as f32)),
                          my_color);
}

// **********

