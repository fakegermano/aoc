package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	line := 0
	numbers := make([]int, 0)
	boards := make([][5][5]int, 0)
	blank := [5][5]int{
		{0, 0, 0, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 0, 0}}
	i := 0
	current := -1
	for scanner.Scan() {
		value := scanner.Text()
		if line == 0 {
			raw := strings.Split(value, ",")
			for _, n := range raw {
				n_parsed, _ := strconv.ParseInt(n, 10, 64)
				numbers = append(numbers, int(n_parsed))
			}
		} else {
			if value == "" {
				boards = append(boards, blank)
				current = len(boards) - 1
				i = 0
				continue
			} else {
				raw := strings.Fields(value)
				for j, n := range raw {
					n_parsed, _ := strconv.ParseInt(n, 10, 64)
					boards[current][i][j] = int(n_parsed)
				}
				i += 1
			}
		}
		line += 1
	}
	fmt.Println(numbers)
	fmt.Println(boards)
	// TODO this is a inneficient way
	// we should only keep track of which boards have the number
	// and not the shape of the boards!
}
