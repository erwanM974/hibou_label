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
use crate::core::execution::trace::from_model::from_model::InteractionInterpretableAsTraceAction;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::avoid::avoids::AvoidsLifelines;
use crate::core::language::involve::involves::InvolvesLifelines;

use crate::core::language::prune::prunable::LifelinePrunable;
use crate::core::language::syntax::interaction::Interaction;



impl LifelinePrunable for Interaction {
    fn prune(&self, lf_ids : &HashSet<usize>) -> Interaction {
        match self {
            Interaction::Empty => {
                return Interaction::Empty;
            },
            Interaction::Emission(_) => {
                return self.clone();
            },
            Interaction::Reception(_) => {
                return self.clone();
            },
            Interaction::Seq(i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
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
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
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
            Interaction::Sync(sync_acts, i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
                // ***
                let acts1 = pruned_i1.get_all_trace_actions();
                let acts2 = pruned_i2.get_all_trace_actions();
                // ***
                let sync_acts_as_hashset : HashSet<TraceAction> = HashSet::from_iter(sync_acts.iter().cloned());
                let intersetc1 = sync_acts_as_hashset.intersection(&acts1).count();
                let intersetc2 = sync_acts_as_hashset.intersection(&acts2).count();
                // ***
                let new_i : Interaction;
                if intersetc1 == 0 && intersetc2 == 0 {
                    if pruned_i1 == Interaction::Empty {
                        return pruned_i2;
                    } else {
                        if pruned_i2 == Interaction::Empty {
                            return pruned_i1;
                        } else {
                            return Interaction::Par( Box::new(pruned_i1) ,
                                                     Box::new(pruned_i2) );
                        }
                    }
                } else {
                    return Interaction::Sync(sync_acts.clone(),
                                             Box::new(pruned_i1) ,
                                             Box::new(pruned_i2));
                }
            },
            Interaction::Strict(i1, i2) => {
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
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
                let pruned_i1 = i1.prune(lf_ids);
                let pruned_i2 = i2.prune(lf_ids);
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
                        return Interaction::Alt( Box::new( i1.prune(lf_ids)), Box::new( i2.prune(lf_ids)) );
                    } else {
                        return i1.prune(lf_ids);
                    }
                } else {
                    return i2.prune(lf_ids);
                }
            },
            Interaction::Loop(lkind, i1) => {
                if i1.avoids_all_of(lf_ids) {
                    let pruned_i1 = i1.prune(lf_ids);
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

    fn prune_with_affected(&self, lf_ids : &HashSet<usize>) -> (Interaction,HashSet<usize>) {
        match self {
            Interaction::Empty => {
                return (Interaction::Empty,hashset!{});
            },
            Interaction::Emission(_) => {
                return (self.clone(),hashset!{});
            },
            Interaction::Reception(_) => {
                return (self.clone(),hashset!{});
            },
            Interaction::Seq(i1, i2) => {
                let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
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
                let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
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
                let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
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
                let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
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
                        let (pruned_i1,mut aff1) = i1.prune_with_affected(lf_ids);
                        let (pruned_i2,aff2) = i2.prune_with_affected(lf_ids);
                        aff1.extend(aff2);
                        return (Interaction::Alt( Box::new( pruned_i1), Box::new( pruned_i2) ), aff1);
                    } else {
                        let pruned_i1 = i1.prune(lf_ids);
                        let mut aff = i1.involved_lifelines();
                        aff.extend(i2.involved_lifelines());
                        return (pruned_i1,aff);
                    }
                } else {
                    let pruned_i2 = i2.prune(lf_ids);
                    let mut aff = i1.involved_lifelines();
                    aff.extend(i2.involved_lifelines());
                    return (pruned_i2,aff);
                }
            },
            Interaction::Loop(lkind, i1) => {
                if i1.avoids_all_of(lf_ids) {
                    let (pruned_i1,aff1) = i1.prune_with_affected(lf_ids);
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
}



