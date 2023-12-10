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



use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::action::{CommunicationSynchronicity, ReceptionAction};
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::syntax::util::fold_recursive_frags::fold_recursive_strict_frags;


fn basic_door_on_lifeline(num_possible_letters : u32,
                          length_code : u32,
                          length_after_code : u32) -> Interaction {
    let recA = Interaction::Reception(ReceptionAction::new(None,
                                                           0,
                                                           CommunicationSynchronicity::Asynchronous,
                                                           vec![0]));
    let recB = Interaction::Reception(ReceptionAction::new(None,
                                                           1,
                                                           CommunicationSynchronicity::Asynchronous,
                                                           vec![0]));
    let recC = Interaction::Reception(ReceptionAction::new(None,
                                                           2,
                                                           CommunicationSynchronicity::Asynchronous,
                                                           vec![0]));
    let altfrag = match num_possible_letters {
        2 => {
            Interaction::Alt(
                Box::new(recA.clone()),
                Box::new(recB.clone())
            )
        },
        3 => {
            Interaction::Alt(
                Box::new(recA.clone()),
                Box::new(Interaction::Alt(
                    Box::new(recB.clone()),Box::new(recC.clone())
                ))
            )
        },
        _ => {
            panic!("should not be called")
        }
    };

    let iloop = Interaction::Loop(LoopKind::SStrictSeq,Box::new(altfrag.clone()));
    let mut frags = vec![];
    frags.push(&iloop);
    for _ in 0..length_code {
        frags.push(&recA);
    }
    for _ in 0..length_after_code {
        frags.push(&altfrag);
    }
    fold_recursive_strict_frags(&mut frags)
}




pub fn generate_doors_interactions(doors_num : u32,
                        num_possible_letters : u32,
                        length_code : u32,
                        length_after_code : u32) -> Vec<Interaction> {
    match doors_num {
        0 => {
            panic!("should not be reached")
        },
        1 => {
            vec![basic_door_on_lifeline(num_possible_letters,length_code,length_after_code)]
        },
        2 => {
            let i = basic_door_on_lifeline(num_possible_letters,length_code,length_after_code);
            let i_strict = Interaction::Strict(Box::new(i.clone()),Box::new(i.clone()));
            let i_par = Interaction::Par(Box::new(i.clone()),Box::new(i));
            vec![
                i_strict,i_par
            ]
        },
        x => {
            let (mid,shift) = if x % 2 == 0 {
                (x / 2,0_u32)
            } else {
                ((x-1)/2,1_u32)
            };
            let mut ints = vec![];
            for left in 1..=mid {
                let right = x - left;
                let left_ints = generate_doors_interactions(left,num_possible_letters,length_code,length_after_code);
                let right_ints = generate_doors_interactions(right,num_possible_letters,length_code,length_after_code);
                for lint in &left_ints {
                    for rint in &right_ints {
                        let i_strict = Interaction::Strict(Box::new(lint.clone()),Box::new(rint.clone()));
                        let i_par = Interaction::Par(Box::new(lint.clone()),Box::new(rint.clone()));
                        if !ints.contains(&i_strict) {
                            ints.push(i_strict);
                        }
                        if !ints.contains(&i_par) {
                            ints.push(i_par);
                        }
                    }
                }
            }
            ints
        }
    }
}



