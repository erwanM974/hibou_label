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

pub fn cli_canonize(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            ret_print.push( e.to_string() );
            print_retval(ret_print);
            return -1;
        },
        Ok( (gen_ctx,my_int,_) ) => {
            let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
            let process_name : String;
            let search_all : bool;
            // ***
            if matches.is_present("searchall") {
                process_name = format!("{}_canon_all", file_name);
                search_all = true;
            } else {
                process_name = format!("{}_canon_one", file_name);
                search_all = false;
            }
            // ***
            if my_int.has_coregions() || my_int.has_ands() {
                ret_print.push( "Interaction term has coregions and/or ands -> Not Implemented".to_string() );
                print_retval(ret_print);
                return -1;
            }
            // ***
            canon_process_interaction_term(&my_int,&gen_ctx,&process_name,search_all);
            // ***
            ret_print.push( "".to_string());
            ret_print.push( "CANONIZING process for INTERACTION".to_string());
            ret_print.push( format!("from file '{}'",hsf_file_path) );
            ret_print.push( format!("on file : {}.svg",process_name) );
            ret_print.push( "".to_string());
            if search_all {
                ret_print.push( "(searched all transformation sequences)".to_string());
            } else {
                ret_print.push( "(searched one transformation sequence)".to_string());
            }
        }
    }
}