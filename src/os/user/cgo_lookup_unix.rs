// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use super::User;
use crate::errors;
use crate::strings;
use libc;

// //go:build (cgo || darwin) && !osusergo && unix && !android

// package user

// import (
// 	"fmt"
// 	"runtime"
// 	"strconv"
// 	"strings"
// 	"syscall"
// 	"unsafe"
// )

pub fn current_internal() -> std::io::Result<User> {
    lookup_unix_uid(unsafe { libc::getuid() })
}

// fn lookupUser(username string) (*User, error) {
// 	var pwd _C_struct_passwd
// 	var found bool
// 	nameC := make([]byte, len(username)+1)
// 	copy(nameC, username)

// 	err := retryWithBuffer(userBuffer, fn(buf []byte) syscall.Errno {
// 		var errno syscall.Errno
// 		pwd, found, errno = _C_getpwnam_r((*_C_char)(unsafe.Pointer(&nameC[0])),
// 			(*_C_char)(unsafe.Pointer(&buf[0])), _C_size_t(len(buf)))
// 		return errno
// 	})
// 	if err != nil {
// 		return nil, fmt.Errorf("user: lookup username %s: %v", username, err)
// 	}
// 	if !found {
// 		return nil, UnknownUserError(username)
// 	}
// 	return build_user(&pwd), err
// }

// fn lookupUserId(uid string) -> std::io::Result<User> {
// 	i, e := strconv.Atoi(uid)
// 	if e != nil {
// 		return nil, e
// 	}
// 	return lookup_unix_uid(i)
// }

fn lookup_unix_uid(uid: u32) -> std::io::Result<User> {
    let mut pwd = unsafe { std::mem::zeroed::<libc::passwd>() };
    let mut buf = vec![0; 1024];
    let mut result = std::ptr::null_mut::<libc::passwd>();

    loop {
        let err =
            unsafe { libc::getpwuid_r(uid, &mut pwd, buf.as_mut_ptr(), buf.len(), &mut result) };

        // from man getpwuid_r:
        //   On success, getpwnam_r() and getpwuid_r() return zero, and set *result to pwd.
        //   If no matching password record was found, these functions return 0 and store NULL
        //   in *result.  In case of error, an error number is returned, and NULL is stored in *result.
        //
        //   Errors:
        //     ...
        //     ERANGE Insufficient buffer space supplied.

        if err == libc::ERANGE {
            let new_size = buf.len() * 2;
            if !is_size_reasonable(new_size) {
                return Err(errors::new_stdio_other_error(format!(
                    "internal buffer exceeds {} bytes",
                    MAX_BUFFER_SIZE
                )));
            }
            buf.resize(new_size, 0);
            continue;
        }

        if err != 0 {
            return Err(errors::new_stdio_other_error(format!(
                "user: lookup userid {}: {}",
                uid, err
            )));
        } else {
            break;
        }
    }

    if result.is_null() {
        return Err(super::unknown_user_id_error(uid));
    }

    Ok(build_user(&pwd))
}

fn c_to_string(ptr: *const i8) -> String {
    if ptr.is_null() {
        "".to_string()
    } else {
        let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
        cstr.to_string_lossy().to_string()
    }
}

pub(super) fn build_user(pwd: &libc::passwd) -> User {
    let mut name = c_to_string(pwd.pw_gecos);
    // The pw_gecos field isn't quite standardized. Some docs
    // say: "It is expected to be a comma separated list of
    // personal data where the first item is the full name of the
    // user."
    let (cut_name, _, found) = strings::cut(&name, ",");
    if found {
        name = cut_name.to_string();
    }
    User {
        uid: format!("{}", pwd.pw_uid),
        gid: format!("{}", pwd.pw_gid),
        username: c_to_string(pwd.pw_name),
        name,
        home_dir: c_to_string(pwd.pw_dir),
    }
}

// fn lookupGroup(groupname string) (*Group, error) {
// 	var grp _C_struct_group
// 	var found bool

// 	cname := make([]byte, len(groupname)+1)
// 	copy(cname, groupname)

// 	err := retryWithBuffer(groupBuffer, fn(buf []byte) syscall.Errno {
// 		var errno syscall.Errno
// 		grp, found, errno = _C_getgrnam_r((*_C_char)(unsafe.Pointer(&cname[0])),
// 			(*_C_char)(unsafe.Pointer(&buf[0])), _C_size_t(len(buf)))
// 		return errno
// 	})
// 	if err != nil {
// 		return nil, fmt.Errorf("user: lookup groupname %s: %v", groupname, err)
// 	}
// 	if !found {
// 		return nil, UnknownGroupError(groupname)
// 	}
// 	return buildGroup(&grp), nil
// }

// fn lookupGroupId(gid string) (*Group, error) {
// 	i, e := strconv.Atoi(gid)
// 	if e != nil {
// 		return nil, e
// 	}
// 	return lookupUnixGid(i)
// }

// fn lookupUnixGid(gid isize) (*Group, error) {
// 	var grp _C_struct_group
// 	var found bool

// 	err := retryWithBuffer(groupBuffer, fn(buf []byte) syscall.Errno {
// 		var errno syscall.Errno
// 		grp, found, errno = _C_getgrgid_r(_C_gid_t(gid),
// 			(*_C_char)(unsafe.Pointer(&buf[0])), _C_size_t(len(buf)))
// 		return syscall.Errno(errno)
// 	})
// 	if err != nil {
// 		return nil, fmt.Errorf("user: lookup groupid %d: %v", gid, err)
// 	}
// 	if !found {
// 		return nil, UnknownGroupIdError(strconv.Itoa(gid))
// 	}
// 	return buildGroup(&grp), nil
// }

// fn buildGroup(grp *_C_struct_group) *Group {
// 	g := &Group{
// 		Gid:  strconv.Itoa(isize(_C_gr_gid(grp))),
// 		Name: _C_GoString(_C_gr_name(grp)),
// 	}
// 	return g
// }

// type bufferKind _C_int

// var (
// 	userBuffer  = bufferKind(_C__SC_GETPW_R_SIZE_MAX)
// 	groupBuffer = bufferKind(_C__SC_GETGR_R_SIZE_MAX)
// )

// fn (k bufferKind) initialSize() _C_size_t {
// 	sz := _C_sysconf(_C_int(k))
// 	if sz == -1 {
// 		// DragonFly and FreeBSD do not have _SC_GETPW_R_SIZE_MAX.
// 		// Additionally, not all Linux systems have it, either. For
// 		// example, the musl libc returns -1.
// 		return 1024
// 	}
// 	if !is_size_reasonable(int64(sz)) {
// 		// Truncate.  If this truly isn't enough, retryWithBuffer will error on the first run.
// 		return MAX_BUFFER_SIZE
// 	}
// 	return _C_size_t(sz)
// }

// // retryWithBuffer repeatedly calls f(), increasing the size of the
// // buffer each time, until f succeeds, fails with a non-ERANGE error,
// // or the buffer exceeds a reasonable limit.
// fn retryWithBuffer(startSize bufferKind, f fn([]byte) syscall.Errno) error {
// 	buf := make([]byte, startSize)
// 	for {
// 		errno := f(buf)
// 		if errno == 0 {
// 			return nil
// 		} else if runtime.GOOS == "aix" && errno+1 == 0 {
// 			// On AIX getpwuid_r appears to return -1,
// 			// not ERANGE, on buffer overflow.
// 		} else if errno != syscall.ERANGE {
// 			return errno
// 		}
// 		newSize := len(buf) * 2
// 		if !is_size_reasonable(int64(newSize)) {
// 			return fmt.Errorf("internal buffer exceeds %d bytes", MAX_BUFFER_SIZE)
// 		}
// 		buf = make([]byte, newSize)
// 	}
// }

const MAX_BUFFER_SIZE: usize = 1 << 20;

fn is_size_reasonable(sz: usize) -> bool {
    sz > 0 && sz <= MAX_BUFFER_SIZE
}
