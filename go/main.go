package main

import (
	"fmt"
)

func main() {
	output, err := ls()

	if err != nil {
		fmt.Println("Error")
		return
	}

	fmt.Println(output)
}
