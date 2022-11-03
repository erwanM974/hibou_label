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




use std::ptr;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};
use crate::core::general_context::GeneralContext;
use crate::output::to_hfiles::multitrace_to_htf::write_multi_trace_into_file;


pub fn generate_swap_actions_mutant(gen_ctx : &GeneralContext,
                       co_localizations : &CoLocalizations,
                       multi_trace : &MultiTrace,
                       parent_folder : Option<&str>,
                       mutant_name : &str,
                       max_num_swaps : u32) -> String{
    let file_path : String;
    match parent_folder {
        None => {
            file_path = format!("./{:}", mutant_name);
        },
        Some( parent ) => {
            file_path = format!("{:}/{:}", parent, mutant_name);
        }
    }
    // ***
    let mutant_mt = mutate_by_swapping_actions(multi_trace,max_num_swaps);
    write_multi_trace_into_file(&file_path,
                                gen_ctx,
                                co_localizations,
                                &mutant_mt);
    return file_path;
}


fn mutate_by_swapping_actions(multi_trace : &MultiTrace, max_num_swaps : u32) -> MultiTrace {
    // ***
    let mut rem_num_swaps = max_num_swaps.min(multi_trace_max_swap(multi_trace));
    // ***
    let mut rng = rand::thread_rng();
    let rng_coloc_indices = Uniform::from(0..multi_trace.len() );
    // ***
    let mut mutated_mt = multi_trace.clone();
    while rem_num_swaps > 0 {
        let coloc_id : usize = rng_coloc_indices.sample(&mut rng);
        let component : &mut Trace = mutated_mt.get_mut(coloc_id).unwrap();
        // ***
        if mutate_vector_by_swapping(&mut rng, component) {
            rem_num_swaps -= 1;
        }
    }
    return mutated_mt;
}

fn multi_trace_max_swap(multi_trace : &MultiTrace) -> u32 {
    let mut max_swap = 0;
    for trace in multi_trace {
        if trace.len() > 1 {
            max_swap += (trace.len() - 1) as u32;
        }
    }
    return max_swap;
}


fn mutate_vector_by_swapping<T>(rng : &mut ThreadRng,vector : &mut Vec<T>) -> bool {
    let vec_length = vector.len();
    if vec_length > 1 {
        let mut vec_indices : Vec<usize> = (0..vec_length).collect();
        vec_indices.shuffle(rng);
        vector_swap(vector, *vec_indices.get(0).unwrap(), *vec_indices.get(1).unwrap());
        return true;
    }
    return false;
}

fn vector_swap<T>(vector : &mut Vec<T>, a: usize, b: usize) {
    unsafe {
        let pa: *mut T = &mut vector[a];
        let pb: *mut T = &mut vector[b];
        ptr::swap(pa, pb);
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::{vector_swap, mutate_vector_by_swapping};

    #[test]
    fn swap_test() {
        let mut myvec : Vec<u32> = vec![0,1,2,3];
        let expected : Vec<u32> = vec![0,2,1,3];
        vector_swap(&mut myvec,1,2);
        assert_eq!(myvec,expected)
    }

    #[test]
    fn mutate_swap_test() {
        let orig_vec : Vec<u32> = vec![0,1,2,3];
        let mut myvec : Vec<u32> = orig_vec.clone();
        let mut rng = rand::thread_rng();
        mutate_vector_by_swapping(&mut rng, &mut myvec);
        println!("{:?}", myvec);
        assert_ne!(myvec,orig_vec);
        // ***
        let expected : HashSet<u32> = hashset!{0,1,2,3};
        let as_hashset: HashSet<u32> = myvec.iter().cloned().collect();
        assert_eq!(as_hashset,expected)
    }
}