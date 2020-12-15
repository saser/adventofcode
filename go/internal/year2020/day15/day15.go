package day15

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func parse(input string) []int {
	parts := strings.Split(strings.TrimSpace(input), ",")
	numbers := make([]int, len(parts))
	for i, part := range parts {
		number, err := strconv.Atoi(part)
		if err != nil {
			panic(err)
		}
		numbers[i] = number
	}
	return numbers
}

type entry struct {
	prev1, prev2 int
	count        int
}

type spoken map[int]entry

func (s spoken) Add(number, idx int) {
	e, ok := s[number]
	if ok {
		e.prev2 = e.prev1
	}
	e.prev1 = idx
	e.count++
	s[number] = e
}

func (s spoken) Next(previous int) int {
	e := s[previous]
	if e.count == 1 {
		return 0
	}
	return e.prev1 - e.prev2
}

func solve(input string, part int) (string, error) {
	numbers := parse(input)
	s := make(spoken)
	for i, number := range numbers {
		s.Add(number, i)
	}
	previous := numbers[len(numbers)-1]
	var target int
	switch part {
	case 1:
		target = 2020
	case 2:
		target = 30000000
	}
	for i := len(numbers) + 1; i <= target; i++ {
		next := s.Next(previous)
		s.Add(next, i-1)
		previous = next
	}
	return fmt.Sprint(previous), nil
}
