// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

use ggstd::time;
use ggstd::time::Time;
use std::time::SystemTime;

fn main() {
    println!("epoch start: {}", format_time(&time::unix(0, 0)));
    println!("now:         {}", format_time(&time::now()));
    println!(
        "now systime: {}",
        format_time(&Time::from_systime(&SystemTime::now()))
    );
}

fn format_time(t: &Time) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        t.year(),
        t.month(),
        t.day(),
        t.hour(),
        t.minute(),
        t.second()
    )
}
