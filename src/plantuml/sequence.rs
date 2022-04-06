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

use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};

use crate::core::syntax::interaction::{Interaction,LoopKind};
use crate::core::syntax::util::get_recursive_frag::*;
use crate::core::syntax::action::*;

use crate::core::general_context::GeneralContext;


pub fn to_plant_uml_sd(output_path : &String,
                       title : &str,
                       interaction : &Interaction,
                       gen_ctx : &GeneralContext) {
    let mut output_file = File::create(output_path).unwrap();
    output_file.write( "@startuml\n".as_bytes() );
    //output_file.write( "title-".as_bytes() );
    //output_file.write( title.as_bytes() );
    //output_file.write( "\n".as_bytes() );
    to_plant_uml_sd_rec(&mut output_file, interaction, gen_ctx);
    output_file.write( "@enduml\n".as_bytes() );
}

fn to_plant_uml_sd_rec(output_file : &mut File,
                       interaction : &Interaction,
                       gen_ctx : &GeneralContext) {
    match interaction {
        &Interaction::Empty => {},
        &Interaction::Reception(ref rc_act) => {
            let ms_name = gen_ctx.get_ms_name(rc_act.ms_id).unwrap();
            match rc_act.recipients.len() {
                1 => {
                    let lf_id = rc_act.recipients.get(0).unwrap();
                    let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
                    output_file.write( format!("->{} : {}\n", &lf_name, &ms_name).as_bytes() );
                },
                _ => {
                    panic!("translation towards puml-sd does not implement broadcasts");
                }
            }
        },
        &Interaction::Emission(ref em_act) => {
            let ms_name = gen_ctx.get_ms_name(em_act.ms_id).unwrap();
            let lf_name = gen_ctx.get_lf_name(em_act.origin_lf_id).unwrap();
            match em_act.targets.len() {
                0 => {
                    output_file.write( format!("{}-> : {}\n", &lf_name, &ms_name).as_bytes() );
                },
                1 => {
                    let target_ref = em_act.targets.get(0).unwrap();
                    match target_ref {
                        EmissionTargetRef::Gate(_) => {
                            output_file.write( format!("{}->] : {}\n", &lf_name, &ms_name).as_bytes() );
                        },
                        EmissionTargetRef::Lifeline(tar_lf_id) => {
                            let tar_lf_name = gen_ctx.get_lf_name(*tar_lf_id).unwrap();
                            output_file.write( format!("{}->{} : {}\n", &lf_name, &tar_lf_name, &ms_name).as_bytes() );
                        }
                    }
                },
                _ => {
                    panic!("translation towards puml-sd does not implement broadcasts");
                }
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            to_plant_uml_sd_rec(output_file, i1, gen_ctx);
            to_plant_uml_sd_rec(output_file, i2, gen_ctx);
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let mut strict_frags = get_recursive_strict_frags(i1);
            strict_frags.extend_from_slice(&mut get_recursive_strict_frags(i2));
            // ***
            output_file.write( "group strict\n".as_bytes() );
            let mut rem = strict_frags.len();
            for frag in strict_frags {
                to_plant_uml_sd_rec(output_file, frag, gen_ctx);
                rem = rem - 1;
                if rem > 0 {
                    output_file.write( "else\n".as_bytes() );
                }
            }
            output_file.write( "end\n".as_bytes() );
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut par_frags = get_recursive_par_frags(i1);
            par_frags.extend_from_slice(&mut get_recursive_par_frags(i2));
            // ***
            output_file.write( "group par\n".as_bytes() );
            let mut rem = par_frags.len();
            for frag in par_frags {
                to_plant_uml_sd_rec(output_file, frag, gen_ctx);
                rem = rem - 1;
                if rem > 0 {
                    output_file.write( "else\n".as_bytes() );
                }
            }
            output_file.write( "end\n".as_bytes() );
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut alt_frags = get_recursive_alt_frags(i1);
            alt_frags.extend_from_slice(&mut get_recursive_alt_frags(i2));
            // ***
            output_file.write( "alt\n".as_bytes() );
            let mut rem = alt_frags.len();
            for frag in alt_frags {
                to_plant_uml_sd_rec(output_file, frag, gen_ctx);
                rem = rem - 1;
                if rem > 0 {
                    output_file.write( "else\n".as_bytes() );
                }
            }
            output_file.write( "end\n".as_bytes() );
        },
        &Interaction::Loop(ref kind, ref i1) => {
            // ***
            match kind {
                &LoopKind::SStrictSeq => {
                    output_file.write( "group loopX\n".as_bytes() );
                },
                &LoopKind::HHeadFirstWS => {
                    output_file.write( "group loopH\n".as_bytes() );
                },
                &LoopKind::WWeakSeq => {
                    output_file.write( "group loopS\n".as_bytes() );
                },
                &LoopKind::PInterleaving => {
                    output_file.write( "group loopP\n".as_bytes() );
                }
            }
            to_plant_uml_sd_rec(output_file, i1, gen_ctx);
            output_file.write( "end\n".as_bytes() );
        },
        &Interaction::CoReg(_,_,_) => {
            panic!("translation towards puml-sd does not implement co-regions");
        },
        &Interaction::And(_,_) => {
            panic!("translation towards puml-sd does not implement ands");
        }
    }
}