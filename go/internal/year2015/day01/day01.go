package day01

import (
	"bufio"
	"fmt"
	"io"
)

func Day01One(r io.Reader) (string, error) {
	return solveDay01(r, 1)
}

func Day01Two(r io.Reader) (string, error) {
	return solveDay01(r, 2)
}

func solveDay01(r io.Reader, part int) (string, error) {
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
