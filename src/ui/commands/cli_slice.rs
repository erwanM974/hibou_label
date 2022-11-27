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
use crate::io::input::htf::interface::parse_htf_file;
use crate::trace_manip::slice::conf::{SliceGenerationSelection, SliceKind};
use crate::trace_manip::slice::generate::generate_slices;


pub fn cli_slice(matches : &ArgMatches) -> (Vec<String>,u32) {
    let hsf_file_path = matches.value_of("hsf").unwrap();
    match parse_hsf_file(hsf_file_path) {
        Err(e) => {
            return (vec![e.to_string()],1);
        },
        Ok( gen_ctx ) => {
            let htf_file_path = matches.value_of("htf").unwrap();
            match parse_htf_file(&gen_ctx, htf_file_path) {
                Err(e) => {
                    return (vec![e.to_string()],1);
                },
                Ok( (co_localizations,multi_trace) ) => {
                    let mut ret_print = vec![];
                    ret_print.push( "SLICING TRACE".to_string());
                    ret_print.push( format!("from file '{}'",htf_file_path) );
                    ret_print.push( "".to_string());
                    // ***
                    let mu_name : &str = Path::new(htf_file_path).file_stem().unwrap().to_str().unwrap();
                    // ***
                    let parent_folder : Option<&str>;
                    match matches.value_of("parent_folder") {
                        None => {
                            parent_folder = None;
                        },
                        Some( folder_name ) => {
                            parent_folder = Some( folder_name );
                        }
                    }
                    // ***
                    let file_name_prefix_opt : Option<&str>;
                    match matches.value_of("name") {
                        None => {
                            file_name_prefix_opt = None;
                        },
                        Some( got_name ) => {
                            file_name_prefix_opt = Some( got_name );
                        }
                    }
                    // ***
                    let generation_selection : SliceGenerationSelection;
                    if matches.is_present("random") {
                        let extracted = matches.value_of("random").unwrap();
                        let content_str : String = extracted.chars().filter(|c| !c.is_whitespace()).collect();
                        let num_random_slices : u32 = content_str.parse::<u32>().unwrap();
                        // ***
                        let wide_only : bool;
                        if matches.is_present("wide") {
                            wide_only = true;
                        } else {
                            wide_only = false;
                        }
                        // ***
                        generation_selection = SliceGenerationSelection::Random(num_random_slices,wide_only);
                    } else {
                        generation_selection = SliceGenerationSelection::Exhaustive;
                    }
                    // ***
                    let generation_kind : SliceKind;
                    if matches.is_present("kind") {
                        let extracted = matches.value_of("kind").unwrap();
                        let content_str : String = extracted.chars().filter(|c| !c.is_whitespace()).collect();
                        match content_str.as_str() {
                            "prefix" => {
                                generation_kind = SliceKind::Prefix;
                            },
                            "suffix" => {
                                generation_kind = SliceKind::Suffix;
                            },
                            "slice" => {
                                generation_kind = SliceKind::Slice;
                            },
                            _ => {
                                ret_print.push( format!("unknown slice kind : {:}", content_str) );
                                return (ret_print,1);
                            }
                        }
                    } else {
                        generation_kind = SliceKind::Slice;
                    }
                    // ***
                    generate_slices(&gen_ctx,&co_localizations,mu_name,&multi_trace,parent_folder,file_name_prefix_opt,&generation_selection,&generation_kind);
                    return (ret_print,0);
                }
            }
        }
    }
}