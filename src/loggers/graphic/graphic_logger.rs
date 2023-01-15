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


use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::io::output::graphviz::colors::{DotTranslatable, GraphvizColor};
use crate::io::output::graphviz::graph::GraphVizDiGraph;
use crate::io::output::graphviz::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem};
use crate::loggers::graphic::conf::{GraphicProcessLoggerLayout, GraphicProcessLoggerOutputFormat};


use crate::loggers::graphic::get_graph::filter::make_graphic_logger_filter;
use crate::loggers::graphic::get_graph::legend::make_graphic_logger_legend;
use crate::process::abstract_proc::common::FilterEliminationKind;

// ***

pub struct GraphicProcessLogger {
    output_format : GraphicProcessLoggerOutputFormat,
    pub layout : GraphicProcessLoggerLayout,
    // ***
    display_legend : bool,
    pub display_subprocesses : bool,
    // ***
    pub int_repr_sd : bool,
    pub int_repr_tt : bool,
    // ***
    pub temp_folder : String,
    // ***
    parent_folder : String,
    output_file_name : String,
    // ***
    pub graph : GraphVizDiGraph
}

impl GraphicProcessLogger {

    pub fn new(output_format : GraphicProcessLoggerOutputFormat,
               layout : GraphicProcessLoggerLayout,
               display_legend : bool,
               display_subprocesses : bool,
               int_repr_sd : bool,
               int_repr_tt : bool,
               temp_folder : String,
               parent_folder : String,
               output_file_name : String) -> GraphicProcessLogger {
        // ***
        // empties temp directory if exists
        match fs::remove_dir_all(&temp_folder) {
            Ok(_) => {
                // do nothing
            },
            Err(e) => {
                // do nothing
            }
        }
        // creates temp directory if not exist
        fs::create_dir_all(&temp_folder).unwrap();
        // creates parent directory if not exist
        if parent_folder != "".to_string() {
            fs::create_dir_all(&parent_folder).unwrap();
        }
        // ***
        return GraphicProcessLogger{output_format,
            layout,
            display_legend,
            display_subprocesses,
            int_repr_sd,
            int_repr_tt,
            temp_folder:temp_folder,
            parent_folder:parent_folder,
            output_file_name:output_file_name.clone(),
            graph:GraphVizDiGraph::new()};
    }

    pub fn terminate(&mut self,
                options_as_strs : &Vec<String>) {
        // *** LEGEND
        if self.display_legend {
            self.graph.nodes.push(Box::new(make_graphic_logger_legend(options_as_strs)));
        }
        // ***
        let dot_file_name = format!("{:}.dot", self.output_file_name);
        let dot_file_path : PathBuf = [&self.temp_folder, &dot_file_name].iter().collect();
        {
            // ***
            let mut dot_file = File::create(dot_file_path.as_path()).unwrap();
            // ***
            dot_file.write(self.graph.to_dot_string().as_bytes());
        }
        // ***
        match self.output_format {
            GraphicProcessLoggerOutputFormat::png => {
                let output_file_name = format!("{:}.png", self.output_file_name);
                let output_file_path : PathBuf = [&self.parent_folder, &output_file_name].iter().collect();
                // ***
                let status = Command::new("dot")
                    .arg("-Tpng")
                    .arg(dot_file_path.as_path())
                    .arg("-o")
                    .arg(output_file_path.as_path())
                    .output();
            },
            GraphicProcessLoggerOutputFormat::svg => {
                let output_file_name = format!("{:}.svg", self.output_file_name);
                let output_file_path : PathBuf = [&self.parent_folder, &output_file_name].iter().collect();
                // ***
                let status = Command::new("dot")
                    .arg("-Tsvg:cairo")
                    .arg(dot_file_path.as_path())
                    .arg("-o")
                    .arg(output_file_path.as_path())
                    .output();
            }
        }
    }

}


