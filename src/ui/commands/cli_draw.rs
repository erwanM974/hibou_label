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
use crate::io::output::draw_interactions::interface::{InteractionGraphicalRepresentation,draw_interaction};



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
                    let rep_kind : InteractionGraphicalRepresentation;
                    if matches.is_present("representation") {
                        let extracted = matches.value_of("representation").unwrap();
                        match extracted {
                            "sd" => {
                                rep_kind = InteractionGraphicalRepresentation::AsSequenceDiagram;
                            },
                            "tt" => {
                                rep_kind = InteractionGraphicalRepresentation::AsTerm;
                            },
                            _ => {
                                return (vec![format!("unknown representation kind : {:}",extracted)], 1);
                            }
                        }
                    } else {
                        rep_kind = InteractionGraphicalRepresentation::AsSequenceDiagram;
                    }
                    // ***
                    let output_file_name : String;
                    if matches.is_present("output") {
                        let extracted = matches.value_of("output").unwrap();
                        output_file_name = extracted.to_string();
                    } else {
                        let file_name = Path::new(hif_file_path).file_stem().unwrap().to_str().unwrap();
                        output_file_name = format!("{}_repr", file_name);
                    }
                    // ***
                    draw_interaction(&gen_ctx, &int, &rep_kind, &"temp".to_string(), &"".to_string(), &output_file_name);
                    // ***
                    let mut ret_print = vec![];
                    ret_print.push( "".to_string());
                    ret_print.push( "DRAWING INTERACTION".to_string());
                    ret_print.push( format!("from file '{}'",hif_file_path) );
                    ret_print.push( format!("on file : {}",output_file_name) );
                    ret_print.push( "".to_string());
                    return (ret_print,0);
                }
            }
        }
    }
}