// This example shows fastrand pseudorandom number generator.
//
// fastrand is used internally in Go standard library, but not exposed
// to the users of the library.
//
// In rust-ggstd it is exposed as a number of functions in ggstd::runtime.
// On first use fastrand is seeded with random data provided by the operating
// system.
//
// fastrand is much faster than the global math::rand functions and slighly
// faster than math::rand::Rand:
//
//   $ cargo run --example fastrand --release
//     fastrand:              19 ms
//     math::rand global:    352 ms
//     math::rand::Rand:      20 ms

use ggstd::math::rand;

fn main() {
    println!("{:08x}", ggstd::runtime::fastrand());
    println!("{:08x}", ggstd::runtime::fastrand());
    println!("{:16x}", ggstd::runtime::fastrand64());
    println!("{:16x}", ggstd::runtime::fastrand64());
    println!("{:x}", ggstd::runtime::fastrandu());
    println!("{:x}", ggstd::runtime::fastrandu());
    println!("{}", ggstd::runtime::fastrandn(100));
    println!("{}", ggstd::runtime::fastrandn(100));
    println!();

    // testing speed
    {
        use std::time::Instant;

        let count = 10_000_000;

        {
            let start_time = Instant::now();
            for _ in 0..count {
                ggstd::runtime::fastrandn(1_000_000_000);
            }
            let elapsed = Instant::now().duration_since(start_time);
            println!("fastrand:          {:6} ms", elapsed.as_millis());
        }

        {
            let start_time = Instant::now();
            for _ in 0..count {
                rand::int31n(1_000_000_000);
            }
            let elapsed = Instant::now().duration_since(start_time);
            println!("math::rand global: {:6} ms", elapsed.as_millis());
        }

        {
            let start_time = Instant::now();
            let mut r = rand::Rand::new(rand::new_source(123));
            for _ in 0..count {
                r.int31n(1_000_000_000);
            }
            let elapsed = Instant::now().duration_since(start_time);
            println!("math::rand::Rand:  {:6} ms", elapsed.as_millis());
        }
    }
}
