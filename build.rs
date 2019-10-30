//! this file is to test the `readme.md`.
use skeptic;

fn main() {
    // generates doc tests for `readme.md`
    skeptic::generate_doc_tests(&["readme.md"]);
}
