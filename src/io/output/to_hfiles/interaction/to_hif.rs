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
use std::io::Write;
use std::path::Path;

use crate::core::language::syntax::interaction::Interaction;

use crate::core::general_context::GeneralContext;



use crate::io::output::to_hfiles::interaction::interaction::interaction_as_hif_encoding;



pub fn interaction_to_hif(file_path : &Path,
                          gen_ctx : &GeneralContext,
                          interaction : &Interaction) {
    let mut file = File::create(file_path).unwrap();
    file.write(interaction_as_hif_encoding(gen_ctx,&interaction).as_bytes() );
}