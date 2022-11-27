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



use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::syntax::util::fold_recursive_frags::{fold_recursive_par_frags, fold_recursive_seq_frags, fold_recursive_strict_frags};
use crate::core::language::syntax::util::get_recursive_frag::{get_recursive_par_frags, get_recursive_seq_frags, get_recursive_strict_frags};

pub fn loop_empty_simpl(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Loop(ref sk, ref i1) => {
            match **i1 {
                Interaction::Empty => {
                    return vec![Interaction::Empty];
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}


pub fn loop_unnest(interaction : &Interaction) -> Vec<Interaction> {
    match interaction {
        &Interaction::Loop(ref lkA, ref i1) => {
            match **i1 {
                Interaction::Loop(ref lkB, ref i11) => {
                    return vec![Interaction::Loop((lkA.min(lkB)).clone(), i11.clone())];
                },
                _ => {}
            }
        },
        _ => {}
    }
    return vec![];
}

pub fn loop_factorize(interaction : &Interaction) -> Vec<Interaction> {
    let mut got_ints = vec![];
    match interaction {
        &Interaction::Strict(ref i1, ref i2) => {
            match **i1 {
                Interaction::Loop(ref lk, ref i11) => {
                    match lk {
                        LoopKind::SStrictSeq => {
                            if i2 == i11 {
                                got_ints.push(*i1.clone());
                            } else if i2 == i1 {
                                got_ints.push(*i1.clone());
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
            match **i2 {
                Interaction::Loop(ref lk, ref i21) => {
                    match lk {
                        LoopKind::SStrictSeq => {
                            if i1 == i21 {
                                got_ints.push(*i2.clone());
                            } else if i1 == i2 {
                                got_ints.push(*i2.clone());
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        &Interaction::Seq(ref i1, ref i2) => {
            match **i1 {
                Interaction::Loop(ref lk, ref i11) => {
                    match lk {
                        LoopKind::WWeakSeq => {
                            if i2 == i11 {
                                got_ints.push(*i1.clone());
                            } else if i2 == i1 {
                                got_ints.push(*i1.clone());
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
            match **i2 {
                Interaction::Loop(ref lk, ref i21) => {
                    match lk {
                        LoopKind::WWeakSeq => {
                            if i1 == i21 {
                                got_ints.push(*i2.clone());
                            } else if i1 == i2 {
                                got_ints.push(*i2.clone());
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        &Interaction::Par(ref i1, ref i2) => {
            match **i1 {
                Interaction::Loop(ref lk, ref i11) => {
                    match lk {
                        LoopKind::PInterleaving => {
                            if i2 == i11 {
                                got_ints.push(*i1.clone());
                            } else if i2 == i1 {
                                got_ints.push(*i1.clone());
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
            match **i2 {
                Interaction::Loop(ref lk, ref i21) => {
                    match lk {
                        LoopKind::PInterleaving => {
                            if i1 == i21 {
                                got_ints.push(*i2.clone());
                            } else if i1 == i2 {
                                got_ints.push(*i2.clone());
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return got_ints;
}




