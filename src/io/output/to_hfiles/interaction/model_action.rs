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
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, EmissionTargetRef, ReceptionAction};


pub fn emission_as_hif_encoding(gen_ctx : &GeneralContext,
                                em_act : &EmissionAction) -> String {
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
    let synch_key : String;
    match em_act.synchronicity {
        CommunicationSynchronicity::Asynchronous => {
            synch_key = "".to_string();
        },
        CommunicationSynchronicity::Synchronous => {
            synch_key = "<synch>".to_string();
        }
    }
    // ***
    let lf_name = gen_ctx.get_lf_name(em_act.origin_lf_id).unwrap();
    let ms_name = gen_ctx.get_ms_name(em_act.ms_id).unwrap();
    let rcp_num = targ_names.len();
    if rcp_num == 0 {
        return format!("{} -- {}{} ->|", &lf_name, synch_key, &ms_name);
    } else if rcp_num == 1 {
        return format!("{} -- {}{} -> {}", &lf_name, synch_key, &ms_name, targ_names.get(0).unwrap());
    } else {
        return format!("{} -- {}{} -> ({})", &lf_name, synch_key, &ms_name, &targ_names.join(","));
    }
}


pub fn reception_as_hif_encoding(gen_ctx : &GeneralContext,
                                 rc_act : &ReceptionAction) -> String {
    let mut targ_names : Vec<String> = Vec::new();
    for rcp_lf_id in &rc_act.recipients {
        targ_names.push( gen_ctx.get_lf_name(*rcp_lf_id).unwrap() );
    }
    // ***
    let synch_key : String;
    match rc_act.synchronicity {
        CommunicationSynchronicity::Asynchronous => {
            synch_key = "".to_string();
        },
        CommunicationSynchronicity::Synchronous => {
            synch_key = "<synch>".to_string();
        }
    }
    // ***
    let gate_str : String;
    match rc_act.origin_gt_id {
        None => {
            gate_str = "".to_string();
        },
        Some(orig_gt_id) => {
            gate_str = format!("{} -- ",gen_ctx.get_gt_name(orig_gt_id).unwrap());
        }
    }
    // ***
    let ms_name = gen_ctx.get_ms_name(rc_act.ms_id).unwrap();
    let rcp_num = targ_names.len();
    if rcp_num == 1 {
        return format!("{}{}{} -> {}", gate_str, synch_key, &ms_name, targ_names.get(0).unwrap());
    } else if rcp_num > 1 {
        return format!("{}{}{} -> ({})", gate_str, synch_key, &ms_name, &targ_names.join(","));
    } else {
        panic!();
    }
}