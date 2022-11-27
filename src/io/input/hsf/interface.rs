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
use crate::io::file_extensions::HIBOU_SIGNATURE_FILE_EXTENSION;
use crate::io::input::error::HibouParsingError;

use crate::io::input::hsf::implem::parse_hsf_string;

pub fn parse_hsf_file(file_path : &str) -> Result<GeneralContext,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_SIGNATURE_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_SIGNATURE_FILE_EXTENSION.to_string()));
    }
    match fs::read_to_string(file_path) {
        Ok( unparsed_hsf_str ) => {
            return parse_hsf_string(unparsed_hsf_str);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}


