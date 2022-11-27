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
use crate::core::transformation::get_transfos::get_transfos::get_transfos;
use crate::core::transformation::transfodef::InteractionTransformation;
use crate::process::canon_proc::transformations::phases::{CanonizationPhase, transfos_phase1, transfos_phase2};


pub fn get_canonize_transfos(interaction : &Interaction, phase : &CanonizationPhase, get_all : bool) -> Vec<InteractionTransformation> {
    match phase {
        CanonizationPhase::FirstDefactorize => {
            return get_transfos(interaction,get_all,&transfos_phase1());
        },
        CanonizationPhase::SecondFactorize => {
            return get_transfos(interaction,get_all,&transfos_phase2());
        }
    }
}

