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
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, EmissionTargetRef};
use crate::io::textual_convention::{SYNTAX_EMISSION, SYNTAX_EMISSION_SYNCHRONOUS};


pub fn emission_as_gv_label(gen_ctx : &GeneralContext,
                    em_act : &EmissionAction) -> String {
    // ***
    let ms_name = gen_ctx.get_ms_name(em_act.ms_id).unwrap();
    let lf_name = gen_ctx.get_lf_name(em_act.origin_lf_id).unwrap();
    // ***
    let mut targ_names : Vec<String> = Vec::new();
    for targ_ref in &em_act.targets {
        match targ_ref {
            EmissionTargetRef::Lifeline(tar_lf_id) => {
                targ_names.push( gen_ctx.get_lf_name(*tar_lf_id).unwrap() );
            },
            EmissionTargetRef::Gate(tar_gt_id) => {
                targ_names.push( gen_ctx.get_gt_name(*tar_gt_id).unwrap() );
            }
        }
    }
    // ***
    let emission_symb : &'static str;
    match em_act.synchronicity {
        CommunicationSynchronicity::Asynchronous => {
            emission_symb = SYNTAX_EMISSION;
        },
        CommunicationSynchronicity::Synchronous => {
            emission_symb = SYNTAX_EMISSION_SYNCHRONOUS;
        }
    }
    // ***
    if targ_names.len() == 0 {
        return format!("{}{}{}", &lf_name, emission_symb, &ms_name);
    } else {
        return format!("{}{}{}({})", &lf_name, emission_symb, &ms_name, &targ_names.join(","));
    }
}