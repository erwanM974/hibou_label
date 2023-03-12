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
use crate::core::language::hide::hideable::LifelineHideable;
use crate::core::language::syntax::interaction::Interaction;


impl LifelineHideable for Interaction {

    fn hide(&self, lfs_to_remove: &HashSet<usize>) -> Interaction {
        match self {
            Interaction::Empty => {
                return Interaction::Empty;
            },
            Interaction::Emission( ref em_act ) => {
                return em_act.hide(lfs_to_remove);
            },
            Interaction::Reception( ref rc_act ) => {
                return rc_act.hide(lfs_to_remove);
            },
            Interaction::Seq(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                return Interaction::Seq(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    }
                }
            },
            Interaction::Sync(sync_acts,i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                // ***
                let mut new_sync_acts= vec![];
                for sync_act in sync_acts {
                    if !lfs_to_remove.contains(&sync_act.lf_id) {
                        new_sync_acts.push(sync_act.clone());
                    }
                }
                if new_sync_acts.len() > 0 {
                    return Interaction::Sync(new_sync_acts,
                                              Box::new(i1hid),
                                              Box::new(i2hid) );
                } else {
                    match &i1hid {
                        Interaction::Empty => {
                            return i2hid;
                        },
                        _ => {
                            match &i2hid {
                                Interaction::Empty => {
                                    return i1hid;
                                },
                                _ => {
                                    return Interaction::Par(Box::new(i1hid),
                                                            Box::new(i2hid) );
                                }
                            }
                        }
                    }
                }
            },
            Interaction::CoReg(cr,i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                let mut new_cr= vec![];
                                for concurrent_lf in cr {
                                    if !lfs_to_remove.contains(concurrent_lf) {
                                        new_cr.push(*concurrent_lf);
                                    }
                                }
                                if new_cr.len() > 0 {
                                    return Interaction::CoReg(new_cr,
                                                              Box::new(i1hid),
                                                              Box::new(i2hid) );
                                } else {
                                    return Interaction::Seq(Box::new(i1hid),
                                                              Box::new(i2hid) );
                                }
                            }
                        }
                    }
                }
            },
            Interaction::Strict(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                return Interaction::Strict(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    }
                }
            },
            Interaction::Alt(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        match &i2hid {
                            Interaction::Empty => {
                                return Interaction::Empty
                            },
                            _ => {
                                return Interaction::Alt(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    },
                    _ => {
                        return Interaction::Alt(Box::new(i1hid), Box::new(i2hid));
                    }
                }
            },
            Interaction::Par(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                return Interaction::Par(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    }
                }
            },
            Interaction::Loop(opkind,i1) => {
                let i1hid = i1.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return Interaction::Empty;
                    },
                    Interaction::Loop(opkind2,i11) => {
                        return Interaction::Loop((opkind.min(opkind2)).clone(), i11.clone());
                    },
                    _ => {
                        return Interaction::Loop(opkind.clone(),Box::new(i1hid) );
                    }
                }
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

}

