use ggstd::image::{self, color, png, Image};
use ggstd::math::rand;

fn main() {
    println!("global rand without seed:");
    println!("{:x}", rand::int());
    println!("{:x}", rand::int());
    println!("{:x}", rand::int());
    println!("{:x}", rand::int());

    #[allow(deprecated)]
    rand::seed(1);

    println!();
    println!("global rand with seed(1):");
    println!("{:x}", rand::int());
    println!("{:x}", rand::int());
    println!("{:x}", rand::int());
    println!("{:x}", rand::int());

    println!();
    println!("rand::Rand::new(rand::new_source(1)):");
    let mut r = rand::Rand::new(rand::new_source(1));
    println!("{:x}", r.int());
    println!("{:x}", r.int());
    println!("{:x}", r.int());
    println!("{:x}", r.int());

    println!();
    println!("global rand::float64:");
    println!("{}", rand::float64());
    println!("{}", rand::float64());
    println!("{}", rand::float64());
    println!("{}", rand::float64());

    println!();
    println!("global rand::norm_float64:");
    println!("{}", rand::norm_float64());
    println!("{}", rand::norm_float64());
    println!("{}", rand::norm_float64());
    println!("{}", rand::norm_float64());

    // create 3 png images with random dot placements
    for seq in 0..3 {
        random_dots_png(seq);
    }

    // showing that random numbers are uniformely distributed
    uniformity_ints_png();
    uniformity_floats_png();

    // unless we want normal distribution
    normal_distr_floats_png();
}

fn uniformity_ints_png() {
    let width = 1000;
    let samples = 100000;
    let mut counters = vec![0_usize; width];
    for _ in 0..samples {
        let x = rand::intn(width as isize) as usize;
        counters[x] += 1;
    }
    save_counters_to_png(width, width / 4, &counters, "uniform-ints.png");
}

fn uniformity_floats_png() {
    let width = 1000;
    let samples = 100000;
    let mut counters = vec![0_usize; width];
    for _ in 0..samples {
        let x = (rand::float32() * width as f32) as usize;
        counters[x] += 1;
    }
    save_counters_to_png(width, width / 4, &counters, "uniform-floats.png");
}

fn normal_distr_floats_png() {
    let width = 1000;
    let samples = 400000;
    let mut counters = vec![0_usize; width];
    for _ in 0..samples {
        let x = (rand::norm_float64() * width as f64 / 3.0 + width as f64) / 2.0;
        if x >= 0.0 && x < width as f64 {
            counters[x as usize] += 1;
        }
    }
    save_counters_to_png(width, width, &counters, "normal-distr-floats.png");
}

fn save_counters_to_png(width: usize, height: usize, counters: &[usize], file_name: &str) {
    let mut img = white_image(width, height);
    (0..width).for_each(|x| {
        img.set(
            x as isize,
            height as isize - counters[x] as isize,
            &color::BLACK,
        );
    });
    let mut f = std::fs::File::create(file_name).unwrap();
    png::encode(&mut f, &img).unwrap();
}

fn random_dots_png(number: usize) {
    let size = 500;
    let count = 5000;
    let mut img = white_image(size, size);
    for _ in 0..count {
        img.set(
            rand::intn(size as isize),
            rand::intn(size as isize),
            &color::BLACK,
        );
    }
    let mut f = std::fs::File::create(format!("random-dots-{:02}.png", number)).unwrap();
    png::encode(&mut f, &img).unwrap();
}

fn white_image(width: usize, height: usize) -> image::Img {
    let mut img = image::Img::new_nrgba(&image::rect(0, 0, width as isize, height as isize));
    for y in 0..height {
        for x in 0..width {
            img.set(x as isize, y as isize, &color::WHITE);
        }
    }
    img
}
