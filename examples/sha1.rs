use ggstd::crypto::sha1;
use ggstd::encoding::hex;

fn main() {
    let msg = "hello world";
    let hash = sha1::sum(msg.as_bytes());
    println!("message:   {}", msg);
    println!("sha1 hash: {}", hex::encode_to_string(&hash));
}
