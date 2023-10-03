// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// // These may be set to false in init() for a particular platform and/or
// // build flags to let the tests know to skip tests of some features.
// var (
// 	userImplemented      = true
// 	groupImplemented     = true
// 	groupListImplemented = true
// )

use crate::errors;

/// User represents a user account.
#[derive(Debug)]
pub struct User {
    /// uid is the user ID.
    /// On POSIX systems, this is a decimal number representing the uid.
    /// On Windows, this is a security identifier (SID) in a string format.
    /// On Plan 9, this is the contents of /dev/user.
    pub uid: String,
    /// gid is the primary group ID.
    /// On POSIX systems, this is a decimal number representing the gid.
    /// On Windows, this is a SID in a string format.
    /// On Plan 9, this is the contents of /dev/user.
    pub gid: String,
    /// Username is the login name.
    pub username: String,
    /// Name is the user's real or display name.
    /// It might be blank.
    /// On POSIX systems, this is the first (or only) entry in the GECOS field
    /// list.
    /// On Windows, this is the user's display name.
    /// On Plan 9, this is the contents of /dev/user.
    pub name: String,
    /// home_dir is the path to the user's home directory (if they have one).
    pub home_dir: String,
}

/// Group represents a grouping of users.
///
/// On POSIX systems Gid contains a decimal number representing the group ID.
pub struct Group {
    pub gid: String,  // group ID
    pub name: String, // group name
}

/// unknown_user_id_error is returned by LookupId when a user cannot be found.
// type UnknownUserIdError int
pub fn unknown_user_id_error(uid: u32) -> std::io::Error {
    errors::new_stdio_other_error(format!("user: unknown userid {}", uid))
}

// // UnknownUserError is returned by Lookup when
// // a user cannot be found.
// type UnknownUserError string

// func (e UnknownUserError) Error() string {
// 	return "user: unknown user " + string(e)
// }

// // UnknownGroupIdError is returned by LookupGroupId when
// // a group cannot be found.
// type UnknownGroupIdError string

// func (e UnknownGroupIdError) Error() string {
// 	return "group: unknown groupid " + string(e)
// }

// // UnknownGroupError is returned by LookupGroup when
// // a group cannot be found.
// type UnknownGroupError string

// func (e UnknownGroupError) Error() string {
// 	return "group: unknown group " + string(e)
// }
