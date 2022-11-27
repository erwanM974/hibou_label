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
use crate::core::language::syntax::util::get_recursive_frag::{get_recursive_alt_frags, get_recursive_par_frags, get_recursive_seq_frags, get_recursive_strict_frags};
use crate::io::output::to_hfiles::interaction::model_action::{emission_as_hif_encoding, reception_as_hif_encoding};
use crate::io::textual_convention::{SYNTAX_ALT, SYNTAX_EMPTY, SYNTAX_LOOP_H, SYNTAX_LOOP_P, SYNTAX_LOOP_S, SYNTAX_LOOP_W, SYNTAX_PAR, SYNTAX_SEQ, SYNTAX_STRICT};


pub fn interaction_as_hif_encoding(gen_ctx : &GeneralContext,
                           interaction : &Interaction) -> String {
    return interaction_as_hif_encoding_inner(gen_ctx,0, interaction);
}

fn op_as_hif_encoding(gen_ctx : &GeneralContext,
                     depth : usize,
                     op_text : &'static str,
                     sub_ints : Vec<&Interaction>) -> String {
    let ints_strs : Vec<String> = sub_ints.iter().map(|i| interaction_as_hif_encoding_inner(gen_ctx,depth+1,i)).collect();
    return format!("{0}{1}(\n{2}\n{0})", "\t".repeat(depth), op_text, ints_strs.join(",\n"));
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

fn interaction_as_hif_encoding_inner(gen_ctx : &GeneralContext,
                             depth : usize,
                            interaction : &Interaction) -> String {
    match interaction {
        &Interaction::Empty => {
            return format!("{}{}", "\t".repeat(depth), SYNTAX_EMPTY);
        },
        &Interaction::Emission(ref em_act) => {
            return format!("{}{}", "\t".repeat(depth), emission_as_hif_encoding(gen_ctx,em_act));
        },
        &Interaction::Reception(ref rc_act) => {
            return format!("{}{}", "\t".repeat(depth), reception_as_hif_encoding(gen_ctx,rc_act));
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let mut strict_frags = get_recursive_strict_frags(i1);
            strict_frags.extend_from_slice(&mut get_recursive_strict_frags(i2));
            return op_as_hif_encoding(gen_ctx,depth,SYNTAX_STRICT,strict_frags);
        },
        &Interaction::Seq(ref i1, ref i2) => {
            let mut seq_frags = get_recursive_seq_frags(i1);
            seq_frags.extend_from_slice(&mut get_recursive_seq_frags(i2));
            return op_as_hif_encoding(gen_ctx,depth,SYNTAX_SEQ,seq_frags);
        },
        &Interaction::CoReg(_, ref i1, ref i2) => {
            panic!("todo");
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut par_frags = get_recursive_par_frags(i1);
            par_frags.extend_from_slice(&mut get_recursive_par_frags(i2));
            return op_as_hif_encoding(gen_ctx,depth,SYNTAX_PAR,par_frags);
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut alt_frags = get_recursive_alt_frags(i1);
            alt_frags.extend_from_slice(&mut get_recursive_alt_frags(i2));
            return op_as_hif_encoding(gen_ctx,depth,SYNTAX_ALT,alt_frags);
        },
        &Interaction::Loop(ref lk, ref i1) => {
            let i1_string = interaction_as_hif_encoding_inner(gen_ctx,depth+1,i1);
            return format!("{0}{1}(\n{2}\n{0})", "\t".repeat(depth), loop_kind_as_text(lk), i1_string);
        },
        _ => {
            panic!("non-conform interaction");
        }
    }

}