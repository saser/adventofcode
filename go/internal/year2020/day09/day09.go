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
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	numbers := parse(input)
	c := make(counts, Lookback)
	for _, n := range numbers[:Lookback] {
		c.Push(n)
	}
	for i := Lookback; i < len(numbers); i++ {
		target := numbers[i]
		if !c.TwoSum(target) {
			return fmt.Sprint(target), nil
		}
		c.Push(target)
		c.Pop(numbers[i-Lookback])
	}
	panic("unreachable")
}

type counts map[int64]int

func (c counts) Push(n int64) {
	c[n] = c[n] + 1
}

func (c counts) Pop(n int64) {
	c[n] = c[n] - 1
	if c[n] <= 0 {
		delete(c, n)
	}
}

func (c counts) TwoSum(target int64) bool {
	for base := range c {
		other := target - base
		if other == base {
			continue
		}
		if c[other] > 0 {
			return true
		}
	}
	return false
}
