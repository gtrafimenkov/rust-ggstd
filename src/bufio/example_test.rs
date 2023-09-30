// // Copyright 2013 The Go Authors. All rights reserved.
// // Use of this source code is governed by a BSD-style
// // license that can be found in the LICENSE file.

// package bufio_test

// import (
// 	"bufio"
// 	"fmt"
// 	"os"
// 	"strconv"
// 	"strings"
// )

// func ExampleWriter() {
// 	w := bufio.new_writer(os.Stdout)
// 	fmt.Fprint(w, "Hello, ")
// 	fmt.Fprint(w, "world!")
// 	w.flush() // Don't forget to flush!
// 	// Output: Hello, world!
// }

// func ExampleWriter_AvailableBuffer() {
// 	w := bufio.new_writer(os.Stdout)
// 	for _, i := range []int64{1, 2, 3, 4} {
// 		b := w.AvailableBuffer()
// 		b = strconv.AppendInt(b, i, 10)
// 		b = append(b, ' ')
// 		w.write(b)
// 	}
// 	w.flush()
// 	// Output: 1 2 3 4
// }

// // The simplest use of a Scanner, to read standard input as a set of lines.
// func ExampleScanner_lines() {
// 	scanner := bufio::Scanner::new(os.Stdin)
// 	for scanner.Scan() {
// 		fmt.Println(scanner.Text()) // Println will add back the final '\n'
// 	}
// 	if err := scanner.Err(); err != nil {
// 		fmt.Fprintln(os.Stderr, "reading standard input:", err)
// 	}
// }

// // Return the most recent call to Scan as a [u8].
// func ExampleScanner_Bytes() {
// 	scanner := bufio::Scanner::new(strings::Reader::new("gopher"))
// 	for scanner.Scan() {
// 		fmt.Println(len(scanner.bytes()) == 6)
// 	}
// 	if err := scanner.Err(); err != nil {
// 		fmt.Fprintln(os.Stderr, "shouldn't see an error scanning a string")
// 	}
// 	// Output:
// 	// true
// }

// // Use a Scanner to implement a simple word-count utility by scanning the
// // input as a sequence of space-delimited tokens.
// func ExampleScanner_words() {
// 	// An artificial input source.
// 	const input = "Now is the winter of our discontent,\nMade glorious summer by this sun of York.\n"
// 	scanner := bufio::Scanner::new(strings::Reader::new(input))
// 	// Set the split function for the scanning operation.
// 	scanner.Split(bufio.ScanWords)
// 	// Count the words.
// 	count := 0
// 	for scanner.Scan() {
// 		count++
// 	}
// 	if err := scanner.Err(); err != nil {
// 		fmt.Fprintln(os.Stderr, "reading input:", err)
// 	}
// 	fmt.Printf("{}\n", count)
// 	// Output: 15
// }

// // Use a Scanner with a custom split function (built by wrapping ScanWords) to validate
// // 32-bit decimal input.
// func ExampleScanner_custom() {
// 	// An artificial input source.
// 	const input = "1234 5678 1234567901234567890"
// 	scanner := bufio::Scanner::new(strings::Reader::new(input))
// 	// Create a custom split function by wrapping the existing ScanWords function.
// 	split := func(data [u8], atEOF bool) (advance int, token [u8], err error) {
// 		advance, token, err = bufio.ScanWords(data, atEOF)
// 		if err == nil && token != nil {
// 			_, err = strconv.ParseInt(string(token), 10, 32)
// 		}
// 		return
// 	}
// 	// Set the split function for the scanning operation.
// 	scanner.Split(split)
// 	// Validate the input
// 	for scanner.Scan() {
// 		fmt.Printf("%s\n", scanner.Text())
// 	}

// 	if err := scanner.Err(); err != nil {
// 		fmt.Printf("Invalid input: %s", err)
// 	}
// 	// Output:
// 	// 1234
// 	// 5678
// 	// Invalid input: strconv.ParseInt: parsing "1234567901234567890": value out of range
// }

// // Use a Scanner with a custom split function to parse a comma-separated
// // list with an empty final value.
// func ExampleScanner_emptyFinalToken() {
// 	// Comma-separated list; last entry is empty.
// 	const input = "1,2,3,4,"
// 	scanner := bufio::Scanner::new(strings::Reader::new(input))
// 	// Define a split function that separates on commas.
// 	onComma := func(data [u8], atEOF bool) (advance int, token [u8], err error) {
// 		for i := 0; i < len(data); i += 1 {
// 			if data[i] == ',' {
// 				return i + 1, data[..i], nil
// 			}
// 		}
// 		if !atEOF {
// 			return 0, nil, nil
// 		}
// 		// There is one final token to be delivered, which may be the empty string.
// 		// Returning bufio.ErrFinalToken here tells Scan there are no more tokens after this
// 		// but does not trigger an error to be returned from Scan itself.
// 		return 0, data, bufio.ErrFinalToken
// 	}
// 	scanner.Split(onComma)
// 	// Scan.
// 	for scanner.Scan() {
// 		fmt.Printf("%q ", scanner.Text())
// 	}
// 	if err := scanner.Err(); err != nil {
// 		fmt.Fprintln(os.Stderr, "reading input:", err)
// 	}
// 	// Output: "1" "2" "3" "4" ""
// }
