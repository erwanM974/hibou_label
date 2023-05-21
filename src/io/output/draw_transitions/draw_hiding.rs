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

use std::collections::HashSet;
use std::path::Path;

use image::Rgb;
use image_colored_text::draw::multi_line::MultiLineTextAlignment;
use image_colored_text::ttp::TextToPrint;

use crate::core::general_context::GeneralContext;
use crate::io::output::draw_commons::hibou_color_palette::*;
use crate::io::output::draw_commons::make_image_of_text::new_image_with_colored_text;


// **********


pub fn draw_hiding(path : &Path,
                   gen_ctx : &GeneralContext,
                   lfs_to_hide : &HashSet<usize>) {
    // ***
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    {
        let mut ttp = Vec::new();
        ttp.push( TextToPrint::new("eliminate lifelines [".to_string(),Rgb(HCP_Black)) );
        // ***
        {
            let names_of_lfs : Vec<TextToPrint> = lfs_to_hide.iter()
                .map(|lf_id|
                    TextToPrint::new(gen_ctx.get_lf_name(*lf_id).unwrap(),Rgb(HC_Lifeline))
                ).collect();

            let mut to_add = names_of_lfs.into_iter()
                .fold(vec![],
                      |prev,lf|
                          [prev, vec![TextToPrint::new(",".to_string(),Rgb(HCP_Black)), lf]].concat());

            ttp.append( &mut to_add);
        }
        // ***
        ttp.push( TextToPrint::new("]".to_string(),Rgb(HCP_Black)) );
        text_lines.push( ttp );
    }
    // ***
    new_image_with_colored_text(path,
                                &MultiLineTextAlignment::Center,
                                &text_lines);
}


