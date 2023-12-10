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


use std::fmt::Formatter;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::transformation::get_transfos::get_all_transfos::get_all_transformations_rec;
use crate::core::transformation::get_transfos::get_one_transfo::get_one_transformation_rec;
use crate::core::transformation::transfokind::InteractionTransformationKind;
use crate::core::transformation::transfores::InteractionTransformationResult;

pub struct InteractionTransformationPhase {
    pub transfos : Vec<InteractionTransformationKind>,
    /*
    pub get_all : bool,
    // a priority order in the application of the InteractionTransformationKind
    pub ordered : bool*/
}

impl std::fmt::Display for InteractionTransformationPhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut got = self.transfos.iter().fold("[".to_string(), |prev,phase|
            format!("{:},{:}",prev,phase)
        );
        got.push_str("]");
        write!(f, "{}", got)
    }
}

impl InteractionTransformationPhase {
    pub fn new(transfos : Vec<InteractionTransformationKind>,/*get_all : bool,ordered : bool*/) -> InteractionTransformationPhase {
        return InteractionTransformationPhase{transfos/*,get_all,ordered*/};
    }

    pub fn get_transfos(&self, interaction : &Interaction, get_all : bool) -> Vec<InteractionTransformationResult> {
        if get_all {
            return get_all_transformations_rec(&self.transfos,interaction);
        } else {
            match get_one_transformation_rec(&self.transfos,interaction) {
                None => {
                    return Vec::new();
                },
                Some( got ) => {
                    return vec![got];
                }
            }
        }
    }

    /*pub fn apply_phase(&self,interaction : &Interaction) -> Vec<InteractionTransformationResult> {
        if self.ordered {
            panic!("not implemented");
        } else {
            return self.get_transfos(interaction,&self.transfos);
        }
    }

    fn get_transfos(&self,
                    interaction : &Interaction,
                    transfos : &Vec<InteractionTransformationKind>) -> Vec<InteractionTransformationResult> {
        if self.get_all {
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
    }*/
}


