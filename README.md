<p align="center">
  <a href="https://travis-ci.org/jeandudey/xcur-rs">
    <img src="https://travis-ci.org/jeandudey/xcur-rs.svg?branch=master">
  </a>

  <br>

  <a href="http://jeandudey.github.io/xcur-rs/index.html">
    <strong>Documentation</strong>
  </a>
</p>

# xcur-rs
Parser for XCursor files in Rust.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
xcur = "0.1.*"
```

And this to your crate root:
```rust
extern crate xcur;
```

## Example
```rust
extern crate xcur;
extern crate nom;

use xcur::parser::File as XCursorFile;
use nom::IResult;

fn main() {
    let data = include_bytes!("./assets/xterm");
    let xcursor_file = match XCursorFile::parse(data) {
        IResult::Done(_i, file) => file,
        IResult::Incomplete(e) => panic!("Incomplete, needed: {}", e),
        IResult::Error(e) => panic!("Error: {}", e),
    };
}
```

## License
```
xcur-rs - Parser for XCursor files in Rust
Copyright (C) 2016  Jean Pierre Dudey

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
```
