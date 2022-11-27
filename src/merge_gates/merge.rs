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


use crate::core::language::syntax::interaction::{Interaction};
use crate::core::language::syntax::action::*;
use crate::core::language::position::position::Position;
use crate::core::language::syntax::util::get_recursive_frag::*;
use crate::core::language::syntax::util::fold_recursive_frags::*;
use crate::core::general_context::GeneralContext;
use crate::canonize::transformations::phases::InteractionTermTransformation;
use crate::canonize::transformations::transfokind::TransformationKind;



fn get_possible_merges_from_root(interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    let mut merges : Vec<InteractionTermTransformation> = Vec::new();
    match interaction {
        &Interaction::And(ref i1, ref i2) => {
            match (&**i1,&**i2) {
                (Interaction::Action(ref act1),Interaction::Action(ref act2)) => {
                    match (&act1.act_kind,&act2.act_kind) {
                        (&ObservableActionKind::Reception(ref rec_orig),&ObservableActionKind::Emission(ref emit_targs)) => {
                            match merge_actions((&act2.lf_id,emit_targs,&act2.ms_id), (&act1.lf_id,rec_orig,&act1.ms_id)) {
                                None => {},
                                Some( merged_action ) => {
                                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeAction,
                                                                                   Position::Epsilon,
                                                                                   Interaction::Action(merged_action)));
                                }
                            }
                        },
                        (&ObservableActionKind::Emission(ref emit_targs),&ObservableActionKind::Reception(ref rec_orig)) => {
                            match merge_actions((&act1.lf_id,emit_targs,&act1.ms_id), (&act2.lf_id,rec_orig,&act2.ms_id)) {
                                None => {},
                                Some( merged_action ) => {
                                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeAction,
                                                                                   Position::Epsilon,
                                                                                   Interaction::Action(merged_action)));
                                }
                            }
                        },
                        _ => {}
                    }
                },

                _ => {}
            }
            // ***
        _ => {}
    }
    return merges;
}




