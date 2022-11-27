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



use std::path::PathBuf;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;
use crate::io::output::draw_commons::colored_text::draw_ttp::{DrawnColoredTextAlignment, new_image_with_colored_text};
use crate::io::output::draw_commons::colored_text::ttp::TextToPrint;
use crate::io::output::draw_commons::sd_drawing_conf::{FONT_WIDTH, MARGIN, VERTICAL_SIZE};
use crate::io::output::draw_traces::implem::ext_mu::extract_texts_on_multi_trace;
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;


pub fn draw_multitrace(gen_ctx : &GeneralContext,
                       co_localizations : &CoLocalizations,
                       multi_trace : &MultiTrace,
                       flags : &MultiTraceAnalysisFlags,
                       is_simulation : bool,
                       sim_crit_loop : bool,
                       sim_crit_act : bool,
                        parent_folder : &String,
                        output_file_name : &String) {
    // ***
    let output_file_name = format!("{:}.png", output_file_name);
    let output_path : PathBuf = [parent_folder, &output_file_name].iter().collect();
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
    new_image_with_colored_text(output_path.as_path(),&DrawnColoredTextAlignment::Left, img_width,img_height,text_lines)
}