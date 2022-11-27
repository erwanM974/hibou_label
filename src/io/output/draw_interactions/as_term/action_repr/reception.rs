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
use crate::core::language::syntax::action::{CommunicationSynchronicity, ReceptionAction};
use crate::io::textual_convention::{SYNTAX_RECEPTION, SYNTAX_RECEPTION_SYNCHRONOUS};


pub fn reception_as_gv_label (gen_ctx : &GeneralContext,
                    rc_act : &ReceptionAction) -> String {
    // ***
    let ms_name = gen_ctx.get_ms_name(rc_act.ms_id).unwrap();
    // ***
    let mut targ_names : Vec<String> = Vec::new();
    for rcp_lf_id in &rc_act.recipients {
        targ_names.push( gen_ctx.get_lf_name(*rcp_lf_id).unwrap() );
    }
    // ***
    let reception_symb : &'static str;
    match rc_act.synchronicity {
        CommunicationSynchronicity::Asynchronous => {
            reception_symb = SYNTAX_RECEPTION;
        },
        CommunicationSynchronicity::Synchronous => {
            reception_symb = SYNTAX_RECEPTION_SYNCHRONOUS;
        }
    }
    // ***
    let rcp_gate_str : String;
    match rc_act.origin_gt_id {
        None => {
            rcp_gate_str = "".to_string();
        },
        Some(orig_gt_id) => {
            rcp_gate_str = format!("[{:}]", gen_ctx.get_gt_name(orig_gt_id).unwrap());
        }
    }
    // ***
    let rcp_num = targ_names.len();
    if rcp_num == 1 {
        return format!("{}{}{}{}", targ_names.get(0).unwrap(), reception_symb, &rcp_gate_str, &ms_name);
    } else if rcp_num > 1 {
        return format!("({}){}{}{}", &targ_names.join(","), reception_symb, &rcp_gate_str, &ms_name);
    } else {
        panic!();
    }
}

