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
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;
use crate::output::rendering::colored_text::ttp::TextToPrint;
use crate::output::rendering::custom_draw::multitrace::ext_mu::extract_texts_on_multi_trace;
use crate::output::rendering::custom_draw::utils::colored_text::{DrawnColoredTextAlignment, new_image_with_colored_text};
use crate::output::rendering::sd_drawing_conf::{FONT_WIDTH, MARGIN, VERTICAL_SIZE};
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;

pub fn draw_multitrace(gen_ctx : &GeneralContext,
                       co_localizations : &CoLocalizations,
                   path_str : &String,
                   multi_trace : &MultiTrace,
                   flags : &MultiTraceAnalysisFlags,
                   is_simulation : bool,
                   sim_crit_loop : bool,
                   sim_crit_act : bool) {
    let path = Path::new( path_str );
    // ***
    let mut text_lines : Vec<Vec<TextToPrint>> = extract_texts_on_multi_trace(gen_ctx,
                                                                              co_localizations,
                                                                              multi_trace,
                                                                              flags,
                                                                              is_simulation,sim_crit_loop,sim_crit_act);
    // ***
    // ***
    let line_lens : Vec<usize> = text_lines.iter().map(|x| TextToPrint::char_count(x) ).collect();
    let max_x_shift = *line_lens.iter().max().unwrap();
    // ***
    let img_width : f32 = 2.0*MARGIN + (max_x_shift as f32)*FONT_WIDTH/2.0;
    let img_height : f32 = MARGIN + (text_lines.len() as f32)*(MARGIN + VERTICAL_SIZE);
    // ***
    new_image_with_colored_text(path,&DrawnColoredTextAlignment::Left, img_width,img_height,text_lines)
}