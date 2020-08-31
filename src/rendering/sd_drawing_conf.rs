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

pub const HIBOU_GRAPHIC_FONT: &'static [u8] = include_bytes!("DejaVuSansMono.ttf");// include_bytes!("UbuntuMono-Regular.ttf");

// **********
pub const BASE_HORIZONTAL_SIZE : f32 = 50.0;
pub const BASE_VERTICAL_SIZE : f32 = 5.5;
pub const BASE_MARGIN : f32 = BASE_VERTICAL_SIZE;
pub const BASE_FRAGMENT_PADDING : f32 = 5.0;
pub const BASE_FRAGMENT_TITLE_MARGIN : f32 = 1.0;
// **********
pub const BASE_THICKNESS : f32 = 1.0;
pub const BASE_FONT_HEIGHT : f32 = 12.4;
pub const BASE_EVAL_X_PADDING : f32 = BASE_HORIZONTAL_SIZE/3.5;
pub const BASE_EVAL_HEIGHT : f32 = BASE_VERTICAL_SIZE/3.0;
pub const BASE_ARROW_HEAD_LENGTH : f32 = 5.0;
pub const BASE_FRONTIER_CIRCLE_RADIUS : f32 = 5.0;
// **********
const SCALE_FACTOR : f32 = 2.0;
// **********
pub const MARGIN : f32 = BASE_MARGIN*SCALE_FACTOR;
pub const HORIZONTAL_SIZE : f32 = BASE_HORIZONTAL_SIZE*SCALE_FACTOR;
pub const VERTICAL_SIZE : f32 = BASE_VERTICAL_SIZE*SCALE_FACTOR;
pub const FRAGMENT_PADDING : f32 = BASE_FRAGMENT_PADDING*SCALE_FACTOR;
pub const FRAGMENT_TITLE_MARGIN : f32 = BASE_FRAGMENT_TITLE_MARGIN*SCALE_FACTOR;
// **********
pub const THICKNESS : f32 = BASE_THICKNESS*SCALE_FACTOR;
pub const FONT_HEIGHT : f32 = BASE_FONT_HEIGHT*SCALE_FACTOR;
pub const FONT_X_PROPORTION : f32 = 1.0;
pub const FONT_WIDTH : f32 = FONT_HEIGHT*FONT_X_PROPORTION;
pub const EVAL_X_PADDING : f32 = BASE_EVAL_X_PADDING*SCALE_FACTOR;
pub const EVAL_HEIGHT : f32 = BASE_EVAL_HEIGHT*SCALE_FACTOR;
pub const ARROW_HEAD_LENGTH : f32 = BASE_ARROW_HEAD_LENGTH*SCALE_FACTOR;
pub const FRONTIER_CIRCLE_RADIUS : f32 = BASE_FRONTIER_CIRCLE_RADIUS*SCALE_FACTOR;
// **********