package day01

import (
	"bufio"
	"fmt"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanRunes)
	floor := 0
	position := 0
	for sc.Scan() {
		tok := sc.Text()
		position++
		switch tok {
		case "(":
			floor++
		case ")":
			floor--
		case "\n":
			break
		default:
			return "", fmt.Errorf("year 2015, day 01, part 1: invalid token: %s", tok)
		}
		if part == 2 && floor == -1 {
			return fmt.Sprint(position), nil
		}
	}
	return fmt.Sprint(floor), nil
}
