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
use std::collections::{BTreeSet, HashMap};

use image::{Rgb, RgbImage};
use image_colored_text::draw::single_line::{draw_line_of_colored_text, DrawCoord};
use image_colored_text::ttp::TextToPrint;
use imageproc::drawing::draw_line_segment_mut;


use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::syntax::util::get_recursive_frag::{get_recursive_strict_frags, get_recursive_par_frags, get_recursive_alt_frags, get_recursive_coreg_frags, get_recursive_sync_frags};
use crate::io::output::draw_commons::font::{get_hibou_font, HIBOU_FONT_SCALE};
use crate::io::output::draw_commons::hibou_color_palette::HCP_Black;
use crate::io::output::draw_commons::sd_drawing_conf::*;
use crate::io::output::draw_interactions::as_sd::action_repr::emission::draw_emission;
use crate::io::output::draw_interactions::as_sd::action_repr::reception::draw_reception;
use crate::io::output::draw_interactions::as_sd::util::dimensions_tools::get_y_pos_from_yshift;
use crate::io::output::draw_interactions::as_sd::util::lf_coords::DrawingLifelineCoords;
use crate::io::output::draw_traces::implem::trace_action::diagram_repr_trace_actions;
use crate::io::textual_convention::*;


// **********

pub fn draw_interaction_rec(    image : &mut RgbImage,
                                gen_ctx : &GeneralContext,
                                interaction : &Interaction,
                                lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                lf_num : usize,
                                nest_shift : &mut u32,
                                yshift : &mut u32)
                        -> [usize;2] { // returns left and right borders of the interaction
    match interaction {
        &Interaction::Empty => {
            return [lf_num,0]; // because when going up we keep the minimum on the left and maximum on the right
        },
        &Interaction::Emission(ref em_act) => {
            let lr_bounds = draw_emission(image,gen_ctx,em_act,lf_x_widths,*yshift);
            *yshift = *yshift + 3;
            return lr_bounds;
        },
        &Interaction::Reception(ref rc_act) => {
            let lr_bounds = draw_reception(image,gen_ctx,rc_act,lf_x_widths,*yshift);
            *yshift = *yshift + 3;
            return lr_bounds;
        },
        &Interaction::Seq(ref i1,ref i2) => {
            let wr1 : [usize;2] = draw_interaction_rec(image, gen_ctx,i1, lf_x_widths,  lf_num,nest_shift, yshift);
            *yshift = *yshift +1;
            let wr2 : [usize;2] = draw_interaction_rec(image,  gen_ctx,i2, lf_x_widths,  lf_num,nest_shift, yshift);
            return [ std::cmp::min(wr1[0],wr2[0]) , std::cmp::max(wr1[1],wr2[1]) ];
        },
        &Interaction::Strict(ref i1,ref i2) => {
            let mut frags = get_recursive_strict_frags(i1);
            frags.extend( get_recursive_strict_frags(i2) );
            let label = vec![TextToPrint::new(SYNTAX_STRICT.to_string(),Rgb(HCP_Black))];
            return draw_n_ary_combined_fragment(image, gen_ctx,frags,lf_x_widths, lf_num,label, nest_shift, yshift);
        },
        &Interaction::CoReg(ref cr, ref i1,ref i2) => {
            let mut frags = get_recursive_coreg_frags(cr, i1);
            frags.extend( get_recursive_coreg_frags(cr,i2) );
            return draw_n_ary_coregion(image, gen_ctx,frags,cr,lf_x_widths, lf_num, nest_shift, yshift);
        },
        &Interaction::Alt(ref i1,ref i2) => {
            let mut frags = get_recursive_alt_frags(i1);
            frags.extend( get_recursive_alt_frags(i2) );
            let label = vec![TextToPrint::new(SYNTAX_ALT.to_string(),Rgb(HCP_Black))];
            return draw_n_ary_combined_fragment(image, gen_ctx,frags,lf_x_widths, lf_num,label, nest_shift, yshift);
        },
        &Interaction::Par(ref i1,ref i2) => {
            let mut frags = get_recursive_par_frags(i1);
            frags.extend( get_recursive_par_frags(i2) );
            let label = vec![TextToPrint::new(SYNTAX_PAR.to_string(),Rgb(HCP_Black))];
            return draw_n_ary_combined_fragment(image, gen_ctx,frags,lf_x_widths, lf_num,label, nest_shift, yshift);
        },
        &Interaction::Sync(ref sync_acts, ref i1,ref i2) => {
            let mut frags = get_recursive_sync_frags(sync_acts,i1);
            frags.extend( get_recursive_sync_frags(sync_acts,i2) );
            let mut label = vec![TextToPrint::new(SYNTAX_SYNC.to_string(),Rgb(HCP_Black))];
            let sync_acts_as_set = BTreeSet::from_iter(sync_acts.iter().cloned());
            label.append(&mut diagram_repr_trace_actions(&sync_acts_as_set,gen_ctx,true));
            return draw_n_ary_combined_fragment(image, gen_ctx,frags,lf_x_widths, lf_num,label, nest_shift, yshift);
        },
        &Interaction::Loop(ref lkind, ref i1) => {
            match lkind {
                LoopKind::SStrictSeq => {
                    let label = vec![TextToPrint::new(SYNTAX_LOOP_S.to_string(),Rgb(HCP_Black))];
                    return draw_unary_combined_fragment(image,  gen_ctx,i1,lf_x_widths, lf_num,label, nest_shift, yshift);
                },
                LoopKind::HHeadFirstWS => {
                    let label = vec![TextToPrint::new(SYNTAX_LOOP_H.to_string(),Rgb(HCP_Black))];
                    return draw_unary_combined_fragment(image,  gen_ctx,i1,lf_x_widths, lf_num,label, nest_shift, yshift);
                },
                LoopKind::WWeakSeq => {
                    let label = vec![TextToPrint::new(SYNTAX_LOOP_W.to_string(),Rgb(HCP_Black))];
                    return draw_unary_combined_fragment(image,  gen_ctx,i1,lf_x_widths, lf_num,label, nest_shift, yshift);
                },
                LoopKind::PInterleaving => {
                    let label = vec![TextToPrint::new(SYNTAX_LOOP_P.to_string(),Rgb(HCP_Black))];
                    return draw_unary_combined_fragment(image,  gen_ctx,i1,lf_x_widths, lf_num,label, nest_shift, yshift);
                }
            }
        },
        _ => {
            panic!("non-conform interaction");
        }
    }
}

fn draw_unary_combined_fragment(    image : &mut RgbImage,
                                    gen_ctx : &GeneralContext,
                                    i1 : &Interaction,
                                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                    lf_num : usize,
                                    label : Vec<TextToPrint>,
                                    nest_shift : &mut u32,
                                    yshift : &mut u32) -> [usize;2] {
    // draw content and gather data
    *nest_shift += 1;
    let start_y : u32 = *yshift;
    *yshift += 3;
    let lr_bounds : [usize;2] = draw_interaction_rec(image,  gen_ctx,i1, lf_x_widths,  lf_num,nest_shift, yshift);
    *yshift += 1;
    let end_y : u32 = *yshift;
    *nest_shift -= 1;
    // draw frame
    let mut y_drafts : Vec<u32> = [start_y,end_y].to_vec();
    draw_combined_fragment_frame(image, label, *nest_shift,lf_x_widths,lr_bounds[0],lr_bounds[1],y_drafts);
    return lr_bounds;
}

fn draw_n_ary_combined_fragment(  image : &mut RgbImage,
                                  gen_ctx : &GeneralContext,
                                  sub_ints : Vec<&Interaction>,
                                  lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                  lf_num : usize,
                                  label : Vec<TextToPrint>,
                                  nest_shift : &mut u32,
                                  yshift : &mut u32) -> [usize;2] {
    let mut y_drafts : Vec<u32> = Vec::new();
    // draw content and gather data
    *nest_shift += 1;
    y_drafts.push(*yshift);
    *yshift += 2;
    //
    let mut min_lf_id : usize = gen_ctx.get_lf_num();
    let mut max_lf_id : usize = 0;
    for my_int in sub_ints {
        *yshift += 1;
        let lr_bounds = draw_interaction_rec(image,  gen_ctx,my_int, lf_x_widths,  lf_num,nest_shift, yshift);
        min_lf_id = cmp::min( min_lf_id, lr_bounds[0]);
        max_lf_id = cmp::max( max_lf_id, lr_bounds[1]);
        *yshift += 1;
        y_drafts.push(*yshift);
    }
    *nest_shift -= 1;
    //
    let lr_bounds: [usize;2] = [ min_lf_id, max_lf_id ];
    // draw frame
    draw_combined_fragment_frame(image,label,*nest_shift,lf_x_widths,lr_bounds[0],lr_bounds[1],y_drafts);
    return lr_bounds;
}

fn draw_n_ary_coregion(  image : &mut RgbImage,
                                  gen_ctx : &GeneralContext,
                                  sub_ints : Vec<&Interaction>,
                                  coreg_ids : &Vec<usize>,
                                  lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                  lf_num : usize,
                                  nest_shift : &mut u32,
                                  yshift : &mut u32) -> [usize;2] {
    let mut y_drafts : Vec<u32> = Vec::new();
    // draw content and gather data
    *nest_shift += 1;
    y_drafts.push(*yshift);
    //*yshift += 2;
    //
    let mut min_lf_id : usize = gen_ctx.get_lf_num();
    let mut max_lf_id : usize = 0;
    for my_int in sub_ints {
        *yshift += 1;
        let lr_bounds = draw_interaction_rec(image,  gen_ctx,my_int, lf_x_widths,  lf_num,nest_shift, yshift);
        min_lf_id = cmp::min( min_lf_id, lr_bounds[0]);
        max_lf_id = cmp::max( max_lf_id, lr_bounds[1]);
        *yshift += 1;
        y_drafts.push(*yshift);
    }
    *nest_shift -= 1;
    //
    let lr_bounds: [usize;2] = [ min_lf_id, max_lf_id ];
    // draw frame
    draw_coregion_frame(image,*nest_shift,lf_x_widths,coreg_ids,y_drafts);
    return lr_bounds;
}

fn draw_combined_fragment_frame(    image : &mut RgbImage,
                                    label : Vec<TextToPrint>,
                                    nest_shift : u32,
                                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                    left_bound : usize,
                                    right_bound : usize,
                                    y_drafts : Vec<u32>) {
    match (lf_x_widths.get(&left_bound), lf_x_widths.get(&right_bound)) {
        (Some(left_lf_coords),Some(right_lf_coords)) => {
            let x_left : f32 = left_lf_coords.x_start + (nest_shift as f32)*FRAGMENT_PADDING;
            let x_right : f32 = (right_lf_coords.x_start + right_lf_coords.x_span_outer) - (nest_shift as f32)*FRAGMENT_PADDING;

            let mut y_coords : Vec<f32> = y_drafts.into_iter().map(|y| get_y_pos_from_yshift(y) ).collect::< Vec<f32> >();
            let y_start : f32 = y_coords.remove(0);
            let y_end : f32 = y_coords.pop().unwrap();// - (nest_shift as f32)*FRAGMENT_PADDING;
            draw_line_segment_mut(image,
                                  (x_left, y_start),
                                  (x_left, y_end),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_right, y_start),
                                  (x_right, y_end),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_left, y_start),
                                  (x_right, y_start),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_left, y_end),
                                  (x_right, y_end),
                                  Rgb(HCP_Black));
            for y_coord in y_coords {
                draw_line_segment_mut(image,
                                      (x_left, y_coord),
                                      (x_right, y_coord),
                                      Rgb(HCP_Black));
            }
            draw_line_of_colored_text(image,
                                      &DrawCoord::StartingAt(x_left + FRAGMENT_TITLE_MARGIN),
                                      &DrawCoord::CenteredAround(y_start + VERTICAL_SIZE+ FRAGMENT_TITLE_MARGIN),
                                      &label,
                                      &get_hibou_font(),
                                      &HIBOU_FONT_SCALE);
        },
        _ => {}
    }
}


fn draw_coregion_frame(    image : &mut RgbImage,
                                    nest_shift : u32,
                                    lf_x_widths : &HashMap<usize,DrawingLifelineCoords>,
                                    coreg_ids : &Vec<usize>,
                                    y_drafts : Vec<u32>) {

    let mut x_coords : Vec<&DrawingLifelineCoords> = Vec::new();
    {
        for lf_id in coreg_ids {
            match lf_x_widths.get(lf_id) {
                None => {},
                Some( xcord ) => {
                    x_coords.push(xcord);
                }
            }
        }
    }

    let mut y_coords : Vec<f32> = y_drafts.into_iter().map(|y| get_y_pos_from_yshift(y) ).collect::< Vec<f32> >();
    let y_start : f32 = y_coords.remove(0);
    // ***
    let y_end : f32 = y_coords.pop().unwrap();
    for lf_coord in x_coords {
        let x_left = lf_coord.x_middle - lf_coord.x_span_outer/2.0 + (nest_shift as f32)*FRAGMENT_PADDING;
        let x_right = lf_coord.x_middle + lf_coord.x_span_outer/2.0 - (nest_shift as f32)*FRAGMENT_PADDING;
        // ***
        draw_line_segment_mut(image,
                              (x_left, y_start),
                              (x_right, y_start),
                              Rgb(HCP_Black));
        draw_line_segment_mut(image,
                              (x_left, y_start),
                              (x_left, y_start + VERTICAL_SIZE/2.0),
                              Rgb(HCP_Black));
        draw_line_segment_mut(image,
                              (x_right, y_start),
                              (x_right, y_start + VERTICAL_SIZE/2.0),
                              Rgb(HCP_Black));
        // ***
        draw_line_segment_mut(image,
                              (x_left, y_end),
                              (x_right, y_end),
                              Rgb(HCP_Black));
        draw_line_segment_mut(image,
                              (x_left, y_end),
                              (x_left, y_end - VERTICAL_SIZE/2.0),
                              Rgb(HCP_Black));
        draw_line_segment_mut(image,
                              (x_right, y_end),
                              (x_right, y_end - VERTICAL_SIZE/2.0),
                              Rgb(HCP_Black));
        // ***
        for y_coord in &y_coords {
            draw_line_segment_mut(image,
                                  (x_left, *y_coord),
                                  (x_right, *y_coord),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_left, *y_coord + VERTICAL_SIZE/4.0),
                                  (x_left, *y_coord - VERTICAL_SIZE/4.0),
                                  Rgb(HCP_Black));
            draw_line_segment_mut(image,
                                  (x_right, *y_coord + VERTICAL_SIZE/4.0),
                                  (x_right, *y_coord - VERTICAL_SIZE/4.0),
                                  Rgb(HCP_Black));
        }
    }
    // ***
    /*
    let font = FontCollection::from_bytes(HIBOU_GRAPHIC_FONT).unwrap().into_font().unwrap();
    let scale = Scale { x: FONT_WIDTH, y: FONT_HEIGHT };
    draw_colored_text(image,&label,x_left+FRAGMENT_TITLE_MARGIN,y_start + VERTICAL_SIZE);
    */
}






