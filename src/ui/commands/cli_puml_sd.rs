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

use crate::io::input::hsf::interface::parse_hsf_file;
use crate::io::input::hif::interface::parse_hif_file;

use crate::plantuml::sequence::to_plant_uml_sd;

pub fn cli_puml_sd(matches : &ArgMatches) -> (Vec<String>,u32) {
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
                    let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
                    let spec_output_file = format!("{}_sd.puml", file_name);
                    // ***
                    ret_print.push( "".to_string());
                    ret_print.push( "TRANSLATING INTERACTION to puml-sd".to_string());
                    ret_print.push( format!("from file '{}'",hsf_file_path) );
                    ret_print.push( format!("on file : {}",spec_output_file) );
                    ret_print.push( "".to_string());
                    to_plant_uml_sd(&spec_output_file,file_name, &int, &gen_ctx);
                    // ***
                    return (ret_print,0);
                }
            }
        }
    }
}