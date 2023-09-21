// Copyright 2023 The rust-ggstd authors. All rights reserved.
// Copyright 2015 The Go Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use ggstd::compat;
use ggstd::strconv;

fn main() {
    // ExampleAppendBool();
    // ExampleAppendFloat();
    // ExampleAppendInt();
    example_append_quote();
    // ExampleAppendQuoteRune();
    // ExampleAppendQuoteRuneToASCII();
    // ExampleAppendQuoteToASCII();
    // ExampleAppendUint();
    // ExampleAtoi();
    // ExampleCanBackquote();
    // ExampleFormatBool();
    // ExampleFormatFloat();
    // ExampleFormatInt();
    // ExampleFormatUint();
    example_is_graphic();
    example_is_print();
    // ExampleItoa();
    // ExampleParseBool();
    // ExampleParseFloat();
    // ExampleParseInt();
    // ExampleParseUint();
    example_quote();
    // ExampleQuoteRune();
    // ExampleQuoteRuneToASCII();
    // ExampleQuoteRuneToGraphic();
    example_quote_to_ascii();
    example_quote_to_graphic();
    // ExampleUnquote();
    // ExampleUnquoteChar();
    // ExampleNumError();
}

// fn ExampleAppendBool() {
// 	let b = []byte("bool:");
// 	b = strconv::AppendBool(b, true)
// 	println!("{}, compat::string(&b));

// 	// Output:
// 	// bool:true
// }

// fn ExampleAppendFloat() {
// 	let b32 = []byte("float32:");
// 	b32 = strconv::AppendFloat(b32, 3.1415926535, 'E', -1, 32)
// 	println!("{}, string(b32));

// 	let b64 = []byte("float64:");
// 	b64 = strconv::AppendFloat(b64, 3.1415926535, 'E', -1, 64)
// 	println!("{}, string(b64));

// 	// Output:
// 	// float32:3.1415927E+00
// 	// float64:3.1415926535E+00
// }

// fn ExampleAppendInt() {
// 	let b10 = []byte("int (base 10):");
// 	b10 = strconv::AppendInt(b10, -42, 10)
// 	println!("{}, string(b10));

// 	let b16 = []byte("int (base 16):");
// 	b16 = strconv::AppendInt(b16, -42, 16)
// 	println!("{}, string(b16));

// 	// Output:
// 	// int (base 10):-42
// 	// int (base 16):-2a
// }

fn example_append_quote() {
    let mut b = b"quote:".to_vec();
    strconv::append_quote(&mut b, r#""Fran & Freddie's Diner""#);
    println!("{}", compat::string(&b));

    // Output:
    // quote:"\"Fran & Freddie's Diner\""
}

// fn ExampleAppendQuoteRune() {
// 	let b = []byte("rune:");
// 	b = strconv::AppendQuoteRune(b, '☺')
// 	println!("{}, compat::string(&b));

// 	// Output:
// 	// rune:'☺'
// }

// fn ExampleAppendQuoteRuneToASCII() {
// 	let b = []byte("rune (ascii):");
// 	b = strconv::AppendQuoteRuneToASCII(b, '☺')
// 	println!("{}, compat::string(&b));

// 	// Output:
// 	// rune (ascii):'\u263a'
// }

// fn ExampleAppendQuoteToASCII() {
// 	let b = []byte("quote (ascii):");
// 	b = strconv::append_quote_to_ascii(b, r#""Fran & Freddie's Diner""#)
// 	println!("{}, compat::string(&b));

// 	// Output:
// 	// quote (ascii):"\"Fran & Freddie's Diner\""
// }

// fn ExampleAppendUint() {
// 	let b10 = []byte("uint (base 10):");
// 	b10 = strconv::AppendUint(b10, 42, 10)
// 	println!("{}, string(b10));

// 	let b16 = []byte("uint (base 16):");
// 	b16 = strconv::AppendUint(b16, 42, 16)
// 	println!("{}, string(b16));

// 	// Output:
// 	// uint (base 10):42
// 	// uint (base 16):2a
// }

// fn ExampleAtoi() {
// 	let v = "10";
// 	let if s, err = strconv::Atoi(v); err == nil {;
// 		fmt.Printf("%T, %v", s, s)
// 	}

// 	// Output:
// 	// int, 10
// }

// fn ExampleCanBackquote() {
// 	println!("{}, strconv::CanBackquote("Fran & Freddie's Diner ☺"));
// 	println!("{}, strconv::CanBackquote("`can't backquote thisr#""));

// 	// Output:
// 	// true
// 	// false
// }

// fn ExampleFormatBool() {
// 	let v = true;
// 	let s = strconv::FormatBool(v);
// 	fmt.Printf("%T, %v\n", s, s)

// 	// Output:
// 	// string, true
// }

// fn ExampleFormatFloat() {
// 	let v = 3.1415926535;

// 	let s32 = strconv::FormatFloat(v, 'E', -1, 32);
// 	fmt.Printf("%T, %v\n", s32, s32)

// 	let s64 = strconv::FormatFloat(v, 'E', -1, 64);
// 	fmt.Printf("%T, %v\n", s64, s64)

// 	// Output:
// 	// string, 3.1415927E+00
// 	// string, 3.1415926535E+00
// }

// fn ExampleFormatInt() {
// 	let v = int64(-42);

// 	let s10 = strconv::FormatInt(v, 10);
// 	fmt.Printf("%T, %v\n", s10, s10)

// 	let s16 = strconv::FormatInt(v, 16);
// 	fmt.Printf("%T, %v\n", s16, s16)

// 	// Output:
// 	// string, -42
// 	// string, -2a
// }

// fn ExampleFormatUint() {
// 	let v = uint64(42);

// 	let s10 = strconv::FormatUint(v, 10);
// 	fmt.Printf("%T, %v\n", s10, s10)

// 	let s16 = strconv::FormatUint(v, 16);
// 	fmt.Printf("%T, %v\n", s16, s16)

// 	// Output:
// 	// string, 42
// 	// string, 2a
// }

fn example_is_graphic() {
    let shamrock = strconv::is_graphic('☘');
    println!("{}", shamrock);

    let a = strconv::is_graphic('a');
    println!("{}", a);

    let bel = strconv::is_graphic('\x07');
    println!("{}", bel);

    // Output:
    // true
    // true
    // false
}

fn example_is_print() {
    let c = strconv::is_print('\u{263a}');
    println!("{}", c);

    let bel = strconv::is_print('\x07');
    println!("{}", bel);

    // Output:
    // true
    // false
}

// fn ExampleItoa() {
// 	let i = 10;
// 	let s = strconv::Itoa(i);
// 	fmt.Printf("%T, %v\n", s, s)

// 	// Output:
// 	// string, 10
// }

// fn ExampleParseBool() {
// 	let v = "true";
// 	let if s, err = strconv::ParseBool(v); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}

// 	// Output:
// 	// bool, true
// }

// fn ExampleParseFloat() {
// 	let v = "3.1415926535";
// 	let if s, err = strconv::ParseFloat(v, 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseFloat(v, 64); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseFloat("NaN", 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	// ParseFloat is case insensitive
// 	let if s, err = strconv::ParseFloat("nan", 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseFloat("inf", 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseFloat("+Inf", 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseFloat("-Inf", 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseFloat("-0", 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseFloat("+0", 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}

// 	// Output:
// 	// float64, 3.1415927410125732
// 	// float64, 3.1415926535
// 	// float64, NaN
// 	// float64, NaN
// 	// float64, +Inf
// 	// float64, +Inf
// 	// float64, -Inf
// 	// float64, -0
// 	// float64, 0
// }

// fn ExampleParseInt() {
// 	let v32 = "-354634382";
// 	let if s, err = strconv::ParseInt(v32, 10, 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseInt(v32, 16, 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}

// 	let v64 = "-3546343826724305832";
// 	let if s, err = strconv::ParseInt(v64, 10, 64); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseInt(v64, 16, 64); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}

// 	// Output:
// 	// int64, -354634382
// 	// int64, -3546343826724305832
// }

// fn ExampleParseUint() {
// 	let v = "42";
// 	let if s, err = strconv::ParseUint(v, 10, 32); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}
// 	let if s, err = strconv::ParseUint(v, 10, 64); err == nil {;
// 		fmt.Printf("%T, %v\n", s, s)
// 	}

// 	// Output:
// 	// uint64, 42
// 	// uint64, 42
// }

fn example_quote() {
    // This string literal contains a tab character.
    let s = strconv::quote(r#""Fran & Freddie's Diner	☺""#);
    println!("{}", s);

    // Output:
    // "\"Fran & Freddie's Diner\t☺\""
}

// fn ExampleQuoteRune() {
// 	let s = strconv::QuoteRune('☺');
// 	println!("{}", s);

// 	// Output:
// 	// '☺'
// }

// fn ExampleQuoteRuneToASCII() {
// 	let s = strconv::QuoteRuneToASCII('☺');
// 	println!("{}", s);

// 	// Output:
// 	// '\u263a'
// }

// fn ExampleQuoteRuneToGraphic() {
// 	let s = strconv::QuoteRuneToGraphic('☺');
// 	println!("{}", s);

// 	s = strconv::QuoteRuneToGraphic('\u263a')
// 	println!("{}", s);

// 	s = strconv::QuoteRuneToGraphic('\u000a')
// 	println!("{}", s);

// 	s = strconv::QuoteRuneToGraphic('	') // tab character
// 	println!("{}", s);

// 	// Output:
// 	// '☺'
// 	// '☺'
// 	// '\n'
// 	// '\t'
// }

fn example_quote_to_ascii() {
    // This string literal contains a tab character.
    let s = strconv::quote_to_ascii(r#""Fran & Freddie's Diner	☺""#);
    println!("{}", s);

    // Output:
    // "\"Fran & Freddie's Diner\t\u263a\""
}

fn example_quote_to_graphic() {
    let s = strconv::quote_to_graphic("☺");
    println!("{}", s);

    // This string literal contains a tab character.
    let s = strconv::quote_to_graphic("This is a \u{263a}	\u{000a}");
    println!("{}", s);

    let s = strconv::quote_to_graphic(r#"" This is a ☺ \n ""#);
    println!("{}", s);

    // Output:
    // "☺"
    // "This is a ☺\t\n"
    // "\" This is a ☺ \\n \""
}

// fn ExampleUnquote() {
// 	let s, err = strconv::Unquote("You can't unquote a string without quotes");
// 	fmt.Printf("%q, %v\n", s, err)
// 	s, err = strconv::Unquote("\"The string must be either double-quoted\"")
// 	fmt.Printf("%q, %v\n", s, err)
// 	s, err = strconv::Unquote("`or backquoted.r#"")
// 	fmt.Printf("%q, %v\n", s, err)
// 	s, err = strconv::Unquote("'\u263a'") // single character only allowed in single quotes
// 	fmt.Printf("%q, %v\n", s, err)
// 	s, err = strconv::Unquote("'\u2639\u2639'")
// 	fmt.Printf("%q, %v\n", s, err)

// 	// Output:
// 	// "", invalid syntax
// 	// "The string must be either double-quoted", <nil>
// 	// "or backquoted.", <nil>
// 	// "☺", <nil>
// 	// "", invalid syntax
// }

// fn ExampleUnquoteChar() {
// 	let v, mb, t, err = strconv::UnquoteChar(`\"Fran & Freddie's Diner\"`, '"');
// 	if err != nil {
// 		log.Fatal(err)
// 	}

// 	println!("{}, "value:", string(v));
// 	println!("{}, "multibyte:", mb);
// 	println!("{}, "tail:", t);

// 	// Output:
// 	// value: "
// 	// multibyte: false
// 	// tail: Fran & Freddie's Diner\"
// }

// fn ExampleNumError() {
// 	let str = "Not a number";
// 	let if _, err = strconv::ParseFloat(str, 64); err != nil {;
// 		let e = err.(*strconv::NumError);
// 		println!("{}, "Func:", e.Func);
// 		println!("{}, "Num:", e.Num);
// 		println!("{}, "Err:", e.Err);
// 		println!("{}, err);
// 	}

// 	// Output:
// 	// Func: ParseFloat
// 	// Num: Not a number
// 	// Err: invalid syntax
// 	// strconv::ParseFloat: parsing "Not a number": invalid syntax
// }
