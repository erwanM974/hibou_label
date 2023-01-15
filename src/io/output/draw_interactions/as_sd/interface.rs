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
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::draw_interactions::as_sd::interaction_repr::interaction::make_interaction_image;


pub fn draw_int_as_sd(gen_ctx : &GeneralContext,
                      interaction : &Interaction,
                      parent_folder : &String,
                      output_file_name : &String) {
    // ***
    let output_file_name = format!("{:}.png", output_file_name);
    let output_path : PathBuf = [parent_folder, &output_file_name].iter().collect();
    let image = make_interaction_image(gen_ctx,interaction);
    image.save(output_path.as_path());
}



