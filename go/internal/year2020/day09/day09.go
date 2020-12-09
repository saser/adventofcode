package day09

import (
	"fmt"
	"strconv"
	"strings"
)

var Lookback = 25

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func parse(input string) []int64 {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	is := make([]int64, len(lines))
	for i, line := range lines {
		n, err := strconv.ParseInt(line, 10, 64)
		if err != nil {
			panic(err)
		}
		is[i] = n
	}
	return is
}

func solve(input string, part int) (string, error) {
	numbers := parse(input)
	var invalid int64
	for i := Lookback; i < len(numbers); i++ {
		target := numbers[i]
		start := i - Lookback
		end := i - 1
		for numbers[start] >= target {
			start++
		}
		for numbers[end] >= target {
			end--
		}
		if !twoSum(target, numbers[start:end+1]) {
			invalid = target
			break
		}
	}
	if part == 1 {
		return fmt.Sprint(invalid), nil
	}
	start, end := 0, 1
	sum := numbers[0]
	for sum != invalid {
		if sum < invalid {
			sum += numbers[end]
			end++
		} else {
			sum -= numbers[start]
			start++
		}
	}
	max := int64(0)
	min := invalid + 1
	for _, n := range numbers[start:end] {
		if n < min {
			min = n
		}
		if n > max {
			max = n
		}
	}
	return fmt.Sprint(min + max), nil
}

func twoSum(target int64, numbers []int64) bool {
	for i, n1 := range numbers {
		for _, n2 := range numbers[i+1:] {
			if n1+n2 == target {
				return true
			}
		}
	}
	return false
}
