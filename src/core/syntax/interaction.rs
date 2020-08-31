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
use std::collections::{HashMap,HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::core::syntax::position::*;
use crate::core::syntax::action::*;

#[derive(Clone, PartialEq, Debug)]
pub enum ScheduleOperatorKind {
    Strict,
    Seq,
    Par
}

#[derive(Clone, PartialEq, Debug)]
pub enum Interaction {
    Empty,
    Action(ObservableAction),
    Strict(Box<Interaction>,Box<Interaction>),
    Seq(Box<Interaction>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Par(Box<Interaction>,Box<Interaction>),
    Loop(ScheduleOperatorKind,Box<Interaction>)
}


impl Interaction {

    pub fn as_leaf(&self) -> &ObservableAction {
        match self {
            Interaction::Action(act) => {
                return act;
            },
            _ => {
                panic!("called as_leaf on something that's not a leaf : {:?}", self);
            }
        }
    }

    pub fn get_sub_interaction(&self, my_pos : &Position) -> &Interaction {
        match my_pos {
            Position::Epsilon => {
                return self;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
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
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }

    pub fn express_empty(&self) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            }, &Interaction::Action(_) => {
                return false;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.express_empty() || i2.express_empty();
            }, &Interaction::Loop(_, _) => {
                return true;
            }
        }
    }

    pub fn avoids(&self, lf_id : usize) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            }, &Interaction::Action(ref act) => {
                if act.occupation_after().contains(&lf_id) {
                    return false;
                } else {
                    return true;
                }
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.avoids(lf_id) || i2.avoids(lf_id);
            }, &Interaction::Loop(_, _) => {
                return true;
            }
        }
    }

    pub fn involves(&self, lf_id : usize) -> bool {
        match self {
            &Interaction::Empty => {
                return false;
            }, &Interaction::Action(ref act) => {
                if act.occupation_after().contains(&lf_id) {
                    return true;
                } else {
                    return false;
                }
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Loop(_, ref i1) => {
                return i1.involves(lf_id);
            }
        }
    }

    pub fn get_loop_depth_at_pos(&self, my_pos : &Position) -> u32 {
        match my_pos {
            Position::Epsilon => {
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

}



