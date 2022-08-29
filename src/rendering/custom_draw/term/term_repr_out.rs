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


use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};

use std::process::Command;

use crate::rendering::graphviz::graph::*;
use crate::rendering::graphviz::node_style::*;
use crate::rendering::graphviz::edge_style::*;
use crate::rendering::graphviz::common::*;

use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;
use crate::core::syntax::position::Position;

use crate::rendering::custom_draw::term::term_repr::interaction_repr;


pub fn to_term_repr(name : &String,
                    interaction : &Interaction,
                    gen_ctx : &GeneralContext) {
    let mut file = File::create(&format!("{:}.dot",name)).unwrap();
    file.write( interaction_repr(interaction,gen_ctx,name,false).as_bytes() );
    let status = Command::new("dot")
        .arg("-Tsvg:cairo")
        .arg(&format!("{:}.dot",name))
        .arg("-o")
        .arg(&format!("{:}.svg",name))
        .output();
}

pub fn to_term_repr_temp(name : &String,
                         interaction : &Interaction,
                         gen_ctx : &GeneralContext) {
    let mut file = File::create(&format!("temp/{:}.dot",name)).unwrap();
    file.write( interaction_repr(interaction,gen_ctx,name,false).as_bytes() );
    let status = Command::new("dot")
        .arg("-Tpng")
        .arg(&format!("temp/{:}.dot",name))
        .arg("-o")
        .arg(&format!("temp/{:}.png",name))
        .output();
}










