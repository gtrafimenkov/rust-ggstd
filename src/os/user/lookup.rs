// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// import "sync"

// pub(super) const USER_FILE: &'static str = "/etc/passwd";
// pub(super) const GROUP_FILE: &'static str = "/etc/group";

// var colon = []byte{':'}

/// Current returns the current user.
///
/// The first call will cache the current user information.
/// Subsequent calls will return the cached value and will not reflect
/// changes to the current user.
pub fn current() -> std::io::Result<super::User> {
    // 	cache.Do(fn() { cache.u, cache.err = current() })
    // 	if cache.err != nil {
    // 		return nil, cache.err
    // 	}
    // 	u := *cache.u // copy
    // 	return &u, nil
    super::current_internal()
}

// // cache of the current user
// var cache struct {
// 	sync.Once
// 	u   *User
// 	err error
// }

// // Lookup looks up a user by username. If the user cannot be found, the
// // returned error is of type UnknownUserError.
// fn Lookup(username string) (*User, error) {
// 	if u, err := current(); err == nil && u.Username == username {
// 		return u, err
// 	}
// 	return lookupUser(username)
// }

// // LookupId looks up a user by userid. If the user cannot be found, the
// // returned error is of type UnknownUserIdError.
// fn LookupId(uid string) (*User, error) {
// 	if u, err := current(); err == nil && u.Uid == uid {
// 		return u, err
// 	}
// 	return lookupUserId(uid)
// }

// // LookupGroup looks up a group by name. If the group cannot be found, the
// // returned error is of type UnknownGroupError.
// fn LookupGroup(name string) (*Group, error) {
// 	return lookupGroup(name)
// }

// // LookupGroupId looks up a group by groupid. If the group cannot be found, the
// // returned error is of type UnknownGroupIdError.
// fn LookupGroupId(gid string) (*Group, error) {
// 	return lookupGroupId(gid)
// }

// // GroupIds returns the list of group IDs that the user is a member of.
// fn (u *User) GroupIds() ([]string, error) {
// 	return listGroups(u)
// }
