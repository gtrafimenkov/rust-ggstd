// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2011 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// const (
// 	PathSeparator     = '\\' // OS-specific path separator
// 	PathListSeparator = ';'  // OS-specific path list separator
// )

/// is_path_separator reports whether c is a directory separator character.
pub fn is_path_separator(c: char) -> bool {
    c == std::path::MAIN_SEPARATOR || c == '/'
}

// // basename removes trailing slashes and the leading
// // directory name and drive letter from path name.
// fn basename(name string) string {
// 	// Remove drive letter
// 	if len(name) == 2 && name[1] == ':' {
// 		name = "."
// 	} else if len(name) > 2 && name[1] == ':' {
// 		name = name[2:]
// 	}
// 	i := len(name) - 1
// 	// Remove trailing slashes
// 	for ; i > 0 && (name[i] == '/' || name[i] == '\\'); i-- {
// 		name = name[:i]
// 	}
// 	// Remove leading directory name
// 	for i--; i >= 0; i-- {
// 		if name[i] == '/' || name[i] == '\\' {
// 			name = name[i+1:]
// 			break
// 		}
// 	}
// 	return name
// }

fn is_abs(path: &str) -> bool {
    std::path::Path::new(path).is_absolute()
    // 	v := volumeName(path)
    // 	if v == "" {
    // 		return false
    // 	}
    // 	path = path[len(v):]
    // 	if path == "" {
    // 		return false
    // 	}
    // 	return is_path_separator(path[0])
}

// fn volumeName(path string) (v string) {
// 	if len(path) < 2 {
// 		return ""
// 	}
// 	// with drive letter
// 	c := path[0]
// 	if path[1] == ':' &&
// 		('0' <= c && c <= '9' || 'a' <= c && c <= 'z' ||
// 			'A' <= c && c <= 'Z') {
// 		return path[:2]
// 	}
// 	// is it UNC
// 	if l := len(path); l >= 5 && is_path_separator(path[0]) && is_path_separator(path[1]) &&
// 		!is_path_separator(path[2]) && path[2] != '.' {
// 		// first, leading `\\` and next shouldn't be `\`. its server name.
// 		for n := 3; n < l-1; n++ {
// 			// second, next '\' shouldn't be repeated.
// 			if is_path_separator(path[n]) {
// 				n++
// 				// third, following something characters. its share name.
// 				if !is_path_separator(path[n]) {
// 					if path[n] == '.' {
// 						break
// 					}
// 					for ; n < l; n++ {
// 						if is_path_separator(path[n]) {
// 							break
// 						}
// 					}
// 					return path[:n]
// 				}
// 				break
// 			}
// 		}
// 	}
// 	return ""
// }

// fn fromSlash(path string) string {
// 	// Replace each '/' with '\\' if present
// 	var pathbuf []byte
// 	var lastSlash int
// 	for i, b := range path {
// 		if b == '/' {
// 			if pathbuf == nil {
// 				pathbuf = make([]byte, len(path))
// 			}
// 			copy(pathbuf[lastSlash:], path[lastSlash:i])
// 			pathbuf[i] = '\\'
// 			lastSlash = i + 1
// 		}
// 	}
// 	if pathbuf == nil {
// 		return path
// 	}

// 	copy(pathbuf[lastSlash:], path[lastSlash:])
// 	return string(pathbuf)
// }

// fn dirname(path string) string {
// 	vol := volumeName(path)
// 	i := len(path) - 1
// 	for i >= len(vol) && !is_path_separator(path[i]) {
// 		i--
// 	}
// 	dir := path[len(vol) : i+1]
// 	last := len(dir) - 1
// 	if last > 0 && is_path_separator(dir[last]) {
// 		dir = dir[:last]
// 	}
// 	if dir == "" {
// 		dir = "."
// 	}
// 	return vol + dir
// }

// // This is set via go:linkname on runtime.canUseLongPaths, and is true when the OS
// // supports opting into proper long path handling without the need for fixups.
// var canUseLongPaths bool

/// fix_long_path returns the extended-length (\\?\-prefixed) form of
/// path when needed, in order to avoid the default 260 character file
/// path limit imposed by Windows. If path is not easily converted to
/// the extended-length form (for example, if path is a relative path
/// or contains .. elements), or is short enough, fix_long_path returns
/// path unmodified.
///
/// See [https://msdn.microsoft.com/en-us/library/windows/desktop/aa365247(v=vs.85).aspx#maxpath]
///
/// Rust standard library has `std::fs::canonicalize` which is somewhat similar,
/// but it doesn't work for paths which don't point to actual files on disk.
pub fn fix_long_path(path: &str) -> String {
    // 	if canUseLongPaths {
    // 		return path
    // 	}
    // Do nothing (and don't allocate) if the path is "short".
    // Empirically (at least on the Windows Server 2013 builder),
    // the kernel is arbitrarily okay with < 248 bytes. That
    // matches what the docs above say:
    // "When using an API to create a directory, the specified
    // path cannot be so long that you cannot append an 8.3 file
    // name (that is, the directory name cannot exceed MAX_PATH
    // minus 12)." Since MAX_PATH is 260, 260 - 12 = 248.
    //
    // The MSDN docs appear to say that a normal path that is 248 bytes long
    // will work; empirically the path must be less then 248 bytes long.
    if path.len() < 248 {
        // Don't fix. (This is how Go 1.7 and earlier worked,
        // not automatically generating the \\?\ form)
        return path.to_string();
    }

    // The extended form begins with \\?\, as in
    // \\?\c:\windows\foo.txt or \\?\UNC\server\share\foo.txt.
    // The extended form disables evaluation of . and .. path
    // elements and disables the interpretation of / as equivalent
    // to \. The conversion here rewrites / to \ and elides
    // . elements as well as trailing or duplicate separators. For
    // simplicity it avoids the conversion entirely for relative
    // paths or paths containing .. elements. For now,
    // \\server\share paths are not converted to
    // \\?\UNC\server\share paths because the rules for doing so
    // are less well-specified.
    if path.len() >= 2 && &path[..2] == "\\\\" {
        // Don't canonicalize UNC paths.
        return path.to_string();
    }
    if !is_abs(path) {
        // Relative path
        return path.to_string();
    }

    let prefix = r"\\?";

    let mut pathbuf = String::with_capacity(prefix.len() + path.len() + r"\".len());
    pathbuf.push_str(prefix);
    let pathchars: Vec<char> = path.chars().collect();
    let n = pathchars.len();
    let mut r = 0;
    while r < n {
        // for ch in path.chars() {
        // 		switch {
        if is_path_separator(pathchars[r]) {
            // empty block
            r += 1;
            continue;
        }
        if pathchars[r] == '.' && (r + 1 == n || is_path_separator(pathchars[r + 1])) {
            // /./
            r += 1;
            continue;
        }
        if r + 1 < n
            && pathchars[r] == '.'
            && pathchars[r + 1] == '.'
            && (r + 2 == n || is_path_separator(pathchars[r + 2]))
        {
            // /../ is currently unhandled
            return path.to_string();
        }
        pathbuf.push('\\');
        while r < n && !is_path_separator(pathchars[r]) {
            pathbuf.push(pathchars[r]);
            r += 1;
        }
    }
    // A drive's root directory needs a trailing \
    if pathbuf.len() == r"\\?\c:".len() {
        pathbuf.push('\\');
    }
    pathbuf
}

// // fixRootDirectory fixes a reference to a drive's root directory to
// // have the required trailing slash.
// fn fixRootDirectory(p string) string {
// 	if len(p) == len(`\\?\c:`) {
// 		if is_path_separator(p[0]) && is_path_separator(p[1]) && p[2] == '?' && is_path_separator(p[3]) && p[5] == ':' {
// 			return p + r"\`
// 		}
// 	}
// 	return p
// }
