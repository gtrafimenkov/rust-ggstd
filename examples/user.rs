// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

use ggstd::os;
use ggstd::os::user;

fn main() {
    let res = user::current().unwrap();
    println!("Uid:      {}", res.uid);
    println!("Gid:      {}", res.gid);
    println!("Username: {}", res.username);
    println!("Name:     {}", res.name);
    println!("HomeDir:  {}", res.home_dir);

    println!("user_id:  {}", os::get_uid());
}
