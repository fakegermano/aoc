package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	freq := make(map[int]int)
	for scanner.Scan() {
		var value string
		fmt.Sscan(scanner.Text(), &value)

		for i, c := range value {
			if _, ok := freq[len(value)-i-1]; !ok {
				freq[len(value)-i-1] = 0
			}
			switch c {
			case '0':
				freq[len(value)-i-1] -= 1
			case '1':
				freq[len(value)-i-1] += 1
			}
		}
	}
	gamma := 0
	epsilon := 0
	for k, v := range freq {
		b := 0
		nb := 1
		if v >= 0 {
			b = 1
			nb = 0
		}
		gamma += (1 << k) * b
		epsilon += (1 << k) * nb
		//fmt.Println(k, v, b)
	}
	fmt.Println(gamma * epsilon)
}
