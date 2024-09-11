// Copyright 2023 The rust-ggstd authors.
// SPDX-License-Identifier: 0BSD

package main

import (
	"fmt"
	"os"
)

func main() {
	hostname, err := os.Hostname()
	if err != nil {
		fmt.Println("Error:", err)
		return
	}
	fmt.Println("Hostname:", hostname)
}
