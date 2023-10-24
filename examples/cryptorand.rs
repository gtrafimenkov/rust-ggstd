use ggstd::crypto;

fn main() {
    let mut buf = vec![0; 20];
    crypto::rand::read(&mut buf).unwrap();
    println!("{}", ggstd::encoding::hex::encode_to_string(&buf));
    crypto::rand::read(&mut buf).unwrap();
    println!("{}", ggstd::encoding::hex::encode_to_string(&buf));
}
