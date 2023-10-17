use ggstd::hash::{crc32, Hash32};
use std::io::Write;

fn main() {
    {
        let mut crc = crc32::new_ieee();
        crc.write_all("hello".as_bytes()).unwrap();
        println!("{:08x}", crc.sum32());
    }

    {
        let crc_table = crc32::make_table(crc32::PredefinedPolynomials::IEEE as u32);
        let mut crc = crc32::new(&crc_table);
        crc.write_all("hello".as_bytes()).unwrap();
        println!("{:08x}", crc.sum32());
    }
}
