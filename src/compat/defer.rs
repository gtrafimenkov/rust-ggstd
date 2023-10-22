/// Defer allows to run a piece of code when a specific scope
/// is finished.  It is somewhat similar to Go's defer statement,
/// but Go's defer runs at the function end, this Defer runs
/// at the scope exit.
///
/// Example:
///   let _d = Defer(|| println!("finishing 1"));
///   let _d = Defer(|| println!("finishing 2"));
pub struct Defer<F: Fn()>(pub F);

impl<F: Fn()> Drop for Defer<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

/// DeferDirRemoval runs recursive directory removal when the
/// object goes out of the scope.  Errors during removal are ignored.
///
/// Example:
///   let _d = DeferDirRemoval::new("/tmp/foo-bar-baz");
pub struct DeferDirRemoval<'a> {
    path: &'a str,
}

impl<'a> DeferDirRemoval<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }
}

impl<'a> Drop for DeferDirRemoval<'a> {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(self.path);
    }
}
