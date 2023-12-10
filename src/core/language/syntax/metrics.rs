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


use strum_macros::EnumIter; // 0.17.1

use std::collections::{HashMap, HashSet};
use strum_macros::IntoStaticStr;
use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::core::language::syntax::action::EmissionTargetRef;
use crate::core::language::syntax::interaction::{Interaction, LoopKind};


#[derive(IntoStaticStr, EnumIter, Clone, PartialEq, Eq, PartialOrd, Copy, Ord, Hash, Debug)]
pub enum SymbolKind {
    Empty,
    Action,
    Strict,
    Seq,
    Par,
    LoopS,
    LoopOther,
    CoReg,
    Alt,
    Sync,
    And
}


pub struct InteractionMetrics {
    pub symbols : HashMap<SymbolKind,u32>,
    pub depth : u32,
    pub max_nested_loop_depth : u32,
    pub lifelines : HashSet<usize>,
}

impl InteractionMetrics {

    pub fn add_csv_title_line(results : &mut String) {
        for symbol in SymbolKind::iter() {
            let as_static_str : &'static str = symbol.into();
            results.push_str(as_static_str);
            results.push_str(",");
        }
        results.push_str("depth,");
        results.push_str("loop_depth,");
        results.push_str("num_lifelines,");
        results.push_str("num_symbols,");
    }

    pub fn get_num_symbols(&self) -> u32 {
        self.symbols.iter().fold(0_u32,|x,(_,c)|x + *c)
    }

    pub fn add_csv_line(&self, results : &mut String) {
        for (_,num_occ) in self.symbols.iter()
            .sorted_by(|(s1,_),(s2,_)| Ord::cmp(s1,s2)) {
            results.push_str(&num_occ.to_string());
            results.push_str(",");
        }
        results.push_str(&self.depth.to_string());
        results.push_str(",");
        results.push_str(&self.max_nested_loop_depth.to_string());
        results.push_str(",");
        results.push_str(&self.lifelines.len().to_string());
        results.push_str(",");
        results.push_str(&format!("{:}",self.get_num_symbols()));
        results.push_str(",");
    }


    pub fn extract_from_interaction(i : &Interaction) -> Self {
        let mut metrics = Self {
            symbols:HashMap::new(),
            depth:0,
            max_nested_loop_depth:0,
            lifelines:HashSet::new()
        };
        for symbol_kind in SymbolKind::iter() {
            metrics.symbols.insert(symbol_kind,0);
        }
        metrics.extract(i,0,0);
        metrics
    }

    fn extract(&mut self,
               i : &Interaction,
               depth : u32,
               loop_depth : u32) {
        match i {
            Interaction::Empty => {
                self.depth = self.depth.max(depth);
                self.max_nested_loop_depth = self.max_nested_loop_depth.max(loop_depth);
                let count = self.symbols.get_mut(&SymbolKind::Empty).unwrap();
                *count += 1;
            }, Interaction::Emission(em) => {
                self.depth = self.depth.max(depth);
                self.max_nested_loop_depth = self.max_nested_loop_depth.max(loop_depth);
                let count = self.symbols.get_mut(&SymbolKind::Action).unwrap();
                *count += 1;
                self.lifelines.insert(em.origin_lf_id);
                for target in &em.targets {
                    match target {
                        EmissionTargetRef::Lifeline(lf) => {
                            self.lifelines.insert(*lf);
                        },
                        _ => {}
                    }
                }
            }, Interaction::Reception(rc) => {
                self.depth = self.depth.max(depth);
                self.max_nested_loop_depth = self.max_nested_loop_depth.max(loop_depth);
                let count = self.symbols.get_mut(&SymbolKind::Action).unwrap();
                *count += 1;
                for lf in &rc.recipients {
                    self.lifelines.insert(*lf);
                }
            }, Interaction::Strict(ref i1, ref i2) => {
                let count = self.symbols.get_mut(&SymbolKind::Strict).unwrap();
                *count += 1;
                self.extract(i1,depth + 1, loop_depth);
                self.extract(i2,depth + 1, loop_depth);
            }, Interaction::Seq(ref i1, ref i2) => {
                let count = self.symbols.get_mut(&SymbolKind::Seq).unwrap();
                *count += 1;
                self.extract(i1,depth + 1, loop_depth);
                self.extract(i2,depth + 1, loop_depth);
            }, Interaction::CoReg(_, ref i1, ref i2) => {
                let count = self.symbols.get_mut(&SymbolKind::CoReg).unwrap();
                *count += 1;
                self.extract(i1,depth + 1, loop_depth);
                self.extract(i2,depth + 1, loop_depth);
            }, Interaction::Par(ref i1, ref i2) => {
                let count = self.symbols.get_mut(&SymbolKind::Par).unwrap();
                *count += 1;
                self.extract(i1,depth + 1, loop_depth);
                self.extract(i2,depth + 1, loop_depth);
            }, Interaction::Alt(ref i1, ref i2) => {
                let count = self.symbols.get_mut(&SymbolKind::Alt).unwrap();
                *count += 1;
                self.extract(i1,depth + 1, loop_depth);
                self.extract(i2,depth + 1, loop_depth);
            }, Interaction::Sync(_, ref i1, ref i2) => {
                let count = self.symbols.get_mut(&SymbolKind::Sync).unwrap();
                *count += 1;
                self.extract(i1,depth + 1, loop_depth);
                self.extract(i2,depth + 1, loop_depth);
            }, Interaction::And( ref i1, ref i2) => {
                let count = self.symbols.get_mut(&SymbolKind::And).unwrap();
                *count += 1;
                self.extract(i1,depth + 1, loop_depth);
                self.extract(i2,depth + 1, loop_depth);
            }, Interaction::Loop(lk, ref i1) => {
                match *lk {
                    LoopKind::SStrictSeq => {
                        let count = self.symbols.get_mut(&SymbolKind::LoopS).unwrap();
                        *count += 1;
                    },
                    _ => {
                        let count = self.symbols.get_mut(&SymbolKind::LoopOther).unwrap();
                        *count += 1;
                    }
                }
                self.extract(i1,depth + 1, loop_depth + 1);
            }
        }
    }
}


impl Interaction {


    pub fn max_nested_loop_depth(&self) -> u32 {
        match *self {
            Interaction::Empty => {
                return 0;
            }, Interaction::Emission(_) => {
                return 0;
            }, Interaction::Reception(_) => {
                return 0;
            }, Interaction::Strict(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, Interaction::Seq(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, Interaction::Par(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, Interaction::Alt(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, Interaction::Loop(_, ref i1) => {
                return 1 + i1.max_nested_loop_depth();
            }, Interaction::Sync(_, ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn total_loop_num(&self) -> u32 {
        match self {
            &Interaction::Empty => {
                return 0;
            }, &Interaction::Emission(_) => {
                return 0;
            }, &Interaction::Reception(_) => {
                return 0;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Loop(_, ref i1) => {
                return 1 + i1.total_loop_num();
            }, &Interaction::Sync(_, ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }


}