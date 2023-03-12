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


use std::cmp::Ordering;
use crate::core::language::syntax::interaction::Interaction;

impl Ord for Interaction {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self,other) {
            (Interaction::Empty,Interaction::Empty) => {
                return Ordering::Equal;
            },
            (Interaction::Empty,_) => {
                return Ordering::Less;
            },
            (_,Interaction::Empty) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::Emission(em1),Interaction::Emission(em2)) => {
                return em1.cmp(em2);
            },
            (Interaction::Emission(_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Emission(_)) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::Reception(rc1),Interaction::Reception(rc2)) => {
                return rc1.cmp(rc2);
            },
            (Interaction::Reception(_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Reception(_)) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::Par(self_i1,self_i2),Interaction::Par(other_i1,other_i2)) => {
                let cmp_left = self_i1.cmp(other_i1);
                match &cmp_left {
                    Ordering::Equal => {
                        return self_i2.cmp(other_i2);
                    },
                    _ => {
                        return cmp_left;
                    }
                }
            },
            (Interaction::Par(_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Par(_,_)) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::CoReg(self_cr,self_i1,self_i2),Interaction::CoReg(other_cr, other_i1,other_i2)) => {
                let max_cr_len = self_cr.len().max(other_cr.len());
                for i in 0..max_cr_len {
                    match (self_cr.get(i) ,other_cr.get(i) ) {
                        ( Some( cr_ref1 ), Some(cr_ref2) ) => {
                            if cr_ref1 < cr_ref2 {
                                return Ordering::Less;
                            }
                            if cr_ref1 > cr_ref2 {
                                return Ordering::Greater;
                            }
                        },
                        (None,Some(_)) => {
                            return Ordering::Less;
                        },
                        (Some(_),None) => {
                            return Ordering::Greater;
                        },
                        (None,None) => {}
                    }
                }
                // ***
                let cmp_left = self_i1.cmp(other_i1);
                match &cmp_left {
                    Ordering::Equal => {
                        return self_i2.cmp(other_i2);
                    },
                    _ => {
                        return cmp_left;
                    }
                }
            },
            (Interaction::CoReg(_,_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::CoReg(_,_,_)) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::Seq(self_i1,self_i2),Interaction::Seq(other_i1,other_i2)) => {
                let cmp_left = self_i1.cmp(other_i1);
                match &cmp_left {
                    Ordering::Equal => {
                        return self_i2.cmp(other_i2);
                    },
                    _ => {
                        return cmp_left;
                    }
                }
            },
            (Interaction::Seq(_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Seq(_,_)) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::Strict(self_i1,self_i2),Interaction::Strict(other_i1,other_i2)) => {
                let cmp_left = self_i1.cmp(other_i1);
                match &cmp_left {
                    Ordering::Equal => {
                        return self_i2.cmp(other_i2);
                    },
                    _ => {
                        return cmp_left;
                    }
                }
            },
            (Interaction::Strict(_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Strict(_,_)) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::Alt(self_i1,self_i2),Interaction::Alt(other_i1,other_i2)) => {
                let cmp_left = self_i1.cmp(other_i1);
                match &cmp_left {
                    Ordering::Equal => {
                        return self_i2.cmp(other_i2);
                    },
                    _ => {
                        return cmp_left;
                    }
                }
            },
            (Interaction::Alt(_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Alt(_,_)) => {
                return Ordering::Greater;
            },
            // ***
            (Interaction::Loop(self_lk,self_i1),Interaction::Loop(other_lk,other_i1)) => {
                let cmp_lk = self_lk.cmp(&other_lk);
                match &cmp_lk {
                    Ordering::Equal => {
                        return self_i1.cmp(other_i1);
                    },
                    _ => {
                        return cmp_lk;
                    }
                }
            },
            (Interaction::Loop(_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Loop(_,_)) => {
                return Ordering::Greater;
            },
            (Interaction::Sync(self_acts,self_i1,self_i2),Interaction::Sync(other_acts,other_i1,other_i2)) => {
                let max_acts_len = self_acts.len().max(other_acts.len());
                for i in 0..max_acts_len {
                    match (self_acts.get(i) ,other_acts.get(i) ) {
                        ( Some( cr_ref1 ), Some(cr_ref2) ) => {
                            if cr_ref1 < cr_ref2 {
                                return Ordering::Less;
                            }
                            if cr_ref1 > cr_ref2 {
                                return Ordering::Greater;
                            }
                        },
                        (None,Some(_)) => {
                            return Ordering::Less;
                        },
                        (Some(_),None) => {
                            return Ordering::Greater;
                        },
                        (None,None) => {}
                    }
                }
                // ***
                let cmp_left = self_i1.cmp(other_i1);
                match &cmp_left {
                    Ordering::Equal => {
                        return self_i2.cmp(other_i2);
                    },
                    _ => {
                        return cmp_left;
                    }
                }
            },
            (Interaction::Sync(_,_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::Sync(_,_,_)) => {
                return Ordering::Greater;
            },
            (Interaction::And(self_i1,self_i2),Interaction::And(other_i1,other_i2)) => {
                let cmp_left = self_i1.cmp(other_i1);
                match &cmp_left {
                    Ordering::Equal => {
                        return self_i2.cmp(other_i2);
                    },
                    _ => {
                        return cmp_left;
                    }
                }
            },
            (Interaction::And(_,_),_) => {
                return Ordering::Less;
            },
            (_,Interaction::And(_,_)) => {
                return Ordering::Greater;
            }
        }
    }
}