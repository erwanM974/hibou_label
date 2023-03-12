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
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::general_context::GeneralContext;
use crate::io::textual_convention::{SYNTAX_EMISSION,SYNTAX_RECEPTION};

pub fn trace_action_as_gv_label (gen_ctx : &GeneralContext,
                                 tr_act : &TraceAction) -> String {
    let lf_name = gen_ctx.get_lf_name(tr_act.lf_id).unwrap();
    // ***
    let act_kind_label : &str;
    match &tr_act.act_kind {
        &TraceActionKind::Reception => {
            act_kind_label = SYNTAX_RECEPTION;
        },
        &TraceActionKind::Emission => {
            act_kind_label = SYNTAX_EMISSION;
        }
    }
    // ***
    let ms_name = gen_ctx.get_ms_name(tr_act.ms_id).unwrap();
    // ***
    return format!("{}{}{}",lf_name,act_kind_label,ms_name);
}

pub fn trace_actions_as_gv_label<'a,I>(gen_ctx : &GeneralContext,
                                     tracts : I) -> String
    where I: Iterator<Item = &'a TraceAction> {
    let sub_strs : Vec<String> = tracts.map(|act| trace_action_as_gv_label(gen_ctx,act)).collect();
    return format!("{{{}}}", sub_strs.join(","));
}