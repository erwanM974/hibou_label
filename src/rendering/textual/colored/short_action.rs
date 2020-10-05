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
use image::Rgb;

use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;


use crate::rendering::textual::convention::*;
use crate::rendering::textual::colored::colored_text::*;
use crate::rendering::hibou_color_palette::*;

use crate::core::trace::{TraceAction,TraceActionKind};


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


