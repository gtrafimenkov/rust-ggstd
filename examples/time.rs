

fn main() {
    let t = ggstd::time::unix(0, 0);
    println!(
        "epoch start: {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        t.year(),
        t.month(),
        t.day(),
        t.hour(),
        t.minute(),
        t.second()
    );

    let t = ggstd::time::now();
    println!(
        "time now:    {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        t.year(),
        t.month(),
        t.day(),
        t.hour(),
        t.minute(),
        t.second()
    );
}
