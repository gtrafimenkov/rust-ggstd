// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2009 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::unicode::utf8;

fn main() {
    // ExampleDecodeLastRune();
    // ExampleDecodeLastRuneInString();
    example_decode_rune();
    // ExampleDecodeRuneInString();
    example_encode_rune();
    example_encode_rune_out_of_range();
    example_full_rune();
    // ExampleFullRuneInString();
    // ExampleRuneCountInString();
    example_rune_len();
    // ExampleRuneStart();
    // ExampleValid();
    // ExampleValidString();
    // ExampleAppendRune();
}

// fn ExampleDecodeLastRune() {
// 	b := []byte("Hello, 世界")

// 	for b.len() > 0 {
// 		r, size := utf8::DecodeLastRune(b)
// 		fmt.Printf("%c %v\n", r, size)

// 		b = b[:b.len()-size]
// 	}
// 	// Output:
// 	// 界 3
// 	// 世 3
// 	//   1
// 	// , 1
// 	// o 1
// 	// l 1
// 	// l 1
// 	// e 1
// 	// H 1
// }

// fn ExampleDecodeLastRuneInString() {
// 	str := "Hello, 世界"

// 	for len(str) > 0 {
// 		r, size := utf8::DecodeLastRuneInString(str)
// 		fmt.Printf("%c %v\n", r, size)

// 		str = str[:len(str)-size]
// 	}
// 	// Output:
// 	// 界 3
// 	// 世 3
// 	//   1
// 	// , 1
// 	// o 1
// 	// l 1
// 	// l 1
// 	// e 1
// 	// H 1
// }

fn example_decode_rune() {
    let mut b = "Hello, 世界".as_bytes();

    while !b.is_empty() {
        let (r, size) = utf8::decode_rune(b);
        println!("{} {}", r, size);

        b = &b[size..];
    }
    // Output:
    // H 1
    // e 1
    // l 1
    // l 1
    // o 1
    // , 1
    //   1
    // 世 3
    // 界 3
}

// fn ExampleDecodeRuneInString() {
// 	str := "Hello, 世界"

// 	for len(str) > 0 {
// 		r, size := utf8::decode_rune_in_string(str)
// 		fmt.Printf("%c %v\n", r, size)

// 		str = str[size:]
// 	}
// 	// Output:
// 	// H 1
// 	// e 1
// 	// l 1
// 	// l 1
// 	// o 1
// 	// , 1
// 	//   1
// 	// 世 3
// 	// 界 3
// }

fn example_encode_rune() {
    let r = '世';
    let mut buf = [0; 3];

    let n = utf8::encode_rune(&mut buf, r as u32);

    println!("{:?}", buf);
    println!("{}", n);
    // Output:
    // [228 184 150]
    // 3
}

fn example_encode_rune_out_of_range() {
    let runes = &[
        // // Less than 0, out of range.
        0xffffffff_u32,
        // Greater than 0x10FFFF, out of range.
        0x110000,
        // The Unicode replacement character.
        utf8::RUNE_ERROR as u32,
    ];
    for (i, c) in runes.iter().enumerate() {
        let mut buf = [0; 3];
        let size = utf8::encode_rune(&mut buf, *c);
        let (r, _) = utf8::decode_rune(&buf);
        println!("{}: {:?} {} {}", i, buf, r, size);
    }
    // Output:
    // 0: [239 191 189] � 3
    // 1: [239 191 189] � 3
    // 2: [239 191 189] � 3
}

fn example_full_rune() {
    let buf = [228, 184, 150]; // 世
    println!("{}", utf8::full_rune(&buf));
    println!("{}", utf8::full_rune(&buf[..2]));
    // Output:
    // true
    // false
}

// fn ExampleFullRuneInString() {
// 	str := "世"
// 	println!("{}", utf8::FullRuneInString(str));
// 	println!("{}", utf8::FullRuneInString(str[:2]));
// 	// Output:
// 	// true
// 	// false
// }

// fn ExampleRuneCount() {
// 	buf := []byte("Hello, 世界")
// 	println!("{}", "bytes =", len(buf));
// 	println!("{}", "runes =", utf8::rune_count(buf));
// 	// Output:
// 	// bytes = 13
// 	// runes = 9
// }

// fn ExampleRuneCountInString() {
// 	str := "Hello, 世界"
// 	println!("{}", "bytes =", len(str));
// 	println!("{}", "runes =", utf8::RuneCountInString(str));
// 	// Output:
// 	// bytes = 13
// 	// runes = 9
// }

fn example_rune_len() {
    println!("{}", utf8::rune_len('a' as u32));
    println!("{}", utf8::rune_len('界' as u32));
    // Output:
    // 1
    // 3
}

// fn ExampleRuneStart() {
// 	buf := []byte("a界")
// 	println!("{}", utf8::RuneStart(buf[0]));
// 	println!("{}", utf8::RuneStart(buf[1]));
// 	println!("{}", utf8::RuneStart(buf[2]));
// 	// Output:
// 	// true
// 	// true
// 	// false
// }

// fn ExampleValid() {
// 	valid := []byte("Hello, 世界")
// 	invalid := []byte{0xff, 0xfe, 0xfd}

// 	println!("{}", utf8::Valid(valid));
// 	println!("{}", utf8::Valid(invalid));
// 	// Output:
// 	// true
// 	// false
// }

// fn ExampleValidRune() {
// 	valid := 'a'
// 	invalid := rune(0xfffffff)

// 	println!("{}", utf8::ValidRune(valid));
// 	println!("{}", utf8::ValidRune(invalid));
// 	// Output:
// 	// true
// 	// false
// }

// fn ExampleValidString() {
// 	valid := "Hello, 世界"
// 	invalid := string([]byte{0xff, 0xfe, 0xfd})

// 	println!("{}", utf8::ValidString(valid));
// 	println!("{}", utf8::ValidString(invalid));
// 	// Output:
// 	// true
// 	// false
// }

// fn ExampleAppendRune() {
// 	buf1 := utf8::AppendRune(nil, 0x10000)
// 	buf2 := utf8::AppendRune([]byte("init"), 0x10000)
// 	println!("{}", string(buf1));
// 	println!("{}", string(buf2));
// 	// Output:
// 	// 𐀀
// 	// init𐀀
// }
