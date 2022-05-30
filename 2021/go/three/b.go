package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func most_freq(input []string, i int) int {
	freq := 0
	for _, in := range input {
		switch in[i] {
		case '0':
			freq -= 1
		case '1':
			freq += 1
		}
	}
	return freq
}

func filter(input []string, i int, c byte) []string {
	filtered := make([]string, 0)
	for _, in := range input {
		if in[i] != c {
			filtered = append(filtered, in)
		}
	}
	return filtered
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	input := make([]string, 0)
	for scanner.Scan() {
		var value string
		fmt.Sscan(scanner.Text(), &value)
		input = append(input, value)
	}
	o2 := make([]string, len(input))
	copy(o2, input)
	for i := 0; i < len(input[0]); i++ {
		if len(o2) == 1 {
			break
		}
		//fmt.Println(i, o2)
		c := byte('0')
		if most_freq(o2, i) < 0 {
			c = byte('1')
		}
		o2 = filter(o2, i, c)
		//fmt.Println(i, o2)
	}
	o2_, _ := strconv.ParseUint(o2[0], 2, 64)
	//fmt.Println(o2_)
	co2 := make([]string, len(input))
	copy(co2, input)
	for i := 0; i < len(input[0]); i++ {
		if len(co2) == 1 {
			break
		}
		//fmt.Println(i, co2)
		c := byte('0')
		if most_freq(co2, i) >= 0 {
			c = byte('1')
		}
		co2 = filter(co2, i, c)
		//fmt.Println(i, co2)
	}
	co2_, _ := strconv.ParseUint(co2[0], 2, 64)
	//fmt.Println(co2_)

	fmt.Println(o2_ * co2_)
}
