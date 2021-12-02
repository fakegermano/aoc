package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	horizontal := 0
	depth := 0
	aim := 0
	for scanner.Scan() {
		var command string
		var value int
		fmt.Sscan(scanner.Text(), &command, &value)

		switch command {
		case "forward":
			horizontal += value
			depth += aim * value
		case "up":
			aim -= value
		case "down":
			aim += value
		}
	}
	fmt.Println(horizontal * depth)
}
