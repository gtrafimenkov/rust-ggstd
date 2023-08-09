use ggstd::crypto::sha256;
use ggstd::encoding::hex;

fn main() {
    let msg = "hello world";
    let hash = sha256::sum256(msg.as_bytes());
    let hash_str = hex::encode_to_string(&hash);
    println!("message: {}", msg);
    println!("hash:    {}", hash_str);
}
