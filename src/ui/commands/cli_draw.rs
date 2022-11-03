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

use clap::ArgMatches;

use crate::input::hsf::interface::parse_hsf_file;
use crate::input::hif::interface::parse_hif_file;
use crate::output::rendering::custom_draw::seqdiag::interaction::draw_interaction;

pub fn cli_draw(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {
            let hif_file_path = matches.value_of("hif").unwrap();
            match parse_hif_file(&gen_ctx,hif_file_path) {
                Err(e) => {
                    return (vec![e.to_string()],1);
                },
                Ok( int) => {
                    let mut ret_print = vec![];
                    let spec_output_file : String;
                    if matches.is_present("output") {
                        let extracted = matches.value_of("output").unwrap();
                        spec_output_file = format!("{}.png", extracted);
                    } else {
                        let file_name = Path::new(hif_file_path).file_stem().unwrap().to_str().unwrap();
                        spec_output_file = format!("{}.png", file_name);
                    }
                    ret_print.push( "".to_string());
                    ret_print.push( "DRAWING INTERACTION".to_string());
                    ret_print.push( format!("from file '{}'",hif_file_path) );
                    ret_print.push( format!("on file : {}",spec_output_file) );
                    ret_print.push( "".to_string());
                    draw_interaction(&gen_ctx,&spec_output_file, &int);
                    return (ret_print,0);
                }
            }
        }
    }
}