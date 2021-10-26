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
use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};

use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::util::get_recursive_frag::*;
use crate::core::syntax::action::*;

use crate::core::general_context::GeneralContext;


pub fn to_plant_uml_ap(output_path : &String,
                       title : &str,
                       interaction : &Interaction,
                       gen_ctx : &GeneralContext) {
    let mut output_file = File::create(output_path).unwrap();
    output_file.write( "@startuml\n".as_bytes() );
    //TODO: for each lifeline make an automaton
    // to do so hide all other lifelines, canonize the remainder and inductively build the automaton
    for lf_id in 0..gen_ctx.get_lf_num() {
        let lf_name = gen_ctx.get_lf_name(lf_id).unwrap();
        let mut lfs_to_remove : HashSet<usize> = (0..(gen_ctx.get_lf_num())).collect();
        lfs_to_remove.remove(&lf_id);
        let projected_int = interaction.hide(&lfs_to_remove);
        output_file.write( format!("state lf_{} {{\n",lf_name).as_bytes() );
        let (last_state_id,_) = to_plant_uml_ap_rec(&mut output_file, &projected_int, gen_ctx, lf_id, 0, 1);
        output_file.write( format!("lf{}_s{} --> [*]\n",lf_id,last_state_id).as_bytes() );
        output_file.write( "}\n".as_bytes() );
    }
    output_file.write( "@enduml\n".as_bytes() );
}

fn to_plant_uml_ap_rec(output_file : &mut File,
                       interaction : &Interaction,
                       gen_ctx : &GeneralContext,
                       lf_id : usize,
                       last_state : u32,
                       state_count : u32) -> (u32,u32) {

    match interaction {
        &Interaction::Empty => {
            return (last_state,state_count);
        },
        &Interaction::Action(ref act) => {
            let ms_name = gen_ctx.get_ms_name(act.ms_id).unwrap();
            let transition_label : String;
            match act.act_kind {
                ObservableActionKind::Reception(_) => {
                    transition_label = format!("?{}",ms_name);
                },
                ObservableActionKind::Emission(ref targets) => {
                    assert!(targets.len() == 0);
                    transition_label = format!("!{}",ms_name);
                }
            }
            let new_state_label = format!("lf{}_s{}",lf_id,state_count);
            let last_state_label = get_last_label(lf_id, last_state);
            output_file.write( format!("{} --> {} : {}\n",last_state_label,new_state_label,transition_label).as_bytes() );
            return (state_count,state_count + 1);
        },
        &Interaction::Seq(ref i1, ref i2) => {
            let (last,count) = to_plant_uml_ap_rec(output_file, i1, gen_ctx, lf_id, last_state,state_count);
            return to_plant_uml_ap_rec(output_file, i2, gen_ctx, lf_id, last,count);
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let (last,count) = to_plant_uml_ap_rec(output_file, i1, gen_ctx, lf_id, last_state,state_count);
            return to_plant_uml_ap_rec(output_file, i2, gen_ctx, lf_id, last,count);
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut new_state_count = state_count;
            // ***
            let fork_id = new_state_count;
            new_state_count = new_state_count +1;
            let fork_label = format!("lf{}_s{}", lf_id, fork_id);
            let last_state_label = get_last_label(lf_id, last_state);
            // ***
            output_file.write(format!("state {} <<fork>>\n", fork_label).as_bytes());
            output_file.write(format!("{} --> {}\n", last_state_label, fork_label).as_bytes());
            // ***
            let mut par_frags = get_recursive_par_frags(i1);
            par_frags.extend_from_slice(&mut get_recursive_par_frags(i2));
            // ***
            let mut last_on_branches : Vec<u32> = Vec::new();
            for frag in par_frags {
                let (last,count) = to_plant_uml_ap_rec(output_file, frag, gen_ctx, lf_id, fork_id, new_state_count);
                new_state_count = count;
                last_on_branches.push( last );
            }
            // ***
            let join_id = new_state_count;
            new_state_count = new_state_count +1;
            let join_label = format!("lf{}_s{}", lf_id,join_id);
            // ***
            output_file.write(format!("state {} <<join>>\n",join_label).as_bytes());
            for last_in_frag in last_on_branches {
                output_file.write(format!("lf{}_s{} --> {}\n",lf_id,last_in_frag, join_label).as_bytes());
            }
            return (join_id,new_state_count);
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut new_state_count = state_count;
            // ***
            let fork_id = new_state_count;
            new_state_count = new_state_count +1;
            let fork_label = format!("lf{}_s{}", lf_id, fork_id);
            let last_state_label = get_last_label(lf_id, last_state);
            // ***
            output_file.write(format!("state {} <<choice>>\n", fork_label).as_bytes());
            output_file.write(format!("{} --> {}\n", last_state_label, fork_label).as_bytes());
            // ***
            let mut alt_frags = get_recursive_alt_frags(i1);
            alt_frags.extend_from_slice(&mut get_recursive_alt_frags(i2));
            // ***
            let mut last_on_branches : Vec<u32> = Vec::new();
            for frag in alt_frags {
                let (last,count) = to_plant_uml_ap_rec(output_file, frag, gen_ctx, lf_id, fork_id, new_state_count);
                new_state_count = count;
                last_on_branches.push( last );
            }
            // ***
            let join_id = new_state_count;
            new_state_count = new_state_count +1;
            let join_label = format!("lf{}_s{}", lf_id,join_id);
            // ***
            output_file.write(format!("state {} <<choice>>\n",join_label).as_bytes());
            for last_in_frag in last_on_branches {
                output_file.write(format!("lf{}_s{} --> {}\n",lf_id,last_in_frag, join_label).as_bytes());
            }
            return (join_id,new_state_count);
        },
        &Interaction::Loop(ref kind, ref i1) => {
            let mut new_state_count = state_count;
            // ***
            let loop_id = new_state_count;
            new_state_count = new_state_count +1;
            let loop_label = format!("lf{}_s{}", lf_id, loop_id);
            let last_state_label = get_last_label(lf_id, last_state);
            // ***
            output_file.write(format!("state {} {{\n", loop_label).as_bytes());
            // ***
            let (last,count) =  to_plant_uml_ap_rec(output_file, i1, gen_ctx, lf_id, 0, new_state_count);
            new_state_count = count;
            output_file.write( format!("lf{}_s{} --> [*]\n",lf_id,last).as_bytes() );
            // ***
            output_file.write( "}\n".as_bytes() );
            output_file.write(format!("{} --> {}\n", last_state_label, loop_label).as_bytes());
            output_file.write(format!("{} --> {}\n", loop_label, loop_label).as_bytes());
            return (loop_id,new_state_count);
        },
        &Interaction::CoReg(_,_,_) => {
            panic!("translation towards puml-ap does not implement co-regions");
        },
        &Interaction::And(_,_) => {
            panic!("translation towards puml-ap does not implement ands");
        }
    }
}

fn get_last_label(lf_id : usize, state_id : u32) -> String {
    if state_id == 0 {
        return "[*]".to_string();
    } else {
        return format!("lf{}_s{}", lf_id, state_id);
    }
}