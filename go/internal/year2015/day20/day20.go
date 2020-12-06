package day20

import (
	"bufio"
	"fmt"
	"strconv"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	if ok := sc.Scan(); !ok {
		return "", fmt.Errorf("year 2015, day 20, part %d: %w", part, sc.Err())
	}
	line := sc.Text()
	target, err := strconv.Atoi(line)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 20, part %d: %w", part, err)
	}
	var presents, maxHouses int
	switch part {
	case 1:
		presents = 10
		maxHouses = 0
	case 2:
		presents = 11
		maxHouses = 50
	}
	answer, ok := firstHouse(target, presents, maxHouses)
	if !ok {
		return "", fmt.Errorf("year 2015, day 20, part %d: no solution found", part)
	}
	return fmt.Sprint(answer), nil
}

func firstHouse(target int, presents int, maxHouses int) (int, bool) {
	houses := make([]int, target+1)
	limit := target / presents
	if limit == 0 {
		limit = 1
	}
	housesLeft := make([]int, limit+1)
	for i, _ := range housesLeft {
		housesLeft[i] = maxHouses
	}
outer:
	for i := 1; i <= limit; i++ {
		for j := i; j <= limit; j += i {
			houses[j] += i * presents
			if maxHouses != 0 {
				housesLeft[i]--
				if housesLeft[i] == 0 {
					continue outer
				}
			}
		}
	}
	for i, house := range houses {
		if house >= target {
			return i, true
		}
	}
	return 0, false
}
