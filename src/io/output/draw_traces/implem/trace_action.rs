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
use image::Rgb;
use itertools::Itertools;

use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::io::output::draw_commons::colored_text::ttp::TextToPrint;
use crate::io::output::draw_commons::hibou_color_palette::*;
use crate::io::textual_convention::*;




pub fn diagram_repr_trace_action(action : &TraceAction, gen_ctx : &GeneralContext) -> Vec<TextToPrint> {
    let mut to_print : Vec<TextToPrint> = Vec::new();
    // ***
    {
        let lf_name = gen_ctx.get_lf_name(action.lf_id).unwrap();
        to_print.push( TextToPrint{text:lf_name,color:Rgb(HC_Lifeline)} );
    }
    // ***
    match &action.act_kind {
        &TraceActionKind::Reception => {
            to_print.push( TextToPrint{text:SYNTAX_RECEPTION.to_string(),color:Rgb(HC_Grammar_Symbol)} );
        },
        &TraceActionKind::Emission => {
            to_print.push( TextToPrint{text:SYNTAX_EMISSION.to_string(),color:Rgb(HC_Grammar_Symbol)} );
        }
    }
    // ***
    {
        let ms_name = gen_ctx.get_ms_name(action.ms_id).unwrap();
        to_print.push( TextToPrint{text:ms_name,color:Rgb(HC_Message)} );
    }
    // ***
    return to_print;
}


pub fn diagram_repr_trace_actions(actions : &HashSet<TraceAction>,
                                  gen_ctx : &GeneralContext,
                                  draw_brackets : bool) -> Vec<TextToPrint> {
    let mut inner_reprs : Vec<Vec<TextToPrint>> =
        actions.iter().sorted().map(|act| diagram_repr_trace_action(act, gen_ctx)).collect();
    if draw_brackets || inner_reprs.len() > 1 {
        let mut joined : Vec<TextToPrint> = vec![];
        {
            let mut rem = inner_reprs.len();
            for mut sub_repr in inner_reprs {
                rem = rem - 1;
                joined.append(&mut sub_repr);
                if rem > 0 {
                    joined.push(TextToPrint{text:",".to_string(), color:Rgb(HC_Grammar_Symbol)});
                }
            }
        }
        //let mut joined = inner_reprs.join(TextToPrint{text:",".to_string(), color:Rgb(HC_Grammar_Symbol)});
        let mut to_print : Vec<TextToPrint> = Vec::new();
        to_print.push( TextToPrint{text:"{".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        to_print.append(&mut joined);
        to_print.push( TextToPrint{text:"}".to_string(), color:Rgb(HC_Grammar_Symbol)} );
        return to_print;
    } else if inner_reprs.len() == 1 {
        return inner_reprs.pop().unwrap();
    } else {
        return vec![];
    }
}