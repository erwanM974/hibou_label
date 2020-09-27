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
        canal_text.push( TextToPrint{text:"] ← ".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        // ***
        let canal_len = trace_canal.trace.len();
        if canal_len > 0 {
            let mut remaining = canal_len;
            for act in &(trace_canal.trace) {
                canal_text.append(&mut diagram_repr_trace_action(act, gen_ctx)  );
                remaining = remaining -1;
                if remaining > 0 {
                    canal_text.push( TextToPrint{text:".".to_string(), color:Rgb(HC_Grammar_Symbol)} );
                }
            }
        } else {
            canal_text.push( TextToPrint{text:"ε".to_string(), color:Rgb(HCP_LightGray)} );
        }
        canal_text.push( TextToPrint{text:" ".to_string(), color:Rgb(HCP_Black)} );
        all_texts.push(canal_text);
    }
    return all_texts;
}
