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

#[derive(Debug)]
pub enum HibouParsingError {
    FileFormatError(String,String),
    FileError(String),
    MatchError(String),
    // ***
    HsfSetupError(String),
    HcfSetupError(String),
    ProcessFilterError(String),
    ProcessPriorityError(String),
    // ***
    MissingMessageDeclarationError(String),
    MissingLifelineDeclarationError(String),
    MissingGateDeclarationError(String),
    MissingLifelineOrGateDeclarationError(String),
    // ***
    EmissionDefinitionError(String),
    OtherDefinitionError(String),
    // ***
    NonDisjointTraceComponents,
    IllDefinedTraceComponents(String)
}

impl fmt::Display for HibouParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HibouParsingError::FileFormatError( got, expected) => {
                return write!(f, "{}", format!("expected '.{}' file and got '.{}' file", expected, got));
            },
            HibouParsingError::FileError(sub_e) => {
                return write!(f, "{}", format!("error while reading SD conf file : {:}", sub_e));
            },
            HibouParsingError::MatchError(sub_e) => {
                return write!(f, "{}", format!("error while parsing SD string : {:}", sub_e));
            },
            // ***
            HibouParsingError::HsfSetupError(sub_e) => {
                return write!(f, "{}", format!("error while parsing setup section of .hsf file : {:}", sub_e));
            },
            HibouParsingError::HcfSetupError(sub_e) => {
                return write!(f, "{}", format!("error while parsing setup section of .hcf file : {:}", sub_e));
            },
            HibouParsingError::ProcessFilterError(sub_e) => {
                return write!(f, "{}", format!("error while parsing filters in .hcf file : {:}", sub_e));
            },
            HibouParsingError::ProcessPriorityError(sub_e) => {
                return write!(f, "{}", format!("error while parsing priorities in .hcf file : {:}", sub_e));
            },
            // ***
            HibouParsingError::MissingMessageDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing message declaration : {:}", sub_e));
            },
            HibouParsingError::MissingLifelineDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing lifeline declaration : {:}", sub_e));
            },
            HibouParsingError::MissingGateDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing gate declaration : {:}", sub_e));
            },
            HibouParsingError::MissingLifelineOrGateDeclarationError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; missing lifeline or gate declaration : {:}", sub_e));
            },
            // ***
            HibouParsingError::EmissionDefinitionError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; emission definition error : {:}", sub_e));
            },
            HibouParsingError::OtherDefinitionError(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; other definition error : {:}", sub_e));
            },
            // ***
            HibouParsingError::NonDisjointTraceComponents => {
                return write!(f, "{}", format!("error while parsing ; non disjoint trace canals"));
            },
            HibouParsingError::IllDefinedTraceComponents(sub_e) => {
                return write!(f, "{}", format!("error while parsing ; ill defined trace canals : {:}", sub_e));
            }
        }
    }
}

