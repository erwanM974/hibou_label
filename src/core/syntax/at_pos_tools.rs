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
use std::hash::{Hash, Hasher};

use crate::core::syntax::action::*;
use crate::core::syntax::interaction::*;
use crate::core::syntax::position::*;
use crate::core::trace::{TraceAction, TraceActionKind};


impl Interaction {

    pub fn get_loop_depth_at_pos(&self, my_pos : &Position) -> u32 {
        match my_pos {
            Position::Epsilon(_) => {
                return 0;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Alt(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Seq(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Loop(_, ref i1) => {
                        return 1 + i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    _ => {
                        panic!("undefined pos");
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Alt(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Seq(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    _ => {
                        panic!("undefined pos");
                    }
                }
            }
        }
    }


    pub fn get_outermost_loop_content(&self, my_pos : &Position) -> Option<(Interaction,Position)> {
        match my_pos {
            Position::Epsilon(_) => {
                return None;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Loop(_ , ref i1) => {
                        return Some( ( *(i1.clone()) , *(sub_pos.clone()) ) );
                    },
                    _ => {
                        panic!();
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }

    pub fn get_sub_interaction(&self, my_pos : &Position) -> &Interaction {
        match my_pos {
            Position::Epsilon(_) => {
                return self;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, _) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, _) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, _) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, _) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, _) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Loop(_ , ref i1) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Seq(_, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, _, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(_, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(_, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(_, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }
}