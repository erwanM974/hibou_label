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
use crate::core::language::eliminate_lf::eliminable::LifelineEliminable;
use crate::core::language::syntax::interaction::Interaction;


impl LifelineEliminable for Interaction {

    fn eliminate_lifelines(&self, lfs_to_eliminate: &HashSet<usize>) -> Interaction {
        match self {
            Interaction::Empty => {
                return Interaction::Empty;
            },
            Interaction::Emission( ref em_act ) => {
                return em_act.eliminate_lifelines(lfs_to_eliminate);
            },
            Interaction::Reception( ref rc_act ) => {
                return rc_act.eliminate_lifelines(lfs_to_eliminate);
            },
            Interaction::Seq(i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate);
                match &new_i1 {
                    Interaction::Empty => {
                        return new_i2;
                    },
                    _ => {
                        match &new_i2 {
                            Interaction::Empty => {
                                return new_i1
                            },
                            _ => {
                                return Interaction::Seq(Box::new(new_i1), Box::new(new_i2));
                            }
                        }
                    }
                }
            },
            Interaction::Sync(sync_acts,i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate);
                // ***
                let mut new_sync_acts= vec![];
                for sync_act in sync_acts {
                    if !lfs_to_eliminate.contains(&sync_act.lf_id) {
                        new_sync_acts.push(sync_act.clone());
                    }
                }
                if new_sync_acts.len() > 0 {
                    return Interaction::Sync(new_sync_acts,
                                              Box::new(new_i1),
                                              Box::new(new_i2) );
                } else {
                    match &new_i1 {
                        Interaction::Empty => {
                            return new_i2;
                        },
                        _ => {
                            match &new_i2 {
                                Interaction::Empty => {
                                    return new_i1;
                                },
                                _ => {
                                    return Interaction::Par(Box::new(new_i1),
                                                            Box::new(new_i2) );
                                }
                            }
                        }
                    }
                }
            },
            Interaction::CoReg(cr,i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate);
                match &new_i1 {
                    Interaction::Empty => {
                        return new_i2;
                    },
                    _ => {
                        match &new_i2 {
                            Interaction::Empty => {
                                return new_i1
                            },
                            _ => {
                                let mut new_cr= vec![];
                                for concurrent_lf in cr {
                                    if !lfs_to_eliminate.contains(concurrent_lf) {
                                        new_cr.push(*concurrent_lf);
                                    }
                                }
                                if new_cr.len() > 0 {
                                    return Interaction::CoReg(new_cr,
                                                              Box::new(new_i1),
                                                              Box::new(new_i2) );
                                } else {
                                    return Interaction::Seq(Box::new(new_i1),
                                                              Box::new(new_i2) );
                                }
                            }
                        }
                    }
                }
            },
            Interaction::Strict(i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate);
                match &new_i1 {
                    Interaction::Empty => {
                        return new_i2;
                    },
                    _ => {
                        match &new_i2 {
                            Interaction::Empty => {
                                return new_i1
                            },
                            _ => {
                                return Interaction::Strict(Box::new(new_i1), Box::new(new_i2));
                            }
                        }
                    }
                }
            },
            Interaction::Alt(i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate);
                match &new_i1 {
                    Interaction::Empty => {
                        match &new_i2 {
                            Interaction::Empty => {
                                return Interaction::Empty
                            },
                            _ => {
                                return Interaction::Alt(Box::new(new_i1), Box::new(new_i2));
                            }
                        }
                    },
                    _ => {
                        return Interaction::Alt(Box::new(new_i1), Box::new(new_i2));
                    }
                }
            },
            Interaction::Par(i1,i2) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate);
                let new_i2 = i2.eliminate_lifelines(lfs_to_eliminate);
                match &new_i1 {
                    Interaction::Empty => {
                        return new_i2;
                    },
                    _ => {
                        match &new_i2 {
                            Interaction::Empty => {
                                return new_i1
                            },
                            _ => {
                                return Interaction::Par(Box::new(new_i1), Box::new(new_i2));
                            }
                        }
                    }
                }
            },
            Interaction::Loop(opkind,i1) => {
                let new_i1 = i1.eliminate_lifelines(lfs_to_eliminate);
                match &new_i1 {
                    Interaction::Empty => {
                        return Interaction::Empty;
                    },
                    Interaction::Loop(opkind2,i11) => {
                        return Interaction::Loop((opkind.min(opkind2)).clone(), i11.clone());
                    },
                    _ => {
                        return Interaction::Loop(opkind.clone(),Box::new(new_i1) );
                    }
                }
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

}

