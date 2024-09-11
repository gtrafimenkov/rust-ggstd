// Copyright 2024 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

use ggstd::os;

fn main() {
    let hostname = os::hostname().expect("failed to get the host name");
    println!("Hostname: {}", hostname);
}
