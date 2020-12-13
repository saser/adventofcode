package day13

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

func parse(input string) (int, []int) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	ts, err := strconv.Atoi(lines[0])
	if err != nil {
		panic(err)
	}
	parts := strings.Split(lines[1], ",")
	buses := make([]int, len(parts))
	for i, part := range parts {
		var bus int
		if part == "x" {
			bus = -1
		} else {
			bus, err = strconv.Atoi(part)
			if err != nil {
				panic(err)
			}
		}
		buses[i] = bus
	}
	return ts, buses
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	ts, buses := parse(input)
	minWait := 60
	var bestBus int
	for _, bus := range buses {
		if bus == -1 {
			continue
		}
		next := ((ts / bus) + 1) * bus
		if wait := next - ts; wait < minWait {
			minWait = wait
			bestBus = bus
		}
	}
	return fmt.Sprint(bestBus * minWait), nil
}
