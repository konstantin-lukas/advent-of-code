package main

import (
	. "aoc2022/src"
	"fmt"
	"os"
)

func main() {
	test()
}

func test() {

	var results [][]int
	for i := 1; i < 2; i++ {
		var bytes, err = os.ReadFile(fmt.Sprintf("example/day%02d", i))
		if err != nil {
			panic("Could not read file")
		}
		results = append(results, []int{Day01Part1(string(bytes)), 24000})
	}

	for i, value := range results {
		if value[0] != value[1] {
			panic(fmt.Sprintf(
				"Day %d part %d returned an incorrect result. Expected %d, got %d",
				(i/2)+1,
				(i%2)+1,
				value[1],
				value[0],
			))
		} else {
			fmt.Printf("Day %d part %d: %d", (i/2)+1, (i%2)+1, value[1])
		}
	}
}
