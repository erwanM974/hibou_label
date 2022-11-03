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

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

use image::Rgb;
use itertools::Itertools;
use crate::core::colocalizations::CoLocalizations;

use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::position::position::Position;
use crate::output::commons::textual_representation::position::position_to_text;
use crate::output::rendering::colored_text::trace_action::diagram_repr_trace_action;
use crate::output::rendering::colored_text::ttp::TextToPrint;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::output::rendering::custom_draw::utils::colored_text::{DrawnColoredTextAlignment, new_image_with_colored_text};
use crate::output::rendering::hibou_color_palette::*;
use crate::output::rendering::sd_drawing_conf::*;

// **********



pub fn draw_firing_simple(path_str : &String,
                   gen_ctx : &GeneralContext,
                   action_position : &Position,
                   executed_actions : &HashSet<TraceAction>) {
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    draw_firing_finalize(path_str,text_lines, gen_ctx, action_position, executed_actions);
}


pub fn draw_firing_analysis(path_str : &String,
                   gen_ctx : &GeneralContext,
                   co_localizations : &CoLocalizations,
                   action_position : &Position,
                   executed_actions : &HashSet<TraceAction>,
                   consu_set : &HashSet<usize>,
                   sim_map : &HashMap<usize,SimulationStepKind>) {

    // ***
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    if consu_set.len() > 0 || sim_map.len() > 0 {
        let mut ttp: Vec<TextToPrint> = Vec::new();
        for coloc_id in consu_set {
            ttp.push( TextToPrint{text:"C".to_string(),color:Rgb(HC_Grammar_Symbol)} );
            write_lfs_texts(gen_ctx,co_localizations,coloc_id,Rgb(HC_Grammar_Symbol),&mut ttp);
            ttp.push( TextToPrint{text:" ".to_string(),color:Rgb(HCP_Black)} );
        }
        for (coloc_id,sim_kind) in sim_map {
            ttp.push( TextToPrint{text:"S".to_string(),color:Rgb(HCP_LightGray)} );
            write_lfs_texts(gen_ctx,co_localizations,coloc_id,Rgb(HCP_LightGray),&mut ttp);
            match sim_kind {
                SimulationStepKind::BeforeStart => {
                    ttp.push( TextToPrint{text:"↑".to_string(),color:Rgb(HCP_LightGray)} );
                },
                SimulationStepKind::AfterEnd => {
                    ttp.push( TextToPrint{text:"↓".to_string(),color:Rgb(HCP_LightGray)} );
                }
            }
            // ***
            ttp.push( TextToPrint{text:" ".to_string(),color:Rgb(HCP_Black)} );
        }
        text_lines.push( ttp );
    }
    // ***
    draw_firing_finalize(path_str,text_lines, gen_ctx, action_position, executed_actions);
}


fn draw_firing_finalize(path_str : &String,
                        text_lines : Vec<Vec<TextToPrint>>,
       gen_ctx : &GeneralContext,
       action_position : &Position,
       executed_actions : &HashSet<TraceAction>,) {
    let mut text_lines = text_lines;
    {
        let mut ttp: Vec<TextToPrint> = Vec::new();
        for tr_act in executed_actions {
            ttp.append( &mut diagram_repr_trace_action(tr_act,gen_ctx) );
            ttp.push( TextToPrint{text:" ".to_string(),color:Rgb(HCP_Black)} );
        }
        // ***
        ttp.push( TextToPrint{text:"@p".to_string(),color:Rgb(HCP_StandardPurple)} );
        ttp.push( TextToPrint{text:position_to_text(action_position),color:Rgb(HCP_Black)} );
        text_lines.push( ttp );
    }
    // ***
    let line_lens : Vec<usize> = text_lines.iter().map(|x| TextToPrint::char_count(x) ).collect();
    let max_x_shift = *line_lens.iter().max().unwrap();
    // ***
    let img_width : f32 = 2.0*MARGIN + (max_x_shift as f32)*FONT_WIDTH/2.0;
    let img_height : f32 = MARGIN + (text_lines.len() as f32)*(MARGIN + VERTICAL_SIZE);
    // ***
    let path = Path::new( path_str );
    new_image_with_colored_text(path,&DrawnColoredTextAlignment::Center,img_width,img_height,text_lines)
}

fn write_lfs_texts(gen_ctx : &GeneralContext,
                   co_localizations : &CoLocalizations,
                   coloc_id : &usize,
                   color : Rgb<u8>,
                   ttp : &mut Vec<TextToPrint>) {
    ttp.push( TextToPrint{text:"[".to_string(),color} );
    let coloc_lfs = co_localizations.get_coloc_lfs_ids(*coloc_id);
    let mut remaining_lfs = coloc_lfs.len();
    for lf_id in coloc_lfs.iter().sorted() {
        let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
        ttp.push( TextToPrint{text:lf_name,color:Rgb(HC_Lifeline)} );
        remaining_lfs -= 1;
        if remaining_lfs > 0 {
            ttp.push( TextToPrint{text:",".to_string(),color} );
        }
    }
    ttp.push( TextToPrint{text:"]".to_string(),color} );
}

