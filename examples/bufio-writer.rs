// Copyright 2023 The rust-ggstd authors.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::bufio;
use std::io::Write;

fn main() {
    example_writer().unwrap();
    // Output: Hello, world!
}

fn example_writer() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    let mut w = bufio::Writer::new(&mut stdout_lock);
    w.write_all(b"Hello, ")?;
    w.write_all(b"world!")?;
    w.flush()?; // Don't forget to flush!
    Ok(())
}
