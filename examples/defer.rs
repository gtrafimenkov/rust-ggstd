// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

// Replacement for Go's defer.

use ggstd::compat::Defer;

fn main() {
    println!("starting");
    let msg = "hello".to_string();
    let _d = Defer(|| println!("finishing 1: {}", msg));
    {
        let _d = Defer(|| println!("finishing 2"));
    }
    let _d = Defer(|| println!("finishing 3"));
    println!("working");

    // Output:
    //   starting
    //   finishing 2
    //   working
    //   finishing 3
    //   finishing 1: hello
}
