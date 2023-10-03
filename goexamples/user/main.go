package main

import (
	"fmt"
	"log"
	"os"
	"os/user"
)

func main() {
	user, err := user.Current()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Uid:      %v\n", user.Uid)
	fmt.Printf("Gid:      %v\n", user.Gid)
	fmt.Printf("Username: %v\n", user.Username)
	fmt.Printf("Name:     %v\n", user.Name)
	fmt.Printf("HomeDir:  %v\n", user.HomeDir)

	fmt.Printf("user_id:  %v\n", os.Geteuid())
}
