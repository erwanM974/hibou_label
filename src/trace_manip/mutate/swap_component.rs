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



use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};
use crate::core::execution::trace::multitrace::{MultiTrace};


pub fn mutate_by_swapping_components(multi_trace_1 : &MultiTrace, multi_trace_2 : &MultiTrace, max_num_swaps : usize) -> MultiTrace {
    let num_compos = multi_trace_1.len();
    // ***
    let mut rng = rand::thread_rng();
    let mut compos_indices : Vec<usize> = (0..num_compos).collect();
    compos_indices.shuffle(&mut rng);
    // ***
    let mut rem_num_swaps = max_num_swaps.max(num_compos - 1);
    // ***
    let mut mutated_mt = multi_trace_1.clone();
    while rem_num_swaps > 0 {
        let coloc_id : usize = *compos_indices.get(rem_num_swaps).unwrap();
        vectors_exchange(&mut mutated_mt, &multi_trace_2, coloc_id);
        rem_num_swaps -= 1;
    }
    return mutated_mt;
}


fn mutate_merge_vectors_by_exchange<T : Clone>(rng : &mut ThreadRng, vec_1 : &mut Vec<T>, vec_2 : &Vec<T>) {
    assert_eq!(vec_1.len(),vec_2.len());
    let vec_length = vec_1.len();
    if vec_length >= 1 {
        let rng_indices = Uniform::from(0..vec_length );
        let elt_id : usize = rng_indices.sample(rng);
        // ***
        vectors_exchange(vec_1, vec_2, elt_id);
    }
}


fn vectors_exchange<T : Clone>(vec_1 : &mut Vec<T>, vec_2 : &Vec<T>, index : usize) {
    let substitute : T = vec_2.get(index).unwrap().clone();
    std::mem::replace(&mut vec_1[index], substitute);
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::{vectors_exchange,mutate_merge_vectors_by_exchange};

    #[test]
    fn exchange_test() {
        let mut vec_1 : Vec<u32> = vec![0,1,2,3];
        let vec_2 : Vec<u32> = vec![4,5,6,7];
        vectors_exchange(&mut vec_1, &vec_2, 1);
        let expected : Vec<u32> = vec![0,5,2,3];
        assert_eq!(vec_1,expected)
    }

    #[test]
    fn mutate_merge_test() {
        let mut vec_1 : Vec<u32> = vec![0,1,2];
        let vec_2 : Vec<u32> = vec![4,5,6];
        // ***
        let mut rng = rand::thread_rng();
        mutate_merge_vectors_by_exchange(&mut rng, &mut vec_1, &vec_2);
        // ***
        let expected : HashSet<Vec<u32>> = hashset!{vec![4,1,2],vec![0,5,2],vec![0,1,6]};
        assert!(expected.contains(&vec_1));
    }
}