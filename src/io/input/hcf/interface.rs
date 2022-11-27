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
use std::path::Path;
use crate::core::general_context::GeneralContext;
use crate::io::input::error::HibouParsingError;
use crate::io::input::hcf::implem_ana::parse_hcf_string_for_ana;
use crate::io::input::hcf::implem_explo::parse_hcf_string_for_explore;
use crate::io::file_extensions::{HIBOU_CONFIGURATION_FILE_EXTENSION};
use crate::io::input::hcf::implem_canon::parse_hcf_string_for_canonize;

pub use crate::io::input::hcf::proc_options::opt_explore::HibouExploreOptions;
pub use crate::io::input::hcf::proc_options::opt_analyze::HibouAnalyzeOptions;
pub use crate::io::input::hcf::proc_options::opt_canonize::HibouCanonizeOptions;


pub fn parse_hcf_file_for_explore(gen_ctx : &GeneralContext, file_path : &str) -> Result<HibouExploreOptions,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_CONFIGURATION_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_CONFIGURATION_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_hcf_str ) => {
            return parse_hcf_string_for_explore(gen_ctx,unparsed_hcf_str, file_name);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

pub fn parse_hcf_file_for_ana(gen_ctx : &GeneralContext, file_path : &str) -> Result<HibouAnalyzeOptions,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_CONFIGURATION_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_CONFIGURATION_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_hcf_str ) => {
            return parse_hcf_string_for_ana(gen_ctx,unparsed_hcf_str, file_name);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

pub fn parse_hcf_file_for_canonize(gen_ctx : &GeneralContext, file_path : &str) -> Result<HibouCanonizeOptions,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_CONFIGURATION_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_CONFIGURATION_FILE_EXTENSION.to_string()));
    }
    let file_name : &str = path_object.file_stem().unwrap().to_str().unwrap();
    match fs::read_to_string(file_path) {
        Ok( unparsed_hcf_str ) => {
            return parse_hcf_string_for_canonize(gen_ctx,unparsed_hcf_str, file_name);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}

