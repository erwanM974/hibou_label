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



use std::iter::*;

use itertools::Itertools;

fn get_windows_indexes(min : usize, max : usize) -> Vec<(usize,usize)> {
    let mut size_from_1 = (1..((max-min)+1)).map( |size| {
        (min..max).map( |start| {
            (start,start+size)
        }).filter( |(a,b)| b <= &max).collect::<Vec<(usize,usize)>>()} ).concat();
    size_from_1.insert(0, (min,min) );
    return size_from_1;
}


pub struct Slicer<'a,T> {
    rem_indexes : Vec<(usize,usize)>,
    initial_sequence : &'a Vec<T>
}

impl<T> Slicer<'_,T> {
    pub fn new(initial_sequence : &Vec<T>) -> Slicer<T> {
        let rem_indexes = get_windows_indexes(0, initial_sequence.len());
        return Slicer{rem_indexes,initial_sequence};
    }
}

impl<'a, T> Iterator for Slicer<'a,T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        match self.rem_indexes.pop() {
            None => {
                return None;
            },
            Some( (start_id,end_id) ) => {
                return Some(&self.initial_sequence[start_id..end_id]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::slicer::Slicer;

    #[test]
    fn slicer_product() {
        let vec1 : Vec<u32> = (1..4).collect();
        let vec2 : Vec<u32> = (5..8).collect();
        let mut slicer1 = Slicer::new(&vec1);
        while let Some(got_slice1) = slicer1.next() {
            let mut slicer2 = Slicer::new(&vec2);
            while let Some(got_slice2) = slicer2.next() {
                println!( "{}", format!( "{:?} X {:?}", got_slice1, got_slice2) );
            }
        }
    }

    #[test]
    fn slicer_string() {
        let myvec : Vec<String> = vec!["a".to_string(),"b".to_string(),"c".to_string(),"d".to_string(),"e".to_string()];
        // ***
        let expected_slices = hashset!{"abcde".to_string(),
                                                    "bcde".to_string(),
                                                    "abcd".to_string(),
                                                    "cde".to_string(),
                                                    "bcd".to_string(),
                                                    "abc".to_string(),
                                                    "de".to_string(),
                                                    "cd".to_string(),
                                                    "bc".to_string(),
                                                    "ab".to_string(),
                                                    "e".to_string(),
                                                    "d".to_string(),
                                                    "c".to_string(),
                                                    "b".to_string(),
                                                    "a".to_string(),
                                                    "".to_string()};
        // ***
        let mut got_slices = hashset!{};
        let mut slicer = Slicer::new(&myvec);
        while let Some(got_slice) = slicer.next() {
            //println!( "{}", got_slice.join("") );
            let got_string : String = got_slice.join("");
            got_slices.insert(got_string);
        }
        // ***
        assert_eq!( got_slices, expected_slices);
    }
}