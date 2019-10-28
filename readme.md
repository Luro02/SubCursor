sub_cursor
=========
[![Crates.io: sub_cursor](https://img.shields.io/crates/v/sub_cursor.svg)](https://crates.io/crates/sub_cursor)
[![Documentation](https://docs.rs/hls_m3u8/badge.svg)](https://docs.rs/sub_cursor)
[![Build Status](https://travis-ci.org/luro02/sub_cursor.svg?branch=master)](https://travis-ci.org/luro02/sub_cursor)
[![Code Coverage](https://codecov.io/gh/luro02/sub_cursor/branch/master/graph/badge.svg)](https://codecov.io/gh/luro02/sub_cursor/branch/master)
[![License: Apache](https://img.shields.io/badge/License-Apache%202.0-red.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## What is a SubCursor?
You can think of a [`SubCursor`] as slices for [`Read`]ers or [`Write`]rs instead of buffer.

[`SubCursor`]: https://github.com/Luro02/sub_cursor
[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
### Here is an example usage:

Imagine the following file format

```rust
struct FileOffset {
    start       : u64, // pointer to some data in the file
    end         : u64, // end - start is the length of the file
    filename    : String,
}

struct File {
    data: Vec<FileOffset>,
}
```
the file itself contains many more files and for each of them you have a FileOffset, that points you to the start and end of the file. It also gives you the name for each file.

A File parser could look like this

``` rust
use std::io::{ Read + Seek };
struct Archive<C: Read + Seek> {
    cursor  : C,
    files   : Vec<Vec<u8>>
};
// TODO!
```

## Usage
Add the following to your `Cargo.toml`:

```toml
[dependencies]
sub_cursor = "0.1"
```
if you want to use additional features like ´atomic_refcell´ you should add:
```toml
[dependencies]
sub_cursor = { version = "0.1", features = [ 'atomic_refcell' ] }
```
You can find a list of features in the documentation.

## Features
- `atomic_refcell`: Allows one to use an AtomicRefCell instead of the default std::cell::RefCell
- `czc_cell` ...

## Documentation
You can find the documentation [here](http://doc.rust-lang.org/sub_cursor).

## License

This project is licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](http://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.
