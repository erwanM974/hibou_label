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
use std::iter::FromIterator;


use crate::core::error::HibouCoreError;

#[derive(Clone, PartialEq, Debug)]
pub struct GeneralContext {
    lf_names : Vec<String>,
    ms_names : Vec<String>,
    gt_names : Vec<String>
}



impl GeneralContext {

    pub fn new() -> GeneralContext {
        return GeneralContext {
            lf_names: Vec::new(),
            ms_names: Vec::new(),
            gt_names: Vec::new()
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn add_lf(&mut self, lf_name : String) -> usize {
        match self.get_lf_id(&lf_name) {
            None => {
                self.lf_names.push(lf_name);
                return self.lf_names.len() - 1;
            },
            Some(lf_id) => {
                return lf_id;
            }
        }
    }

    pub fn add_msg(&mut self, ms_name : String) -> usize {
        match self.get_ms_id(&ms_name) {
            None => {
                self.ms_names.push(ms_name);
                return self.ms_names.len() - 1;
            },
            Some(ms_id) => {
                return ms_id;
            }
        }
    }

    pub fn add_gt(&mut self, gt_name : String) -> usize {
        match self.get_gt_id(&gt_name) {
            None => {
                self.gt_names.push(gt_name);
                return self.gt_names.len() - 1;
            },
            Some(gt_id) => {
                return gt_id;
            }
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_id(&self, lf_name : &str) -> Option<usize> {
        return self.lf_names.iter().position(|r| r == lf_name);
    }

    pub fn get_ms_id(&self, ms_name : &str) -> Option<usize> {
        return self.ms_names.iter().position(|n| n == ms_name);
    }

    pub fn get_gt_id(&self, gt_name : &str) -> Option<usize> {
        return self.gt_names.iter().position(|n| n == gt_name);
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_num(&self) -> usize {
        return self.lf_names.len();
    }

    pub fn get_ms_num(&self) -> usize {
        return self.ms_names.len();
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_all_lfs_ids(&self) -> HashSet<usize> {
        return HashSet::from_iter(0..self.get_lf_num() );
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn get_lf_name(&self, lf_id : usize) -> Result<String,HibouCoreError> {
        match self.lf_names.get(lf_id) {
            None => {
                return Err( HibouCoreError::UnknownLifeline(lf_id) );
            },
            Some( got_str ) => {
                return Ok( got_str.to_string() );
            }
        }
    }

    pub fn get_ms_name(&self, ms_id : usize) -> Result<String,HibouCoreError> {
        match self.ms_names.get(ms_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(ms_id) );
            },
            Some( ms_name ) => {
                return Ok( ms_name.to_string() );
            }
        }
    }

    pub fn get_gt_name(&self, gt_id : usize) -> Result<String,HibouCoreError> {
        match self.gt_names.get(gt_id) {
            None => {
                return Err( HibouCoreError::UnknownMessage(gt_id) );
            },
            Some( gt_name ) => {
                return Ok( gt_name.to_string() );
            }
        }
    }

}
