package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	window := []int{-1, -1, -1, -1}
	count := 0
	i := 0
	for scanner.Scan() {
		var current int
		fmt.Sscan(scanner.Text(), &current)
		window[i%4] = current
		if i >= 3 {
			sumCurrent := 0
			sumPrevious := 0
			switch i % 4 {
			case 0:
				sumCurrent = window[0] + window[3] + window[2]
				sumPrevious = window[3] + window[2] + window[1]
			case 1:
				sumCurrent = window[1] + window[0] + window[3]
				sumPrevious = window[0] + window[3] + window[2]
			case 2:
				sumCurrent = window[2] + window[1] + window[0]
				sumPrevious = window[1] + window[0] + window[3]
			case 3:
				sumCurrent = window[3] + window[2] + window[1]
				sumPrevious = window[2] + window[1] + window[0]
			}
			if sumCurrent > sumPrevious {
				fmt.Println(sumPrevious, "increased")
				count += 1
			} else if sumCurrent < sumPrevious {
				fmt.Println(sumPrevious, "decreased")
			} else {
				fmt.Println(sumPrevious, "no change")
			}
		}
		i += 1
	}
	fmt.Println(count)
}
