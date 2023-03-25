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


use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use graphviz_dot_builder::traits::DotTranslatable;

use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::draw_interactions::as_term::interaction_repr::repr::interaction_gv_repr;




pub fn draw_int_as_term(gen_ctx : &GeneralContext,
                    interaction : &Interaction,
                    temp_folder : &String,
                    parent_folder : &String,
                    output_file_name : &String) {
    // ***
    // creates directories if not exist
    fs::create_dir_all(&temp_folder).unwrap();
    // ***
    let temp_file_name = format!("{:}.dot", output_file_name);
    let temp_path : PathBuf = [temp_folder, &temp_file_name].iter().collect();
    let mut file = File::create(temp_path.as_path()).unwrap();
    file.write( interaction_gv_repr(gen_ctx,interaction).to_dot_string().as_bytes() );
    // ***
    let output_file_name = format!("{:}.png", output_file_name);
    let output_path : PathBuf = [parent_folder, &output_file_name].iter().collect();
    // ***
    let status = Command::new("dot")
        .arg("-Tpng")
        .arg(temp_path.as_path())
        .arg("-o")
        .arg(output_path.as_path())
        .output();
}






