package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

func main() {
	fmt.Printf("The smallest Almanac value is: %d\n", part2(parser()))
}

type Almanac struct {
	seeds []int
	maps  [][]int
}

func parser() Almanac {
	seeds := []int{}
	maps := [][]int{}
	set := []int{}
	file, err := ioutil.ReadFile("input.text")

	if err != nil {
		log.Fatal(err)
	}
	file_str := string(file)
	split_str := strings.Split(file_str, "\n\n")
	for index, section := range split_str {
		if index == 0 {
			str_set := strings.Split(strings.Split(section, ": ")[1], " ")
			for _, digitAsString := range str_set {
				digit, _ := strconv.Atoi(digitAsString)
				seeds = append(seeds, digit)
				_ = seeds
			}
		} else {
			str_set := strings.Split(strings.Replace(strings.Split(section, ":\n")[1], "\n", " ", -1), " ")
			_ = str_set //GO compiler sucking
			for _, digitAsString := range str_set {
				digit, _ := strconv.Atoi(digitAsString)
				set = append(set, digit)
			}
			maps = append(maps, set)
			set = []int{}
		}
	}
	return Almanac{
		seeds,
		maps,
	}
}

func part1(a Almanac) int {
	seeds := a.seeds
	for index := range a.maps {
		seeds = part1_helper(seeds, a.maps[index])
		fmt.Println(seeds)
	}

	smallest := -1
	for _, seed := range seeds {
		if seed < smallest || smallest == -1 {
			smallest = seed
		}
	}
	return smallest
}

func part1_helper(seeds []int, setMap []int) []int {
	out := []int{}
	for _, seed := range seeds {
		boolSet := false

		for index := 0; index < len(setMap); index += 3 {
			if !boolSet && seed >= setMap[index+1] && seed < setMap[index+1]+setMap[index+2] {
				out = append(out, setMap[index+0]-setMap[index+1]+seed)
				boolSet = true
			}
		}
		if !boolSet {
			out = append(out, seed)
		}
		boolSet = false
	}
	return out
}

func part2(a Almanac) int {
	seeds := []int{}
	for index := 0; index < len(a.seeds); index += 2 {
		seeds = append(seeds, a.seeds[index])
		seeds = append(seeds, a.seeds[index]+a.seeds[index+1]-1)
	}
	//for index := range a.maps {
	for index := 0; index < len(a.maps); index += 1 {
		seeds = part2_helper(seeds, a.maps[index])
		//fmt.Println(seeds)
	}

	smallest := -1
	for index := 0; index < len(seeds); index += 2 {
		if seeds[index] < smallest || smallest == -1 {
			smallest = seeds[index]
		}
	}
	// fmt.Print("The Final seed is: ")
	// fmt.Println(seeds)
	return smallest
}

func part2_helper(seeds []int, setMap []int) []int {
	out := []int{}
	for seedIndex := 0; seedIndex < len(seeds); seedIndex += 2 {
		fmt.Println(seeds)
		boolSet := false
		seedHigh := seeds[seedIndex+1]
		seedLow := seeds[seedIndex]
		for mapIndex := 0; mapIndex < len(setMap); mapIndex += 3 {
			mapHigh := setMap[mapIndex+1] + setMap[mapIndex+2] - 1
			mapLow := setMap[mapIndex+1]
			if !boolSet && mapLow <= seedLow && seedHigh <= mapHigh { // Completly in range
				out = append(out, setMap[mapIndex]-mapLow+seedLow)
				out = append(out, setMap[mapIndex]-mapLow+seedHigh)
				boolSet = true
				fmt.Print("1:")
			} else if !boolSet && mapLow <= seedLow && seedHigh > mapHigh && seedLow <= mapHigh { // Left Range overflow
				out = append(out, setMap[mapIndex]-mapLow+seedLow)
				out = append(out, setMap[mapIndex]-mapLow+mapHigh)
				seeds = append(seeds, mapHigh+1)
				seeds = append(seeds, seedHigh)
				boolSet = true
				fmt.Print("2:")
			} else if !boolSet && mapLow > seedLow && seedHigh <= mapHigh && seedHigh >= mapLow { // Right Range overflow
				out = append(out, setMap[mapIndex]-mapLow+mapLow)
				out = append(out, setMap[mapIndex]-mapLow+seedHigh)
				seeds = append(seeds, seedLow)
				seeds = append(seeds, mapLow-1)
				boolSet = true
				fmt.Print("3:")
			} else if !boolSet && mapLow > seedLow && seedHigh > mapHigh { // Both Left and Right Range Overflow
				out = append(out, mapLow)
				out = append(out, mapHigh)
				seeds = append(seeds, seedLow)
				seeds = append(seeds, mapLow-1)
				seeds = append(seeds, mapHigh+1)
				seeds = append(seeds, seedHigh)
				boolSet = true
				fmt.Print("4:")
			}

		}

		if !boolSet {
			out = append(out, seeds[seedIndex])
			out = append(out, seeds[seedIndex+1])
		}

	}
	return out
}
