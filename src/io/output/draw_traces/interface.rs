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
use image_colored_text::draw::multi_line::MultiLineTextAlignment;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;
use crate::io::output::draw_commons::make_image_of_text::new_image_with_colored_text;
use crate::io::output::draw_traces::implem::ext_mu::extract_texts_on_multi_trace;
use crate::process::ana::node::flags::MultiTraceAnalysisFlags;


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
    let text_lines = extract_texts_on_multi_trace(gen_ctx,
                                                  co_localizations,
                                                  multi_trace,
                                                  flags,
                                                  is_simulation,
                                                  sim_crit_loop,
                                                  sim_crit_act);
    // ***
    new_image_with_colored_text(output_path.as_path(),
                                &MultiLineTextAlignment::Left,
                                &text_lines);
}