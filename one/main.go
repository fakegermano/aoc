package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	count := 0
	previous := -1
	for scanner.Scan() {
		var current int
		fmt.Sscan(scanner.Text(), &current)
		if previous != -1 && current > previous {
			count += 1
		}
		previous = current
	}
	fmt.Println(count)
}
