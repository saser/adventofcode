package day23

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	a := parse(input, part == 2)
	return fmt.Sprint(collatz(a)), nil
}

func parse(input string, skip bool) uint {
	a := uint(0)
	skips := 1
	if skip {
		a = 1
		skips = 2
	}
loop:
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		if skips > 0 {
			jumps := []string{"jmp", "jio"}
			for _, jump := range jumps {
				if strings.HasPrefix(line, jump) {
					skips--
					break
				}
			}
			continue
		}
		switch strings.Split(line, " ")[0] {
		case "inc":
			a++
		case "tpl":
			a *= 3
		default:
			break loop
		}
	}
	return a
}

func collatz(a uint) uint {
	b := uint(0)
	for a != 1 {
		if a%2 == 0 {
			a /= 2
		} else {
			a = 3*a + 1
		}
		b++
	}
	return b
}
