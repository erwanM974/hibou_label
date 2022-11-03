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

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct CoLocalizations {
    pub locs_lf_ids : Vec<HashSet<usize>>
}

impl CoLocalizations {

    pub fn new(locs_lf_ids :  Vec<HashSet<usize>>) -> CoLocalizations {
        return CoLocalizations{locs_lf_ids};
    }

    pub fn get_trivial_partition(lf_num : usize) -> CoLocalizations {
        let mut all_lfs : HashSet<usize> = HashSet::from_iter((0..lf_num).collect::<Vec<usize>>().iter().cloned());
        return CoLocalizations::new(vec![all_lfs]);
    }

    pub fn get_discrete_partition(lf_num : usize) -> CoLocalizations {
        let mut colocs = vec![];
        for lf_id in 0..lf_num {
            colocs.push( hashset!{lf_id} )
        }
        return CoLocalizations::new(colocs);
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn num_colocs(&self) -> usize {
        return self.locs_lf_ids.len();
    }

    pub fn get_coloc_ids_from_lf_ids(&self, lfs_ids : &HashSet<usize>) -> HashSet<usize> {
        let mut colocs_ids : HashSet<usize> = HashSet::new();
        for lf_id in lfs_ids {
            colocs_ids.insert(self.get_lf_coloc_id(*lf_id).unwrap());
        }
        return colocs_ids;
    }

    pub fn get_lf_ids_from_coloc_ids(&self, coloc_ids : &HashSet<usize>) -> HashSet<usize> {
        let mut lfs_ids : HashSet<usize> = HashSet::new();
        for coloc_id in coloc_ids {
            let got_lf_ids : &HashSet<usize> = self.locs_lf_ids.get(*coloc_id).unwrap();
            lfs_ids.extend(got_lf_ids);
        }
        return lfs_ids;
    }

    pub fn get_coloc_lfs_ids(&self, coloc_id : usize) -> &HashSet<usize> {
        let got_lf_ids : &HashSet<usize> = self.locs_lf_ids.get(coloc_id).unwrap();
        return got_lf_ids;//.clone();
    }

    pub fn get_lf_coloc_id(&self, lf_id : usize) -> Option<usize> {
        for (coloc_id,coloc) in self.locs_lf_ids.iter().enumerate() {
            if coloc.contains(&lf_id) {
                return Some(coloc_id);
            }
        }
        return None;
    }

    pub fn are_colocalizations_singletons(&self) -> bool {
        for coloc in self.locs_lf_ids.iter() {
            if coloc.len() > 1 {
                return false;
            }
        }
        return true;
    }
}