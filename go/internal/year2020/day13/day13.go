package day13

import (
	"fmt"
	"sort"
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

func findBestBus(ts int, buses []int) (int, int) {
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
	return bestBus, minWait
}

type eq struct {
	rem, mod int
}

func crt(eqs []eq) int {
	// Sieve method from:
	// https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving.
	sort.Slice(eqs, func(i, j int) bool {
		return eqs[i].mod > eqs[j].mod
	})
	acc := eqs[0]
	for _, eq := range eqs[1:] {
		for acc.rem%eq.mod != eq.rem {
			acc.rem += acc.mod
		}
		acc.mod *= eq.mod
	}
	return acc.rem
}

func earliest(buses []int) int {
	var eqs []eq
	for i, bus := range buses {
		if bus == -1 {
			continue
		}
		rem := (bus - i) % bus
		if rem < 0 {
			rem += bus
		}
		eqs = append(eqs, eq{
			rem: rem,
			mod: bus,
		})
	}
	return crt(eqs)
}

func solve(input string, part int) (string, error) {
	ts, buses := parse(input)
	switch part {
	case 1:
		bestBus, minWait := findBestBus(ts, buses)
		return fmt.Sprint(bestBus * minWait), nil
	case 2:
		return fmt.Sprint(earliest(buses)), nil
	}
	return "", fmt.Errorf("invalid part: %v", part)
}
