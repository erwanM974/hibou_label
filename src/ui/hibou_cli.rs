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
use clap::App;
use clap::ArgMatches;


use crate::core::general_context::GeneralContext;

use crate::core::trace::*;




use crate::rendering::custom_draw::seqdiag::interaction::draw_interaction;


use crate::rendering::process::graphic_logger::GraphicProcessLogger;

use crate::process::log::*;

use crate::process::analysis::analyze;
use crate::process::exploration::explore;
use crate::from_hfiles::hsf_file::{ProcessKind,parse_hsf_file};
use crate::from_hfiles::htf_file::parse_htf_file;

use crate::plantuml::sequence::to_plant_uml_sd;
use crate::plantuml::automata_product::to_plant_uml_ap;
use crate::canonize::term_repr_out::to_term_repr;
use crate::canonize::process::canon_process_interaction_term;
use crate::merge_gates::process::merge_process_interaction_term;

fn get_ascii_border() -> &'static str {
    return r#"===================="#;
}

fn get_ascii_left() -> Vec<&'static str> {
    let mut my_vec = Vec::new();
    my_vec.push(r#" ___   Holistic   "#);
    my_vec.push(r#"(o,o)  Interaction"#);
    my_vec.push(r#"{`"'}  Behavioral "#);
    my_vec.push(r#"-"-"-  Oracle     "#);
    my_vec.push(r#" \_/   Utility    "#);
    my_vec.push(r#"                  "#);
    my_vec.push(r#"  V-label-0.6.0   "#);
    return my_vec;
}

fn print_retval(ret_print : Vec<String>) {
    let ascii_left = get_ascii_left();
    // ***
    println!("{}", get_ascii_border());
    if ret_print.len() >= ascii_left.len() {
        for i in 0..ascii_left.len() {
            println!("{}  |  {}", ascii_left[i], ret_print[i]);
        }
        for i in ascii_left.len()..ret_print.len() {
            println!("{} |  {}", " ".repeat(19),  ret_print[i]);
        }
    } else {
        for i in 0..ret_print.len() {
            println!("{}  |  {}", ascii_left[i], ret_print[i]);
        }
        for i in ret_print.len()..ascii_left.len() {
            println!("{}  |", ascii_left[i]);
        }
    }
    println!("{}", get_ascii_border());
}



pub fn hibou_cli() -> i32 {

    let yaml = load_yaml!("hibou_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut ret_print : Vec<String> = Vec::new();

    if let Some(matches) = matches.subcommand_matches("draw") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::None) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,my_int,hoptions) ) => {
                let spec_output_file : String;
                if matches.is_present("output") {
                    let extracted = matches.value_of("output").unwrap();
                    spec_output_file = format!("{}.png", extracted);
                } else {
                    let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
                    spec_output_file = format!("{}.png", file_name);
                }
                ret_print.push( "".to_string());
                ret_print.push( "DRAWING INTERACTION".to_string());
                ret_print.push( format!("from file '{}'",hsf_file_path) );
                ret_print.push( format!("on file : {}",spec_output_file) );
                ret_print.push( "".to_string());
                draw_interaction(&spec_output_file, &my_int,&gen_ctx,&None);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("puml_sd") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::None) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,my_int,hoptions) ) => {
                let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
                let spec_output_file = format!("{}_sd.puml", file_name);
                // ***
                ret_print.push( "".to_string());
                ret_print.push( "TRANSLATING INTERACTION to puml-sd".to_string());
                ret_print.push( format!("from file '{}'",hsf_file_path) );
                ret_print.push( format!("on file : {}",spec_output_file) );
                ret_print.push( "".to_string());
                to_plant_uml_sd(&spec_output_file,file_name, &my_int, &gen_ctx);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("puml_ap") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::None) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,my_int,hoptions) ) => {
                let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
                let spec_output_file = format!("{}_ap.puml", file_name);
                // ***
                ret_print.push( "".to_string());
                ret_print.push( "TRANSLATING INTERACTION to puml-ap".to_string());
                ret_print.push( format!("from file '{}'",hsf_file_path) );
                ret_print.push( format!("on file : {}",spec_output_file) );
                ret_print.push( "".to_string());
                to_plant_uml_ap(&spec_output_file,file_name, &my_int, &gen_ctx);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("canonize") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::None) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,my_int,hoptions) ) => {
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
    }else if let Some(matches) = matches.subcommand_matches("merge") {
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
    } else if let Some(matches) = matches.subcommand_matches("term_repr") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::None) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,my_int,hoptions) ) => {
                let file_name = Path::new(hsf_file_path).file_stem().unwrap().to_str().unwrap();
                let output_name = format!("{}_repr", file_name);
                // ***
                ret_print.push( "".to_string());
                ret_print.push( "DRAWING INTERACTION as syntax tree".to_string());
                ret_print.push( format!("from file '{}'",hsf_file_path) );
                ret_print.push( format!("on file : {}.svg",output_name) );
                ret_print.push( "".to_string());
                to_term_repr(&output_name, &my_int, &gen_ctx);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("explore") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::Explore) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,my_int,hoptions) ) => {
                // ***
                ret_print.push( "".to_string());
                ret_print.push( "EXPLORING SEMANTICS".to_string());
                ret_print.push( format!("of interaction from file '{}'",hsf_file_path) );
                ret_print.push( "".to_string());
                // ***
                let node_count = explore(my_int,
                        gen_ctx,
                        hoptions.pre_filters,
                        hoptions.strategy,
                        hoptions.frontier_priorities,
                        hoptions.loggers);

                ret_print.push( format!("node count : {:?}", node_count ) );
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("analyze") {
        let hsf_file_path = matches.value_of("hsf").unwrap();
        match parse_hsf_file(hsf_file_path,&ProcessKind::Analyze) {
            Err(e) => {
                ret_print.push( e.to_string() );
                print_retval(ret_print);
                return -1;
            },
            Ok( (gen_ctx,my_int,hoptions) ) => {
                let htf_file_path = matches.value_of("htf").unwrap();
                match parse_htf_file(htf_file_path,&gen_ctx) {
                    Err(e) => {
                        ret_print.push( e.to_string() );
                        print_retval(ret_print);
                        return -1;
                    },
                    Ok( multi_trace ) => {
                        ret_print.push( "ANALYZING TRACE".to_string());
                        ret_print.push( format!("from file '{}'",htf_file_path) );
                        ret_print.push( "W.R.T. INTERACTION".to_string());
                        ret_print.push( format!("from file '{}'",hsf_file_path) );
                        ret_print.push( "".to_string());

                        let (verdict,node_count) = analyze(my_int,
                                                           multi_trace,
                                                           gen_ctx,
                                                           hoptions.pre_filters,
                                                           hoptions.strategy,
                                                           hoptions.frontier_priorities,
                                                           hoptions.loggers,
                                                           hoptions.ana_kind.unwrap(),
                                                           hoptions.use_locfront,
                                                           hoptions.goal);

                        ret_print.push( format!("verdict    : '{}'", verdict.to_string() ) );
                        ret_print.push( format!("node count : {:?}", node_count ) );
                    }
                }
            }
        }
    }/* else if let Some(matches) = matches.subcommand_matches("bench_hvs1") {
        let hsf_file = matches.value_of("hsf").unwrap();
        let htf_file = matches.value_of("htf").unwrap();
        let report_file = matches.value_of("out").unwrap();
        if matches.is_present("csp") {
            hvs1_bench_analyze(hsf_file,htf_file,report_file,true);
        } else {
            hvs1_bench_analyze(hsf_file,htf_file,report_file,false);
        }
        ret_print.push( "BENCHMARKING hide vs simulation".to_string());
        ret_print.push( format!("with interaction from file '{}'",hsf_file) );
        ret_print.push( format!("and trace from file '{}'",htf_file) );
        ret_print.push( format!("output in file '{}'",report_file) );
        ret_print.push( "".to_string());
    } else if let Some(matches) = matches.subcommand_matches("bench_hvs2") {
        let hsf_file = matches.value_of("hsf").unwrap();
        let htf_file = matches.value_of("htf").unwrap();
        let report_file = matches.value_of("out").unwrap();
        hvs2_bench_analyze(hsf_file,htf_file,report_file);
        ret_print.push( "BENCHMARKING hide vs simulation".to_string());
        ret_print.push( format!("with interaction from file '{}'",hsf_file) );
        ret_print.push( format!("and trace from file '{}'",htf_file) );
        ret_print.push( format!("output in file '{}'",report_file) );
        ret_print.push( "".to_string());
    }*/ else {
        ret_print.push( "".to_string() );
        ret_print.push( "TYPE help or -h to get a summary of the utilities".to_string() );
    }
    // ***
    print_retval(ret_print);
    return 0;
}

