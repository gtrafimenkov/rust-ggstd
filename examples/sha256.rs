use ggstd::crypto::sha256;
use ggstd::encoding::hex;

fn main() {
    let msg = "hello world";
    let hash = sha256::sum256(msg.as_bytes());
    println!("message:     {}", msg);
    println!("sha256 hash: {}", hex::encode_to_string(&hash));

    let hash = sha256::sum224(msg.as_bytes());
    println!("sha224 hash: {}", hex::encode_to_string(&hash));
}
