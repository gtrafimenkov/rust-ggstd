package main

import (
	"fmt"
	"math/rand"
)

func main() {
	rand.Seed(1)
	fmt.Println("global rand with seed(1):")
	fmt.Printf("%x\n", rand.Int())
	fmt.Printf("%x\n", rand.Int())
	fmt.Printf("%x\n", rand.Int())
	fmt.Printf("%x\n", rand.Int())

	fmt.Println()
	fmt.Println("global rand::Float64:")
	fmt.Printf("%v\n", rand.Float64())
	fmt.Printf("%v\n", rand.Float64())
	fmt.Printf("%v\n", rand.Float64())
	fmt.Printf("%v\n", rand.Float64())

	fmt.Println()
	fmt.Println("global rand.NormFloat64:")
	fmt.Printf("%v\n", rand.NormFloat64()*1000)
	fmt.Printf("%v\n", rand.NormFloat64())
	fmt.Printf("%v\n", rand.NormFloat64())
	fmt.Printf("%v\n", rand.NormFloat64())
}
