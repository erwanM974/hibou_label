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

use crate::core::language::include::includes::PartialSemanticInclusion;
use crate::core::language::syntax::action::EmissionAction;
use crate::core::language::syntax::interaction::Interaction;

impl PartialSemanticInclusion for Interaction {

    fn is_included(&self, other: &Self) -> bool {
        if self == other {
            return true;
        }
        match self {
            Interaction::Empty => {
                return other.express_empty();
            },
            Interaction::Emission(em_act) => {
                return is_emission_included_in_interaction(em_act,other);
            }
        }
    }
    
}



fn is_emission_included_in_interaction(em_act : &EmissionAction, interaction : &Interaction) -> bool {
    match interaction {
        Interaction::Empty => {
            return false;
        },
        Interaction::Emission(em_act2) => {
            return em_act.is_included(em_act2);
        },
        Interaction::Reception(_) => {
            return false;
        },
        Interaction::Loop(_,i1) => {
            return is_emission_included_in_interaction(em_act,i1);
        },
        Interaction::Strict(i1,i2) => {
            if i1.express_empty() && is_emission_included_in_interaction(em_act,i2) {
                return true;
            }
            if i2.express_empty() && is_emission_included_in_interaction(em_act,i1) {
                return true;
            }
            return false;
        },
        Interaction::Seq(i1,i2) => {
            if i1.express_empty() && is_emission_included_in_interaction(em_act,i2) {
                return true;
            }
            if i2.express_empty() && is_emission_included_in_interaction(em_act,i1) {
                return true;
            }
            return false;
        },
        Interaction::CoReg(_,i1,i2) => {
            if i1.express_empty() && is_emission_included_in_interaction(em_act,i2) {
                return true;
            }
            if i2.express_empty() && is_emission_included_in_interaction(em_act,i1) {
                return true;
            }
            return false;
        },
        Interaction::Par(i1,i2) => {
            if i1.express_empty() && is_emission_included_in_interaction(em_act,i2) {
                return true;
            }
            if i2.express_empty() && is_emission_included_in_interaction(em_act,i1) {
                return true;
            }
            return false;
        },
        Interaction::Alt(i1,i2) => {
            if is_emission_included_in_interaction(em_act,i2) {
                return true;
            }
            if is_emission_included_in_interaction(em_act,i1) {
                return true;
            }
            return false;
        },
        Interaction::And(_,_) => {
            panic!();
        }
    }
}




