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

use rusttype::{Font, Scale};
use crate::io::output::draw_commons::sd_drawing_conf::{FONT_HEIGHT, FONT_WIDTH};

const DRAWING_GRAPHIC_FONT: &'static [u8] = include_bytes!("DejaVuSansMono.ttf");

//  cannot call non-const fn `Font::<'_>::try_from_bytes` in constants
//pub const HIBOU_FONT : Font = Font::try_from_bytes(DRAWING_GRAPHIC_FONT).unwrap();

pub fn get_hibou_font() -> Font<'static> {
    Font::try_from_bytes(DRAWING_GRAPHIC_FONT).unwrap()
}

pub const HIBOU_FONT_SCALE : Scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };
