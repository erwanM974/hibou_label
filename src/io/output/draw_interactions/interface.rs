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



use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::output::draw_interactions::as_sd::interface::draw_int_as_sd;
use crate::io::output::draw_interactions::as_term::interface::draw_int_as_term;




pub enum InteractionGraphicalRepresentation {
    AsSequenceDiagram,
    AsTerm
}

pub fn draw_interaction(gen_ctx : &GeneralContext,
                        int : &Interaction,
                        repr : &InteractionGraphicalRepresentation,
                        temp_folder : &String,
                        parent_folder : &String,
                        output_file_name : &String) {
    match repr {
        InteractionGraphicalRepresentation::AsSequenceDiagram => {
            draw_int_as_sd(gen_ctx,int,temp_folder,parent_folder,output_file_name);
        },
        InteractionGraphicalRepresentation::AsTerm => {
            draw_int_as_term(gen_ctx,int,temp_folder,parent_folder,output_file_name);
        }
    }
}