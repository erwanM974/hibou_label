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
use std::collections::HashMap;
use image::Rgb;

use crate::core::general_context::GeneralContext;
use crate::core::trace::{MultiTraceCanal,AnalysableMultiTrace,TraceAction,TraceActionKind};

use crate::rendering::textual::colored::colored_text::TextToPrint;
use crate::rendering::textual::colored::short_action::diagram_repr_trace_action;
use crate::rendering::hibou_color_palette::*;

pub fn extract_texts_on_multi_trace(gen_ctx : &GeneralContext,
                                    multi_trace : &AnalysableMultiTrace) -> Vec<Vec<TextToPrint>> {
    let mut all_texts : Vec<Vec<TextToPrint>> = Vec::new();
    for trace_canal in &multi_trace.canals {
        let mut canal_text : Vec<TextToPrint> = Vec::new();
        // ***
        if trace_canal.flag_hidden {
            canal_text.push( TextToPrint{text:"[".to_string(), color:Rgb(HCP_LightGray)} );
            let mut remaining_len = trace_canal.lifelines.len();
            for lf_id in &trace_canal.lifelines {
                let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
                canal_text.push( TextToPrint{text:lf_name, color:Rgb(HCP_LightGray)} );
                remaining_len = remaining_len -1;
                if remaining_len > 0 {
                    canal_text.push( TextToPrint{text:",".to_string(), color:Rgb(HCP_LightGray)} );
                }
            }
            canal_text.push( TextToPrint{text:"]".to_string(), color:Rgb(HCP_LightGray)} );
            canal_text.push( TextToPrint{text:format!("⚐{:}⚑", trace_canal.consumed), color:Rgb(HCP_LightGray)} );
        } else {
            canal_text.push( TextToPrint{text:"[".to_string(), color:Rgb(HC_Grammar_Symbol)} );
            let mut remaining_len = trace_canal.lifelines.len();
            for lf_id in &trace_canal.lifelines {
                let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
                canal_text.push( TextToPrint{text:lf_name, color:Rgb(HC_Lifeline)} );
                remaining_len = remaining_len -1;
                if remaining_len > 0 {
                    canal_text.push( TextToPrint{text:",".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                }
            }
            canal_text.push( TextToPrint{text:"]".to_string(), color:Rgb(HC_Grammar_Symbol)} );
            if trace_canal.simulated_before > 0 {
                canal_text.push( TextToPrint{text:format!("♧{:}", trace_canal.simulated_before), color:Rgb(HC_Grammar_Symbol)} );
            }
            // ***
            let canal_len = trace_canal.trace.len();
            // ***
            if trace_canal.consumed > 0 {
                canal_text.push( TextToPrint{text:format!("⚐{:}", trace_canal.consumed), color:Rgb(HC_Grammar_Symbol)} );
                if canal_len > 0 {
                    canal_text.push( TextToPrint{text:"←".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                    add_trace_text_to_canal(gen_ctx,canal_len,&mut canal_text,&(trace_canal.trace));
                } else {
                    canal_text.push( TextToPrint{text:"⚑".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                    if trace_canal.simulated_after > 0 {
                        canal_text.push( TextToPrint{text:format!("{:}♣", trace_canal.simulated_after), color:Rgb(HC_Grammar_Symbol)} );
                    }
                }
            } else {
                if canal_len > 0 {
                    canal_text.push( TextToPrint{text:"←⚐".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                    add_trace_text_to_canal(gen_ctx,canal_len,&mut canal_text,&(trace_canal.trace));
                    canal_text.push( TextToPrint{text:"⚑".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                } else {
                    if trace_canal.simulated_after > 0 {
                        canal_text.push( TextToPrint{text:format!("⚐{:}⚑", trace_canal.consumed), color:Rgb(HC_Grammar_Symbol)} );
                        canal_text.push( TextToPrint{text:format!("{:}♣", trace_canal.simulated_after), color:Rgb(HC_Grammar_Symbol)} );
                    } else {
                        canal_text.push( TextToPrint{text:"←⚐".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                        canal_text.push( TextToPrint{text:"ε".to_string(), color:Rgb(HCP_LightGray)} );
                        canal_text.push( TextToPrint{text:"⚑".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                    }
                }
            }
        }
        // ***
        canal_text.push( TextToPrint{text:" ".to_string(), color:Rgb(HCP_Black)} );
        all_texts.push(canal_text);
    }
    return all_texts;
}


fn add_trace_text_to_canal(gen_ctx: &GeneralContext, canal_len : usize, canal_text : &mut Vec<TextToPrint>, trace : &Vec<TraceAction>) {
    let mut remaining = canal_len;
    for act in trace {
        canal_text.append(&mut diagram_repr_trace_action(act, gen_ctx)  );
        remaining = remaining -1;
        if remaining > 0 {
            canal_text.push( TextToPrint{text:".".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        }
    }
}