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
    w.write(b"Hello, ")?;
    w.write(b"world!")?;
    w.flush()?; // Don't forget to flush!
    Ok(())
}
