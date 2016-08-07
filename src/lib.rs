#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

// xcur-rs - Parser for XCursor files in Rust
// Copyright (C) 2016  Jean Pierre Dudey
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.


//! Welcome!

#[macro_use]
extern crate nom;

pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser;

    #[test]
    fn parse_sample_file() {
        let data = include_bytes!("../assets/xterm");
        let file = parser::File::parse(data).unwrap();
    }
}
