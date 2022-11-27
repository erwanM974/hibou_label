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
use crate::core::language::syntax::interaction::Interaction;
use crate::io::input::error::HibouParsingError;
use crate::io::input::hif::interaction::parse_hif_string;

use crate::io::file_extensions::{HIBOU_INTERACTION_FILE_EXTENSION};


pub fn parse_hif_file(gen_ctx : &GeneralContext, file_path : &str) -> Result<Interaction,HibouParsingError> {
    let path_object = Path::new(file_path);
    let file_extension : &str = path_object.extension().unwrap().to_str().unwrap();
    if file_extension != HIBOU_INTERACTION_FILE_EXTENSION {
        return Err( HibouParsingError::FileFormatError(file_extension.to_string(),HIBOU_INTERACTION_FILE_EXTENSION.to_string()));
    }
    match fs::read_to_string(file_path) {
        Ok( unparsed_hif_str ) => {
            return parse_hif_string(gen_ctx,unparsed_hif_str);
        },
        Err(e) => {
            return Err( HibouParsingError::FileError(e.to_string()) );
        }
    }
}


