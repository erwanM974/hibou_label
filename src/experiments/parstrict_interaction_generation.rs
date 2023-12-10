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



pub struct NextActionSpec {
    pub next_lf : usize,
    pub next_ms : usize
}

impl NextActionSpec {
    pub fn new(next_lf: usize, next_ms: usize) -> Self {
        Self { next_lf, next_ms }
    }
}

pub fn get_next_action(gen_ctx : &GeneralContext,
                       nas : &mut NextActionSpec) -> Interaction {
    let act = EmissionAction::new(
        nas.next_lf,
        nas.next_ms,
        CommunicationSynchronicity::Asynchronous,
        vec![]
    );
    let int = Interaction::Emission(act);
    if nas.next_ms + 1 < gen_ctx.get_ms_num() {
        nas.next_ms += 1;
    } else {
        if nas.next_lf + 1 < gen_ctx.get_lf_num() {
            nas.next_ms = 0;
            nas.next_lf += 1;
        } else {
            nas.next_ms = 0;
            nas.next_lf = 0;
        }
    }
    int
}

pub fn generate_par_strict_interaction(gen_ctx : &GeneralContext,
                                       nas : &mut NextActionSpec,
                                       num_par : u32,
                                       num_act : u32) -> Interaction {
    match num_act {
        0 => {
            panic!("should not be reached")
        },
        1 => {
            get_next_action(gen_ctx,nas)
        },
        2 => {
            let a1 = Box::new(get_next_action(gen_ctx,nas));
            let a2 = Box::new(get_next_action(gen_ctx,nas));
            if num_par > 0 {
                Interaction::Par(a1,a2)
            } else {
                Interaction::Strict(a1,a2)
            }
        },
        _ => {
            let a1 = Box::new(get_next_action(gen_ctx,nas));
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


