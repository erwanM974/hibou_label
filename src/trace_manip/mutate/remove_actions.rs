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


use std::path::PathBuf;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};

use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};
use crate::core::general_context::GeneralContext;
use crate::io::file_extensions::HIBOU_TRACE_FILE_EXTENSION;
use crate::io::output::to_hfiles::trace::to_htf::write_multi_trace_into_file;


pub fn generate_remove_actions_mutant(gen_ctx : &GeneralContext,
                                    co_localizations : &CoLocalizations,
                                    multi_trace : &MultiTrace,
                                    parent_folder : Option<&str>,
                                    mutant_name : &str,
                                    max_num_removes : u32) -> String {
    let file_name = format!("{:}.{:}", mutant_name, HIBOU_TRACE_FILE_EXTENSION);
    let path : PathBuf = match parent_folder {
        None => {
            [&file_name].iter().collect()
        },
        Some( parent ) => {
            [parent, &file_name].iter().collect()
        }
    };
    // ***
    let mutant_mt = mutate_by_removing_actions(gen_ctx,co_localizations,multi_trace,max_num_removes);
    write_multi_trace_into_file(path.as_path(),
                                gen_ctx,
                                co_localizations,
                                &mutant_mt);
    // ***
    path.into_os_string().to_str().unwrap().to_string()
}



fn mutate_by_removing_actions(gen_ctx : &GeneralContext,
                             co_localizations : &CoLocalizations,
                             multi_trace : &MultiTrace,
                             max_num_removes : u32) -> MultiTrace {
    // ***
    let mut rem_num_removes = max_num_removes;
    // ***
    let mut rng = rand::thread_rng();
    let rng_coloc_indices = Uniform::from(0..multi_trace.len() );
    // ***
    let mut mutated_mt = multi_trace.clone();
    while rem_num_removes > 0 {
        let coloc_id : usize = rng_coloc_indices.sample(&mut rng);
        // ***
        let component : &mut Trace = mutated_mt.get_mut(coloc_id).unwrap();
        mutate_vector_by_removal(&mut rng,component);
        rem_num_removes -= 1;
    }
    // ***
    mutated_mt
}



fn mutate_vector_by_removal<T>(rng : &mut ThreadRng, vector : &mut Vec<T>) {
    let vec_length = vector.len();
    if vec_length > 0 {
        let mut vec_indices : Vec<usize> = (0..vec_length).collect();
        vec_indices.shuffle(rng);
        vector.remove(*vec_indices.get(0).unwrap());
    }
}




#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::{mutate_vector_by_removal};

    #[test]
    fn mutate_remove_test() {
        let orig_vec : Vec<u32> = vec![0,1,2];
        let mut myvec : Vec<u32> = orig_vec.clone();
        let mut rng = rand::thread_rng();
        mutate_vector_by_removal(&mut rng, &mut myvec);
        println!("{:?}", myvec);
        assert_ne!(myvec,orig_vec);
        // ***
        let expected : HashSet<Vec<u32>> = hashset!{vec![1,2],vec![0,2],vec![0,1]};
        assert!(expected.contains(&myvec));
    }

}