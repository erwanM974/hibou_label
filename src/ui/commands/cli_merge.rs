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


pub fn cli_merge(matches : &ArgMatches) -> (Vec<String>,u32) {

    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path,&ProcessKind::None) {
        Err(e) => {
            ret_print.push( e.to_string() );
            print_retval(ret_print);
            return -1;
        },
        Ok( (gen_ctx,my_int,hoptions) ) => {
            let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
            //
            let opt_returns : bool = matches.is_present("returns");
            let opt_complete : bool = matches.is_present("complete");
            let opt_graphic : bool = matches.is_present("graphic");
            // ***
            // ***
            if my_int.has_coregions() {
                ret_print.push( "Interaction term has coregions -> Not Implemented".to_string() );
                print_retval(ret_print);
                return -1;
            }
            merge_process_interaction_term(&my_int,&gen_ctx,opt_returns,opt_complete,opt_graphic,&file_name);
            // ***
            ret_print.push( "".to_string());
            ret_print.push( "MERGING process for INTERACTION".to_string());
            ret_print.push( format!("from file '{}'",hsf_file_path) );
            ret_print.push( "".to_string());
            ret_print.push( "".to_string());
        }
    }
}