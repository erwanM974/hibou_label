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


use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::syntax::util::get_recursive_frag::*;
use crate::core::syntax::util::fold_recursive_frags::*;
use crate::core::general_context::GeneralContext;
use crate::canonize::transformations::phases::InteractionTermTransformation;
use crate::canonize::transformations::transfokind::TransformationKind;

pub fn get_all_merges_rec(interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    let mut results = get_possible_merges_from_root(interaction);
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            for left_transfo in get_all_merges_rec(i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                 Position::Left(Box::new(left_transfo.position)),
                                                                 Interaction::Strict(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_merges_rec(i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                 Position::Right(Box::new(right_transfo.position)),
                                                                 Interaction::Strict(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            for left_transfo in get_all_merges_rec(i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                 Position::Left(Box::new(left_transfo.position)),
                                                                 Interaction::Seq(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_merges_rec(i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                 Position::Right(Box::new(right_transfo.position)),
                                                                 Interaction::Seq(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },
        &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            for left_transfo in get_all_merges_rec(i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                 Position::Left(Box::new(left_transfo.position)),
                                                                 Interaction::CoReg(cr.clone(), Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_merges_rec(i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                 Position::Right(Box::new(right_transfo.position)),
                                                                 Interaction::CoReg(cr.clone(), i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            for left_transfo in get_all_merges_rec(i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                 Position::Left(Box::new(left_transfo.position)),
                                                                 Interaction::Par(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_merges_rec(i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                 Position::Right(Box::new(right_transfo.position)),
                                                                 Interaction::Par(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },
        &Interaction::Alt(ref i1, ref i2) => {
            for left_transfo in get_all_merges_rec(i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                 Position::Left(Box::new(left_transfo.position)),
                                                                 Interaction::Alt(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_merges_rec(i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                 Position::Right(Box::new(right_transfo.position)),
                                                                 Interaction::Alt(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },
        &Interaction::Loop(ref lk, ref i1) => {
            for left_transfo in get_all_merges_rec(i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                 Position::Left(Box::new(left_transfo.position)),
                                                                 Interaction::Loop(lk.clone(), Box::new(left_transfo.result))
                ) );
            }
        },
        /*&Interaction::And(ref i1, ref i2) => {
            for left_transfo in get_possible_merges_from_root(i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                 Position::Left(Box::new(left_transfo.position)),
                                                                 Interaction::And(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_possible_merges_from_root(i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                 Position::Right(Box::new(right_transfo.position)),
                                                                 Interaction::And(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        },*/
        _ => {}
    }
    return results;
}

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
                (Interaction::Strict(ref i11, ref i12),Interaction::Strict(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    let new_int = Interaction::Strict(Box::new(new_left), Box::new(new_right) );
                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeSkip,
                                                                   Position::Epsilon,
                                                                   new_int));
                },
                (Interaction::Seq(ref i11, ref i12),Interaction::Seq(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    let new_int = Interaction::Seq(Box::new(new_left), Box::new(new_right) );
                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeSkip,
                                                                   Position::Epsilon,
                                                                   new_int));
                },
                (Interaction::Par(ref i11, ref i12),Interaction::Par(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    let new_int = Interaction::Par(Box::new(new_left), Box::new(new_right) );
                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeSkip,
                                                                   Position::Epsilon,
                                                                   new_int));
                },
                (Interaction::Alt(ref i11, ref i12),Interaction::Alt(ref i21, ref i22)) => {
                    let new_left = Interaction::And( i11.clone(), i21.clone() );
                    let new_right = Interaction::And( i12.clone(), i22.clone() );
                    let new_int = Interaction::Alt(Box::new(new_left), Box::new(new_right) );
                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeSkip,
                                                                   Position::Epsilon,
                                                                   new_int));
                },
                _ => {}
            }
            // ***
            match **i1 {
                Interaction::Strict(ref i11, ref i12) => {
                    {
                        let new_left = Interaction::And( i11.clone(), i2.clone() );
                        let new_int = Interaction::Strict(Box::new(new_left), i12.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i12.clone(), i2.clone() );
                        let new_int = Interaction::Strict( i11.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Seq(ref i11, ref i12) => {
                    {
                        let new_left = Interaction::And( i11.clone(), i2.clone() );
                        let new_int = Interaction::Seq(Box::new(new_left), i12.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i12.clone(), i2.clone() );
                        let new_int = Interaction::Seq( i11.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Par(ref i11, ref i12) => {
                    {
                        let new_left = Interaction::And( i11.clone(), i2.clone() );
                        let new_int = Interaction::Par(Box::new(new_left), i12.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i12.clone(), i2.clone() );
                        let new_int = Interaction::Par( i11.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Alt(ref i11, ref i12) => {
                    {
                        let new_left = Interaction::And( i11.clone(), i2.clone() );
                        let new_int = Interaction::Alt(Box::new(new_left), i12.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i12.clone(), i2.clone() );
                        let new_int = Interaction::Alt( i11.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight1,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Loop(ref lk, ref i11) => {
                    let new_sub = Interaction::And( i11.clone(), i2.clone() );
                    let new_int = Interaction::Loop(lk.clone(), Box::new(new_sub) );
                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft1,
                                                                   Position::Epsilon,
                                                                   new_int));
                },
                _ => {}
            }
            // ***
            match **i2 {
                Interaction::Strict(ref i21, ref i22) => {
                    {
                        let new_left = Interaction::And( i1.clone(), i21.clone() );
                        let new_int = Interaction::Strict(Box::new(new_left), i22.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i1.clone(), i22.clone() );
                        let new_int = Interaction::Strict( i21.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Seq(ref i21, ref i22) => {
                    {
                        let new_left = Interaction::And( i1.clone(), i21.clone() );
                        let new_int = Interaction::Seq(Box::new(new_left), i22.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i1.clone(), i22.clone() );
                        let new_int = Interaction::Seq( i21.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Par(ref i21, ref i22) => {
                    {
                        let new_left = Interaction::And( i1.clone(), i21.clone() );
                        let new_int = Interaction::Par(Box::new(new_left), i22.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i1.clone(), i22.clone() );
                        let new_int = Interaction::Par( i21.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Alt(ref i21, ref i22) => {
                    {
                        let new_left = Interaction::And( i1.clone(), i21.clone() );
                        let new_int = Interaction::Alt(Box::new(new_left), i22.clone() );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                    {
                        let new_right = Interaction::And( i1.clone(), i22.clone() );
                        let new_int = Interaction::Alt( i21.clone(), Box::new(new_right) );
                        merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndRight2,
                                                                       Position::Epsilon,
                                                                       new_int));
                    }
                },
                Interaction::Loop(ref lk, ref i21) => {
                    let new_sub = Interaction::And( i1.clone(), i21.clone() );
                    let new_int = Interaction::Loop(lk.clone(), Box::new(new_sub) );
                    merges.push(InteractionTermTransformation::new(TransformationKind::MergeAndLeft2,
                                                                   Position::Epsilon,
                                                                   new_int));
                },
                _ => {}
            }
        },
        _ => {}
    }
    return merges;
}


fn merge_actions(emission : (&usize,&Vec<EmissionTargetRef>,&usize), reception : (&usize,&Option<usize>,&usize)) -> Option<ObservableAction> {
    let (src_lf_id,target_refs,src_ms_id) = emission;
    let (tar_lf_id,tar_orig,tar_ms_id) = reception;
    if src_ms_id == tar_ms_id {
        match tar_orig {
            None => {},
            Some( tar_gt_id_orig ) => {
                let mut new_targets : Vec<EmissionTargetRef> = Vec::new();
                let mut got_it = false;
                for targ_ref in target_refs {
                    match targ_ref {
                        EmissionTargetRef::Lifeline(ref other_tar_lf_id) => {
                            new_targets.push( EmissionTargetRef::Lifeline(*other_tar_lf_id) );
                        },
                        EmissionTargetRef::Gate(ref gate_id) => {
                            if gate_id == tar_gt_id_orig && !got_it {
                                new_targets.push( EmissionTargetRef::Lifeline(*tar_lf_id) );
                                got_it = true;
                            } else {
                                new_targets.push( EmissionTargetRef::Gate(*gate_id) );
                            }
                        }
                    }
                }
                if got_it {
                    return Some( ObservableAction{lf_id:*src_lf_id,act_kind:ObservableActionKind::Emission(new_targets),ms_id:*src_ms_id} );
                }
            }
        }
    }
    return None;
}


