package main

import (
	. "aoc2022/src"
	"fmt"
)

func main() {
	fmt.Println("Hello Gophers!")
	test()
}

func test() {
	results := [][]int{
		{Day01Part1(""), 24000},
	}
	for i, value := range results {
		if value[0] != value[1] {
			panic(fmt.Sprintf("Day %d part %d returned an incorrect result", (i/2)+1, (i%2)+1))
		}
	}
}
