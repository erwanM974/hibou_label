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
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::output::commons::textual_convention::{SYNTAX_ALT, SYNTAX_EMPTY, SYNTAX_LOOP_H, SYNTAX_LOOP_P, SYNTAX_LOOP_S, SYNTAX_LOOP_W, SYNTAX_PAR, SYNTAX_SEQ, SYNTAX_STRICT};
use crate::output::commons::textual_representation::model::model_action::{emission_as_text, reception_as_text};

pub fn interaction_as_text(gen_ctx : &GeneralContext,
                           interaction : &Interaction) -> String {
    return interaction_as_text_inner(gen_ctx,0,interaction);
}

fn binary_op_as_text(gen_ctx : &GeneralContext,
                     depth : usize,
                     op_text : &'static str,
                     i1 : &Interaction,
                     i2:&Interaction) -> String {
    let i1_string = interaction_as_text_inner(gen_ctx,depth+1,i1);
    let i2_string = interaction_as_text_inner(gen_ctx,depth+1,i2);
    return format!("{0}{1}(\n{2},\n{3}\n{0})", "\t".repeat(depth), op_text, i1_string, i2_string);
}

fn loop_kind_as_text(lk : &LoopKind) -> &'static str {
    match lk {
        LoopKind::SStrictSeq => {
            return SYNTAX_LOOP_S;
        },
        LoopKind::HHeadFirstWS => {
            return SYNTAX_LOOP_H;
        },
        LoopKind::WWeakSeq => {
            return SYNTAX_LOOP_W;
        },
        LoopKind::PInterleaving => {
            return SYNTAX_LOOP_P;
        }
    }
}

fn interaction_as_text_inner(gen_ctx : &GeneralContext,
                             depth : usize,
                            interaction : &Interaction) -> String {
    match interaction {
        &Interaction::Empty => {
            return format!("{}{}", "\t".repeat(depth), SYNTAX_EMPTY);
        },
        &Interaction::Emission(ref em_act) => {
            return format!("{}{}", "\t".repeat(depth), emission_as_text(em_act));
        },
        &Interaction::Reception(ref rc_act) => {
            return format!("{}{}", "\t".repeat(depth), reception_as_text(rc_act));
        },
        &Interaction::Strict(ref i1, ref i2) => {
            return binary_op_as_text(gen_ctx,depth,SYNTAX_STRICT,i1,i2);
        },
        &Interaction::Seq(ref i1, ref i2) => {
            return binary_op_as_text(gen_ctx,depth,SYNTAX_SEQ,i1,i2);
        },
        &Interaction::CoReg(_, ref i1, ref i2) => {
            panic!("todo");
        },
        &Interaction::Par(ref i1, ref i2) => {
            return binary_op_as_text(gen_ctx,depth,SYNTAX_PAR,i1,i2);
        },
        &Interaction::Alt(ref i1, ref i2) => {
            return binary_op_as_text(gen_ctx,depth,SYNTAX_ALT,i1,i2);
        },
        &Interaction::Loop(ref lk, ref i1) => {
            let i1_string = interaction_as_text_inner(gen_ctx,depth+1,i1);
            return format!("{0}{1}(\n{2}\n{0})", "\t".repeat(depth), loop_kind_as_text(lk), i1_string);
        },
        _ => {
            panic!("non-conform interaction");
        }
    }

}