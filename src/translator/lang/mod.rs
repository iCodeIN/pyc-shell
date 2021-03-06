//! ## Languages
//!
//! `lang` are empty structs which must implement the Translator trait

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "Pyc"
*
*   Pyc is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   Pyc is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with Pyc.  If not, see <http://www.gnu.org/licenses/>.
*
*/

/// ### Language
///
/// Cyrillic alphabet language
/// NOTE: add here new languages
#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum Language {
  Belarusian,
  Bulgarian,
  Russian,
  Serbian,
  Ukrainian,
  Nil
}

/// ## Languages
///
/// Languages are empty structs which must implement the Translator trait

//NOTE: languages are listed here
pub(crate) struct Belarusian {}
pub(crate) struct Bulgarian {}
pub(crate) struct Russian {}
pub(crate) struct Serbian {}
pub(crate) struct Ukrainian {}
pub(crate) struct Nil {}
mod belarusian;
mod bulgarian;
mod russian;
mod serbian;
mod ukrainian;
mod nil;

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
        Language::Belarusian => String::from("бел"),
        Language::Bulgarian => String::from("блг"),
        Language::Russian => String::from("рус"),
        Language::Serbian => String::from("срб"),
        Language::Ukrainian => String::from("укр"),
        Language::Nil => String::from("nil")
        }
    }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_translator_language_to_string() {
    assert_eq!(Language::Belarusian.to_string(), String::from("бел"));
    assert_eq!(Language::Bulgarian.to_string(), String::from("блг"));
    assert_eq!(Language::Russian.to_string(), String::from("рус"));
    assert_eq!(Language::Serbian.to_string(), String::from("срб"));
    assert_eq!(Language::Ukrainian.to_string(), String::from("укр"));
    assert_eq!(Language::Nil.to_string(), String::from("nil"));
  }

}
