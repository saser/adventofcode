package day23

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	a := parse(r, part == 2)
	return fmt.Sprint(collatz(a)), nil
}

func parse(r io.Reader, skip bool) uint {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	a := uint(0)
	skips := 1
	if skip {
		a = 1
		skips = 2
	}
loop:
	for sc.Scan() {
		line := sc.Text()
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
