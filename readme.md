sub_cursor
=========
[![Crates.io: sub_cursor](https://img.shields.io/crates/v/sub_cursor.svg)](https://crates.io/crates/sub_cursor)
[![Documentation](https://docs.rs/sub_cursor/badge.svg)](https://docs.rs/sub_cursor)
[![Build Status](https://travis-ci.org/luro02/sub_cursor.svg?branch=master)](https://travis-ci.org/luro02/sub_cursor)
[![Code Coverage](https://codecov.io/gh/luro02/sub_cursor/branch/master/graph/badge.svg)](https://codecov.io/gh/luro02/sub_cursor/branch/master)
[![License: Apache](https://img.shields.io/badge/License-Apache%202.0-red.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## What is a SubCursor?
You can think of a [`SubCursor`] as slices for [`Read`]ers or [`Write`]rs instead of buffer.

[`SubCursor`]: https://github.com/Luro02/sub_cursor
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html

## Why?
A `SubCursor` provides a more efficient way, to handle data in apis.

Let's imagine you have an archive, that requires files:
```rust
use std::io::{self, Cursor, Read, Seek};

pub struct Archive<T> {
    files: Vec<T>,
}

impl<T: Read + Seek> Archive<T> {
    pub fn new() -> Self {
        Self {
            files: vec![]
        }
    }

    pub fn push(&mut self, value: T) {
        self.files.push(value)
    }
}

fn main() -> io::Result<()> {
    let mut archive = Archive::new();
    // imagine, that these are Files instead of Cursor
    archive.push(Cursor::new(b"This is an example file"));
    archive.push(Cursor::new(b"This is another example file"));

    Ok(())
}
```
Now you have a single file, that contains many smaller files (for example a `.zip`) and you want to add them to the `Archive`, without reading the entire file into memory and wrapping each of them in a `Cursor`.
This can be achieved with a `SubCursor`, which is like slices, but for `Read`er and `Write`er.

```rust
use std::io::{self, Seek, Read, Cursor};
use std::sync::{Arc, Mutex};

use sub_cursor::SubCursor;

pub struct Archive<T> {
    files: Vec<T>,
}

impl<T: Read + Seek> Archive<T> {
    pub fn new() -> Self {
        Self {
            files: vec![]
        }
    }

    pub fn push(&mut self, value: T) {
        self.files.push(value)
    }

    pub fn print_files(&mut self) -> io::Result<()> {
        for file in &mut self.files {
            let mut string = String::new();
            file.read_to_string(&mut string)?;
            println!("{}", string);
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut archive = Archive::new();
    // imagine, that these are Files instead of Cursor
    archive.push(Cursor::new(b"This is an example file"));
    archive.push(Cursor::new(b"This is another example file"));

    let file = Arc::new(Mutex::new(Cursor::new(b"file1,file2,file3")));
    archive.push(
        SubCursor::from(file.clone())
            // first file starts at index 0
            .start(0)
            // and ends at 5
            .end(5)
    );

    archive.push(
        SubCursor::from(file.clone())
            .start(7)
            .end(11)
    );

    archive.push(
        // the end will be set automatically
        SubCursor::from(file.clone()).start(12)
    );

    archive.print_files()?;

    Ok(())
}
```

## Usage
Add the following to your `Cargo.toml`:

```toml
[dependencies]
sub_cursor = "0.1"
```

## Documentation
You can find the documentation [here](https://docs.rs/sub_cursor).

## License

This project is licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](http://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.
