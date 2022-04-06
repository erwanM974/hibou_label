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

use crate::core::syntax::interaction::*;
use crate::core::semantics::frontier::*;


pub fn prune(my_int : &Interaction, lf_ids : &HashSet<usize>) -> Interaction {
    match my_int {
        Interaction::Empty => {
            return Interaction::Empty;
        },
        Interaction::Emission(_) => {
            return my_int.clone();
        },
        Interaction::Reception(_) => {
            return my_int.clone();
        },
        Interaction::Seq(i1, i2) => {
            let pruned_i1 = prune(i1,lf_ids);
            let pruned_i2 = prune(i2,lf_ids);
            if pruned_i1 == Interaction::Empty {
                return pruned_i2;
            }else {
                if pruned_i2 == Interaction::Empty {
                    return pruned_i1;
                } else {
                    return Interaction::Seq( Box::new(pruned_i1) , Box::new(pruned_i2) );
                }
            }
        },
        Interaction::CoReg(cr, i1, i2) => {
            let pruned_i1 = prune(i1,lf_ids);
            let pruned_i2 = prune(i2,lf_ids);
            if pruned_i1 == Interaction::Empty {
                return pruned_i2;
            }else {
                if pruned_i2 == Interaction::Empty {
                    return pruned_i1;
                } else {
                    return Interaction::CoReg( cr.clone(),Box::new(pruned_i1) , Box::new(pruned_i2) );
                }
            }
        },
        Interaction::Strict(i1, i2) => {
            let pruned_i1 = prune(i1,lf_ids);
            let pruned_i2 = prune(i2,lf_ids);
            if pruned_i1 == Interaction::Empty {
                return pruned_i2;
            }else {
                if pruned_i2 == Interaction::Empty {
                    return pruned_i1;
                } else {
                    return Interaction::Strict( Box::new(pruned_i1) , Box::new(pruned_i2) );
                }
            }
        },
        Interaction::Par(i1, i2) => {
            let pruned_i1 = prune(i1,lf_ids);
            let pruned_i2 = prune(i2,lf_ids);
            if pruned_i1 == Interaction::Empty {
                return pruned_i2;
            }else {
                if pruned_i2 == Interaction::Empty {
                    return pruned_i1;
                } else {
                    return Interaction::Par( Box::new(pruned_i1) , Box::new(pruned_i2) );
                }
            }
        },
        Interaction::Alt(i1, i2) => {
            if i1.avoids_all_of(lf_ids) {
                if i2.avoids_all_of(lf_ids) {
                    return Interaction::Alt( Box::new( prune(i1,lf_ids)), Box::new( prune(i2,lf_ids)) );
                } else {
                    return prune(i1,lf_ids);
                }
            } else {
                return prune(i2,lf_ids);
            }
        },
        Interaction::Loop(lkind, i1) => {
            if i1.avoids_all_of(lf_ids) {
                let pruned_i1 = prune(i1,lf_ids);
                if pruned_i1 != Interaction::Empty {
                    return Interaction::Loop(lkind.clone(), Box::new(pruned_i1) );
                }
            }
            return Interaction::Empty;
        },
        _ => {
            panic!("non-conform interaction");
        }
    }
}



pub fn prune_with_affected(my_int : &Interaction, lf_ids : &HashSet<usize>) -> (Interaction,HashSet<usize>) {
    match my_int {
        Interaction::Empty => {
            return (Interaction::Empty,hashset!{});
        },
        Interaction::Emission(_) => {
            return (my_int.clone(),hashset!{});
        },
        Interaction::Reception(_) => {
            return (my_int.clone(),hashset!{});
        },
        Interaction::Seq(i1, i2) => {
            let (pruned_i1,mut aff1) = prune_with_affected(i1,lf_ids);
            let (pruned_i2,aff2) = prune_with_affected(i2,lf_ids);
            aff1.extend(aff2);
            if pruned_i1 == Interaction::Empty {
                return (pruned_i2,aff1);
            }else {
                if pruned_i2 == Interaction::Empty {
                    return (pruned_i1,aff1);
                } else {
                    return (Interaction::Seq( Box::new(pruned_i1) , Box::new(pruned_i2) ),aff1);
                }
            }
        },
        Interaction::CoReg(cr, i1, i2) => {
            let (pruned_i1,mut aff1) = prune_with_affected(i1,lf_ids);
            let (pruned_i2,aff2) = prune_with_affected(i2,lf_ids);
            aff1.extend(aff2);
            if pruned_i1 == Interaction::Empty {
                return (pruned_i2,aff1);
            }else {
                if pruned_i2 == Interaction::Empty {
                    return (pruned_i1,aff1);
                } else {
                    return (Interaction::CoReg( cr.clone(),Box::new(pruned_i1) , Box::new(pruned_i2) ),aff1);
                }
            }
        },
        Interaction::Strict(i1, i2) => {
            let (pruned_i1,mut aff1) = prune_with_affected(i1,lf_ids);
            let (pruned_i2,aff2) = prune_with_affected(i2,lf_ids);
            aff1.extend(aff2);
            if pruned_i1 == Interaction::Empty {
                return (pruned_i2,aff1);
            }else {
                if pruned_i2 == Interaction::Empty {
                    return (pruned_i1,aff1);
                } else {
                    return (Interaction::Strict( Box::new(pruned_i1) , Box::new(pruned_i2) ),aff1);
                }
            }
        },
        Interaction::Par(i1, i2) => {
            let (pruned_i1,mut aff1) = prune_with_affected(i1,lf_ids);
            let (pruned_i2,aff2) = prune_with_affected(i2,lf_ids);
            aff1.extend(aff2);
            if pruned_i1 == Interaction::Empty {
                return (pruned_i2,aff1);
            }else {
                if pruned_i2 == Interaction::Empty {
                    return (pruned_i1,aff1);
                } else {
                    return (Interaction::Par( Box::new(pruned_i1) , Box::new(pruned_i2) ),aff1);
                }
            }
        },
        Interaction::Alt(i1, i2) => {
            if i1.avoids_all_of(lf_ids) {
                if i2.avoids_all_of(lf_ids) {
                    let (pruned_i1,mut aff1) = prune_with_affected(i1,lf_ids);
                    let (pruned_i2,aff2) = prune_with_affected(i2,lf_ids);
                    aff1.extend(aff2);
                    return (Interaction::Alt( Box::new( pruned_i1), Box::new( pruned_i2) ), aff1);
                } else {
                    let pruned_i1 = prune(i1,lf_ids);
                    let mut aff = i1.involved_lifelines();
                    aff.extend(i2.involved_lifelines());
                    return (pruned_i1,aff);
                }
            } else {
                let pruned_i2 = prune(i2,lf_ids);
                let mut aff = i1.involved_lifelines();
                aff.extend(i2.involved_lifelines());
                return (pruned_i2,aff);
            }
        },
        Interaction::Loop(lkind, i1) => {
            if i1.avoids_all_of(lf_ids) {
                let (pruned_i1,aff1) = prune_with_affected(i1,lf_ids);
                if pruned_i1 != Interaction::Empty {
                    return (Interaction::Loop(lkind.clone(), Box::new(pruned_i1) ),aff1);
                } else {
                    return (Interaction::Empty,aff1);
                }
            }
            return (Interaction::Empty,i1.involved_lifelines());
        },
        _ => {
            panic!("non-conform interaction");
        }
    }
}

