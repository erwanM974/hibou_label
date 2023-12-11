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
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction};
use crate::core::language::syntax::interaction::Interaction;
use crate::experiments::next_action::NextActionSpec;


pub fn generate_par_strict_interaction(gen_ctx : &GeneralContext,
                                       nas : &mut NextActionSpec,
                                       num_par : u32,
                                       num_act : u32) -> Interaction {
    match num_act {
        0 => {
            panic!("should not be reached")
        },
        1 => {
            nas.get_next_action(gen_ctx)
        },
        2 => {
            let a1 = Box::new(nas.get_next_action(gen_ctx));
            let a2 = Box::new(nas.get_next_action(gen_ctx));
            if num_par > 0 {
                Interaction::Par(a1,a2)
            } else {
                Interaction::Strict(a1,a2)
            }
        },
        _ => {
            let a1 = Box::new(nas.get_next_action(gen_ctx));
            if num_par > 0 {
                let i2 = generate_par_strict_interaction(gen_ctx,nas,num_par - 1, num_act-1);
                Interaction::Par(a1,Box::new(i2))
            } else {
                let i2 = generate_par_strict_interaction(gen_ctx,nas,0, num_act-1);
                Interaction::Strict(a1,Box::new(i2))
            }
        }
    }
}


