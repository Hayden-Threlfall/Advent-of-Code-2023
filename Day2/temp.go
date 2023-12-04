package main

import (
	"bufio"
	"fmt"
	"strings"
)

func solve(scanner *bufio.Scanner) {
	shape := [3]int{1, 2, 3}
	outcome := [3][3]int{
		{3, 6, 0},
		{0, 3, 6},
		{6, 0, 3},
	}
	move := [3][3]int{
		{2, 0, 1},
		{0, 1, 2},
		{1, 2, 0},
	}
	score := 0
	for scanner.Scan() {
		moves := strings.SplitN(scanner.Text(), " ", 2)
		opponent := int(moves[0][0]) - int('A')
		me := move[opponent][int(moves[1][0])-int('X')]
		score += shape[me] + outcome[opponent][me]
	}
	fmt.Println(score)
}
