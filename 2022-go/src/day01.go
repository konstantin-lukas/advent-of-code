package src

import (
	"strconv"
	"strings"
)

func Day01Part1(data string) int {
	data = strings.ReplaceAll(data, "\r\n", "\n")
	elves := strings.Split(data, "\n\n")

	maxCalories := 0
	for _, elf := range elves {
		calories := 0
		for _, str := range strings.Split(elf, "\n") {
			num, err := strconv.Atoi(str)
			if err != nil {
				panic(err)
			}
			calories += num
		}
		if calories > maxCalories {
			maxCalories = calories
		}
	}
	return maxCalories
}
