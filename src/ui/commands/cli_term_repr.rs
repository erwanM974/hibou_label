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
use std::fs::write;
use std::path::Path;
use std::time::Instant;
use clap::App;
use clap::ArgMatches;
use crate::input::hsf_file::parse_hsf_file;
use crate::output::rendering::custom_draw::term::term_repr_out::to_term_repr;


pub fn cli_term_repr(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],-1)
        },
        Ok( (gen_ctx,my_int,_) ) => {
            let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
            let output_name = format!("{}_repr", file_name);
            // ***
            let mut ret_print = vec![];
            ret_print.push( "".to_string());
            ret_print.push( "DRAWING INTERACTION as syntax tree".to_string());
            ret_print.push( format!("from file '{}'",hsf_file_path) );
            ret_print.push( format!("on file : {}.svg",output_name) );
            ret_print.push( "".to_string());
            to_term_repr(&output_name, &my_int, &gen_ctx);
            return (ret_print,0);
        }
    }
}