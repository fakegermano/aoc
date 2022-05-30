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
	//fmt.Println(numbers)
	//fmt.Println(boards)

	winner := -1
	last_number := -1
	found := false
	for _, number := range numbers {
		if found {
			break
		}
		last_number = number
		for k := range boards {
			for i := 0; i < 5; i++ {
				for j := 0; j < 5; j++ {
					if boards[k][i][j] == number {
						boards[k][i][j] = -1
					}
				}
			}
		}

		for k := range boards {
			if found {
				break
			}
			for i := 0; i < 5; i++ {
				sum := 0
				for j := 0; j < 5; j++ {
					sum += boards[k][i][j]
				}
				if sum == -5 {
					winner = k
					found = true
					break
				}
			}

			if found {
				break
			}
			for j := 0; j < 5; j++ {
				sum := 0
				for i := 0; i < 5; i++ {
					sum += boards[k][i][j]
				}
				if sum == -5 {
					winner = k
					found = true
					break
				}
			}
		}
	}

	sum := 0
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			if boards[winner][i][j] != -1 {
				sum += boards[winner][i][j]
			}
		}
	}
	//fmt.Println(boards[winner])
	//fmt.Println(sum, last_number)
	fmt.Println(sum * last_number)
}
