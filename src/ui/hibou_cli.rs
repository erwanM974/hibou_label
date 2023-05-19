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


use clap::App;

use crate::ui::commands::cli_analyze::cli_analyze;
use crate::ui::commands::cli_draw::cli_draw;
use crate::ui::commands::cli_explore::cli_explore;
use crate::ui::commands::cli_glosem::cli_glosem;
use crate::ui::commands::cli_mutate_insert_noise::cli_mutate_insert_noise;
use crate::ui::commands::cli_mutate_swap_actions::cli_mutate_swap_actions;
use crate::ui::commands::cli_mutate_swap_components::cli_mutate_swap_components;
use crate::ui::commands::cli_puml_ap::cli_puml_ap;
use crate::ui::commands::cli_puml_sd::cli_puml_sd;
use crate::ui::commands::cli_slice::cli_slice;
use crate::ui::util::printing::print_on_hibou_cli;

pub fn hibou_cli() -> i32 {

    let yaml = load_yaml!("hibou_cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut ret_print : Vec<String> = vec![];
    let mut ret_code : u32 = 1;

    if let Some(matches) = matches.subcommand_matches("draw") {
        let mut got = cli_draw(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else if let Some(matches) = matches.subcommand_matches("puml_sd") {
        let mut got = cli_puml_sd(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else if let Some(matches) = matches.subcommand_matches("puml_ap") {
        let mut got = cli_puml_ap(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else if let Some(matches) = matches.subcommand_matches("glosem") {
        let mut got = cli_glosem(matches);
        ret_print = got.0;
        ret_code = got.1;
    }/* else if let Some(matches) = matches.subcommand_matches("canonize") {

    }*//* else if let Some(matches) = matches.subcommand_matches("merge") {

    } else if let Some(matches) = matches.subcommand_matches("term_repr") {

    }*/ else if let Some(matches) = matches.subcommand_matches("explore") {
        let mut got = cli_explore(matches);
        ret_print = got.0;
        ret_code = got.1;
    }/* else if let Some(matches) = matches.subcommand_matches("canonize") {
        let mut got = cli_canonize(matches);
        ret_print = got.0;
        ret_code = got.1;
    }*/ else if let Some(matches) = matches.subcommand_matches("analyze") {
        let mut got = cli_analyze(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else if let Some(matches) = matches.subcommand_matches("slice") {
        let mut got = cli_slice(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else if let Some(matches) = matches.subcommand_matches("mutate_insert_noise") {
        let mut got = cli_mutate_insert_noise(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else if let Some(matches) = matches.subcommand_matches("mutate_swap_actions") {
        let mut got = cli_mutate_swap_actions(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else if let Some(matches) = matches.subcommand_matches("mutate_swap_components") {
        let mut got = cli_mutate_swap_components(matches);
        ret_print = got.0;
        ret_code = got.1;
    } else {
        ret_print = vec!["".to_string(),"TYPE help or -h to get a summary of the utilities".to_string()];
        ret_code = 0
    }
    // ***
    print_on_hibou_cli(ret_print);
    return 0;
}

