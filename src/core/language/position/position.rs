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



#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Position {
    Left(Box<Position>),
    Right(Box<Position>),
    Epsilon(Option<usize>)
}

impl Position {
    pub fn to_string(&self) -> String {
        match self {
            Position::Left(ref in_self) => {
                let mut my_string = "1".to_string();
                let sub_pos = in_self.to_string();
                if sub_pos != "0".to_string() {
                    my_string.push_str( &sub_pos );
                }
                return my_string;
            },
            Position::Right(ref in_self) => {
                let mut my_string = "2".to_string();
                let sub_pos = in_self.to_string();
                if sub_pos != "0".to_string() {
                    my_string.push_str( &sub_pos );
                }
                return my_string;
            },
            Position::Epsilon(sub_pos) => {
                match sub_pos {
                    None => {
                        return "0".to_string();
                    },
                    Some(sbp_idx) => {
                        return format!("s{:}",sbp_idx);
                    }
                }
            }
        }
    }
}





