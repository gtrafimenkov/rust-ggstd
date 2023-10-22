use ggstd::os::TempDir;
use ggstd::os::TempFile;

fn main() {
    let tf = TempFile::new("ggstd-*.txt").unwrap();
    println!("{}", tf.path.to_string_lossy());

    let td = TempDir::new("ggstd-").unwrap();
    println!("{}", td.path.to_string_lossy());

    let tf = TempFile::new_in_dir(&td.path, "ggstd-*.txt").unwrap();
    println!("{}", tf.path.to_string_lossy());
}
