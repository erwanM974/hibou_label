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


use std::cmp;
use std::collections::{HashMap, HashSet};
use image::Rgb;
use itertools::Itertools;

use crate::core::general_context::GeneralContext;
use crate::core::trace::{TraceAction,TraceActionKind};
use crate::process::ana_proc::multitrace::{AnalysableMultiTraceCanal,AnalysableMultiTrace};

use crate::rendering::textual::colored::colored_text::TextToPrint;
use crate::rendering::textual::colored::short_action::diagram_repr_trace_action;
use crate::rendering::hibou_color_palette::*;





fn extract_texts_on_canal_hidden(gen_ctx : &GeneralContext,
                          lifelines : &HashSet<usize>,
                          trace_canal : &AnalysableMultiTraceCanal) -> Vec<Vec<TextToPrint>> {
    let mut canal_l1_text : Vec<TextToPrint> = Vec::new();
    add_lifelines_text_to_canal(gen_ctx,lifelines,&mut canal_l1_text,&mut 0, HCP_LightGray,HCP_LightGray);
    canal_l1_text.push( TextToPrint{text:" ".to_string(), color:Rgb(HCP_Black)} );
    // ***
    if trace_canal.simulated_before > 0 {
        canal_l1_text.push( TextToPrint{text:format!("♧{:}", trace_canal.simulated_before), color:Rgb(HCP_LightGray)} );
    }
    canal_l1_text.push( TextToPrint{text:format!("⚐{:}", trace_canal.consumed), color:Rgb(HCP_LightGray)} );
    canal_l1_text.push( TextToPrint{text:"⚑".to_string(), color:Rgb(HCP_LightGray)} );
    canal_l1_text.push( TextToPrint{text:" HID".to_string(), color:Rgb(HCP_LightGray)} );
    canal_l1_text.push( TextToPrint{text:" ".to_string(), color:Rgb(HCP_Black)} );
    return vec![ canal_l1_text, vec![] ];
}

fn extract_texts_on_canal_visible(gen_ctx : &GeneralContext,
                                 lifelines : &HashSet<usize>,
                                 trace_canal : &AnalysableMultiTraceCanal) -> Vec<Vec<TextToPrint>> {
    let canal_len = trace_canal.trace.len();
    let mut canal_l1_text : Vec<TextToPrint> = Vec::new();
    let mut char_width_canal : usize = 1;
    {
        add_lifelines_text_to_canal(gen_ctx,lifelines,&mut canal_l1_text,&mut char_width_canal, HC_Lifeline,HC_Grammar_Symbol);
        // ***
        canal_l1_text.push( TextToPrint{text:" ←".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        if canal_len > 0 {
            add_trace_text_to_canal(gen_ctx,canal_len,&mut canal_l1_text,&(trace_canal.trace));
        } else {
            canal_l1_text.push( TextToPrint{text:"ε".to_string(), color:Rgb(HCP_LightGray)} );
        }
        // ***
        canal_l1_text.push( TextToPrint{text:" ".to_string(), color:Rgb(HCP_Black)} );
    }
    // ***
    let mut canal_l2_text : Vec<TextToPrint> = Vec::new();
    {
        let blank_space : String = (0..char_width_canal).map(|_| " ").collect::<String>();
        canal_l2_text.push( TextToPrint{text:blank_space, color:Rgb(HCP_Black)} );
        // ***
        if trace_canal.simulated_before > 0 {
            canal_l2_text.push( TextToPrint{text:format!("♧{:}", trace_canal.simulated_before), color:Rgb(HC_Grammar_Symbol)} );
        }
        if (trace_canal.consumed > 0) || (trace_canal.simulated_after > 0) {
            canal_l2_text.push( TextToPrint{text:format!("⚐{:}", trace_canal.consumed), color:Rgb(HC_Grammar_Symbol)} );
            if canal_len == 0 {
                canal_l2_text.push( TextToPrint{text:"⚑".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                if trace_canal.simulated_after > 0 {
                    canal_l2_text.push( TextToPrint{text:format!("{:}♣", trace_canal.simulated_after), color:Rgb(HC_Grammar_Symbol)} );
                }
            }
        }
        canal_l2_text.push( TextToPrint{text:" ".to_string(), color:Rgb(HCP_Black)} );
    }
    // ***
    return vec![ canal_l1_text, canal_l2_text ];
}

pub fn extract_texts_on_multi_trace(gen_ctx : &GeneralContext,
                                    multi_trace : &AnalysableMultiTrace,
                                    is_simulation : bool,
                                    sim_crit_loop : bool,
                                    sim_crit_act : bool) -> Vec<Vec<TextToPrint>> {
    let mut all_texts : Vec<Vec<TextToPrint>> = Vec::new();
    for i in 0..gen_ctx.co_localizations.len() {
        let lifelines = gen_ctx.co_localizations.get(i).unwrap();
        let trace_canal = multi_trace.canals.get(i).unwrap();
        // ***
        if trace_canal.flag_hidden {
            all_texts.extend( extract_texts_on_canal_hidden(gen_ctx,lifelines,trace_canal) );
        } else {
            all_texts.extend( extract_texts_on_canal_visible(gen_ctx,lifelines,trace_canal) )
        }
    }
    if is_simulation {
        let mut simu_vec: Vec<TextToPrint> = vec![];
        // ***
        simu_vec.push( TextToPrint{text:" ⌕ ".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        if (!sim_crit_loop) && (!sim_crit_act) {
            simu_vec.push( TextToPrint{text:"*".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        } else {
            if sim_crit_loop {
                simu_vec.push( TextToPrint{text:format!("L{:} ",multi_trace.rem_loop_in_sim),
                    color:Rgb(HC_Grammar_Symbol)} );
            }
            if sim_crit_act {
                simu_vec.push( TextToPrint{text:format!("A{:}",multi_trace.rem_act_in_sim),
                    color:Rgb(HC_Grammar_Symbol)} );
            }
        }
        // ***
        all_texts.push(simu_vec);
    }
    //
    return all_texts;
}

fn add_lifelines_text_to_canal(gen_ctx : &GeneralContext,
                                      lifelines : &HashSet<usize>,
                                      canal_text : &mut Vec<TextToPrint>,
                                      char_width_canal : &mut usize,
                               lf_color : [u8;3],
                               gram_color : [u8;3]) {
    canal_text.push( TextToPrint{text:"[".to_string(), color:Rgb(gram_color)} );
    let mut remaining_len = lifelines.len();
    for lf_id in lifelines.iter().sorted() {
        let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
        *char_width_canal = *char_width_canal + lf_name.len();
        canal_text.push( TextToPrint{text:lf_name, color:Rgb(lf_color)} );
        remaining_len = remaining_len -1;
        if remaining_len > 0 {
            canal_text.push( TextToPrint{text:",".to_string(), color:Rgb(gram_color)} );
            *char_width_canal = *char_width_canal + 1;
        }
    }
    canal_text.push( TextToPrint{text:"]".to_string(), color:Rgb(gram_color)} );
    *char_width_canal = *char_width_canal + 2;
}


fn add_trace_text_to_canal(gen_ctx: &GeneralContext, canal_len : usize, canal_text : &mut Vec<TextToPrint>, trace : &Vec<HashSet<TraceAction>>) {
    let mut remaining_sequential_events = canal_len;
    for multi_act in trace {
        let mut remaining_simultaneous_actions = multi_act.len();
        if remaining_simultaneous_actions == 1 {
            let act = multi_act.iter().next().unwrap();
            canal_text.append(&mut diagram_repr_trace_action(act, gen_ctx)  );
        } else {
            canal_text.push( TextToPrint{text:"{".to_string(), color:Rgb(HC_Grammar_Symbol)} );
            for act in multi_act.iter().sorted() {
                canal_text.append(&mut diagram_repr_trace_action(act, gen_ctx)  );
                remaining_simultaneous_actions = remaining_simultaneous_actions - 1;
                if remaining_simultaneous_actions > 0 {
                    canal_text.push( TextToPrint{text:",".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                }
            }
            canal_text.push( TextToPrint{text:"}".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        }
        remaining_sequential_events = remaining_sequential_events -1;
        if remaining_sequential_events > 0 {
            canal_text.push( TextToPrint{text:".".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        }
    }
}