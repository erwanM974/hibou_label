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
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::experiments::next_action::NextActionSpec;


pub fn generate_loop_alt_interactions(gen_ctx : &GeneralContext,
                                       nas : &mut NextActionSpec,
                                       num_act : u32) -> Vec<Interaction> {
    let mut ints = vec![];
    {
        let ialt = generate_alt_interaction(gen_ctx,nas,num_act);
        ints.push(Interaction::Loop(LoopKind::SStrictSeq,Box::new(ialt)));
    }
    if num_act >= 3 {
        for left_num_act in 1..=num_act/2 {
            let right_num_act = num_act - left_num_act;
            let left_ialt = generate_alt_interaction(gen_ctx,nas,left_num_act);
            let right_ialt = generate_alt_interaction(gen_ctx,nas,right_num_act);
            let left_int = Interaction::Loop(LoopKind::SStrictSeq,Box::new(left_ialt));
            let right_int = Interaction::Loop(LoopKind::SStrictSeq,Box::new(right_ialt));
            ints.push(Interaction::Par(Box::new(left_int),Box::new(right_int)));
        }
    }
    ints
}


fn generate_alt_interaction(gen_ctx : &GeneralContext,
                                     nas : &mut NextActionSpec,
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
            Interaction::Alt(a1,a2)
        },
        _ => {
            let a1 = Box::new(nas.get_next_action(gen_ctx));
            let i2 = generate_alt_interaction(gen_ctx,nas,num_act-1);
            Interaction::Alt(a1,Box::new(i2))
        }
    }
}