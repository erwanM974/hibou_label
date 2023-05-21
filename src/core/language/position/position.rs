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



use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Position {
    Epsilon(Option<usize>),
    Left(Box<Position>),
    Right(Box<Position>),
    Both(Box<Position>,Box<Position>)
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Position::Epsilon(sub_pos) => {
                match sub_pos {
                    None => {
                        write!(f,"")
                    },
                    Some(sbp_idx) => {
                        write!(f,"s{:}",sbp_idx)
                    }
                }
            },
            Position::Left(ref in_self) => {
                write!(f,"1{:}",in_self)
            },
            Position::Right(ref in_self) => {
                write!(f,"2{:}",in_self)
            },
            Position::Both(ref sub1, ref sub2) => {
                write!(f,"(1{:},2{:})",sub1,sub2)
            }
        }
    }
}





