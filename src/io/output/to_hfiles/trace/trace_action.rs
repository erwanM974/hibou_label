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


use std::collections::HashSet;

use itertools::Itertools;

use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::io::textual_convention::{SYNTAX_EMISSION, SYNTAX_RECEPTION};

pub fn trace_actions_as_htf_encoding(gen_ctx : &GeneralContext, actions : &HashSet<TraceAction>) -> String {
    if actions.len() == 1 {
        return trace_action_as_htf_encoding(gen_ctx,actions.iter().next().unwrap());
    } else {
        let mut actions_as_text= vec![];
        for action in actions.iter().sorted() {
            actions_as_text.push( trace_action_as_htf_encoding(gen_ctx,action) );
        }
        return format!( "{{{:}}}", actions_as_text.join(",") );
    }
}


fn trace_action_as_htf_encoding(gen_ctx : &GeneralContext, action : &TraceAction) -> String {
    let lf_name = gen_ctx.get_lf_name(action.lf_id).unwrap();
    let ms_name = gen_ctx.get_ms_name(action.ms_id).unwrap();
    // ***
    match &action.act_kind {
        &TraceActionKind::Reception => {
            return format!( "{:}{:}{:}", lf_name, SYNTAX_RECEPTION, ms_name );
        },
        &TraceActionKind::Emission => {
            return format!( "{:}{:}{:}", lf_name, SYNTAX_EMISSION, ms_name );
        }
    }
}