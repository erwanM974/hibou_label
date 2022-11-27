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


use crate::core::language::syntax::interaction::Interaction;
use crate::core::transformation::get_transfos::get_all_transfos::get_all_transformations_rec;
use crate::core::transformation::get_transfos::get_one_transfo::get_one_transformation_rec;
use crate::core::transformation::transfodef::InteractionTransformation;
use crate::core::transformation::transfokind::InteractionTransformationKind;


pub fn get_transfos(interaction : &Interaction,
                    get_all : bool,
                    transfos : &Vec<(InteractionTransformationKind, &dyn Fn(&Interaction) -> Vec<Interaction>)>)
            -> Vec<InteractionTransformation> {
    if get_all {
        return get_all_transformations_rec(transfos,interaction);
    } else {
        match get_one_transformation_rec(transfos,interaction) {
            None => {
                return Vec::new();
            },
            Some( got ) => {
                return vec![got];
            }
        }
    }
}

