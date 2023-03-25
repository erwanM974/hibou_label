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
use image_colored_text::draw::multi_line::MultiLineTextAlignment;
use image_colored_text::ttp::TextToPrint;
use itertools::Itertools;
use crate::core::colocalizations::CoLocalizations;

use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::position::position::Position;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::io::output::draw_commons::hibou_color_palette::*;
use crate::io::output::draw_commons::make_image_of_text::new_image_with_colored_text;
use crate::io::output::draw_commons::sd_drawing_conf::*;
use crate::io::output::draw_traces::implem::trace_action::diagram_repr_trace_action;

// **********



pub fn draw_firing_simple(path : &Path,
                   gen_ctx : &GeneralContext,
                   action_position : &Position,
                   executed_actions : &HashSet<TraceAction>) {
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    draw_firing_finalize(path,text_lines, gen_ctx, action_position, executed_actions);
}


pub fn draw_firing_analysis(path : &Path,
                            gen_ctx : &GeneralContext,
                            action_position : &Position,
                            executed_actions : &HashSet<TraceAction>,
                            co_localizations : &CoLocalizations,
                            consu_set : &HashSet<usize>,
                            sim_map : &HashMap<usize,SimulationStepKind>) {

    // ***
    let mut text_lines : Vec<Vec<TextToPrint>> = Vec::new();
    // ***
    if consu_set.len() > 0 || sim_map.len() > 0 {
        let mut ttp: Vec<TextToPrint> = Vec::new();
        for coloc_id in consu_set {
            ttp.push( TextToPrint::new("C".to_string(),Rgb(HC_Grammar_Symbol)) );
            write_lfs_texts(gen_ctx,co_localizations,coloc_id,Rgb(HC_Grammar_Symbol),&mut ttp);
            ttp.push( TextToPrint::new(" ".to_string(),Rgb(HCP_Black)) );
        }
        for (coloc_id,sim_kind) in sim_map {
            ttp.push( TextToPrint::new("S".to_string(),Rgb(HCP_LightGray)) );
            write_lfs_texts(gen_ctx,co_localizations,coloc_id,Rgb(HCP_LightGray),&mut ttp);
            match sim_kind {
                SimulationStepKind::BeforeStart => {
                    ttp.push( TextToPrint::new("↑".to_string(),Rgb(HCP_LightGray)) );
                },
                SimulationStepKind::AfterEnd => {
                    ttp.push( TextToPrint::new("↓".to_string(),Rgb(HCP_LightGray)) );
                }
            }
            // ***
            ttp.push( TextToPrint::new(" ".to_string(),Rgb(HCP_Black)) );
        }
        text_lines.push( ttp );
    }
    // ***
    draw_firing_finalize(path,text_lines, gen_ctx, action_position, executed_actions);
}


fn draw_firing_finalize(path : &Path,
                        text_lines : Vec<Vec<TextToPrint>>,
       gen_ctx : &GeneralContext,
       action_position : &Position,
       executed_actions : &HashSet<TraceAction>,) {
    let mut text_lines = text_lines;
    {
        let mut ttp: Vec<TextToPrint> = Vec::new();
        for tr_act in executed_actions {
            ttp.append( &mut diagram_repr_trace_action(tr_act,gen_ctx) );
            ttp.push( TextToPrint::new(" ".to_string(),Rgb(HCP_Black)) );
        }
        // ***
        ttp.push( TextToPrint::new("@".to_string(),Rgb(HCP_StandardPurple)) );
        ttp.push( TextToPrint::new(action_position.to_string(),Rgb(HCP_Black)) );
        text_lines.push( ttp );
    }
    // ***
    new_image_with_colored_text(path,
                                &MultiLineTextAlignment::Center,
                                &text_lines);
}

fn write_lfs_texts(gen_ctx : &GeneralContext,
                   co_localizations : &CoLocalizations,
                   coloc_id : &usize,
                   color : Rgb<u8>,
                   ttp : &mut Vec<TextToPrint>) {
    ttp.push( TextToPrint::new("[".to_string(),color) );
    let coloc_lfs = co_localizations.get_coloc_lfs_ids(*coloc_id);
    let mut remaining_lfs = coloc_lfs.len();
    for lf_id in coloc_lfs.iter().sorted() {
        let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
        ttp.push( TextToPrint::new(lf_name,Rgb(HC_Lifeline)) );
        remaining_lfs -= 1;
        if remaining_lfs > 0 {
            ttp.push( TextToPrint::new(",".to_string(),color) );
        }
    }
    ttp.push( TextToPrint::new("]".to_string(),color) );
}

