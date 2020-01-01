package day23

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	a, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 23, part %d: %w", part, err)
	}
	return fmt.Sprint(collatz(a)), nil
}

func parse(r io.Reader) (uint, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	if ok := sc.Scan(); !ok {
		return 0, fmt.Errorf("parse: %w", sc.Err())
	}
	parts := strings.SplitN(sc.Text(), " ", 2)
	ops := strings.Split(parts[1], ", ")
	offset, err := strconv.Atoi(ops[1])
	if err != nil {
		return 0, fmt.Errorf("parse: %w", err)
	}
	a := uint(0)
	i := 0
	for i < offset-1 && sc.Scan() {
		switch strings.Split(sc.Text(), " ")[0] {
		case "inc":
			a++
		case "tpl":
			a *= 3
		}
		i++
	}
	return a, nil
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
