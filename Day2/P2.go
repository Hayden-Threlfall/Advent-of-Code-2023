package main

// IF YOU ARE SEEING THIS
// This is my first time using Golang... I apologize in advance

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	scanner := bufio.NewScanner(file)
	b := 0
	bt := 0
	g := 0
	gt := 0
	r := 0
	rt := 0

	bmax := 0
	gmax := 0
	rmax := 0

	total := 0
	game := 1
	for scanner.Scan() { // internally, it advances token based on sperator

		//stuff := "Game 97: 3 green, 3 blue; 5 green, 3 blue, 1 red; 5 green, 1 red, 3 blue; 1 green, 1 red, 2 blue; 2 green, 2 blue, 3 red"
		for _, line := range strings.Split(scanner.Text(), ";") {
			split := strings.Split(line, " blue")
			for _, s := range split {
				if len(s) > 0 {
					if _, err := strconv.Atoi(string(s[len(s)-1])); err == nil {
						bt, _ = strconv.Atoi(string(s[len(s)-1]))
						b += bt
					}
					if _, err := strconv.Atoi(string(s[len(s)-2])); err == nil {
						bt, _ = strconv.Atoi(string(s[len(s)-2]))
						b += bt * 10
					}
				}
			}

			split = strings.Split(line, " green")
			for _, s := range split {
				if len(s) > 0 {

					if _, err := strconv.Atoi(string(s[len(s)-1])); err == nil {
						gt, _ = strconv.Atoi(string(s[len(s)-1]))
						g += gt
						//println(gt)
					}
					if _, err := strconv.Atoi(string(s[len(s)-2])); err == nil {
						gt, _ = strconv.Atoi(string(s[len(s)-2]))
						g += gt * 10
					}
				}
			}

			split = strings.Split(line, " red")
			for _, s := range split {
				if len(s) > 0 {

					if _, err := strconv.Atoi(string(s[len(s)-1])); err == nil {
						rt, _ = strconv.Atoi(string(s[len(s)-1]))
						r += rt
					}
					if _, err := strconv.Atoi(string(s[len(s)-2])); err == nil {
						rt, _ = strconv.Atoi(string(s[len(s)-2]))
						r += rt * 10
					}
				}
			}
			if bmax < b {
				bmax = b
			}
			if gmax < g {
				gmax = g
			}
			if rmax < r {
				rmax = r
			}
			// if b > 14 || r > 12 || g > 13 {
			// 	fail = true
			// 	println("failed")
			// }
			b = 0
			g = 0
			r = 0

		}
		total += bmax * gmax * rmax
		bmax = 0
		gmax = 0
		rmax = 0
		game++
	}
	fmt.Printf("%d \n", total)
}
