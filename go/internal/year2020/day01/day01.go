package day01

import (
	"bufio"
	"fmt"
	"io"
	"sort"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	var ints []int
	for sc.Scan() {
		line := sc.Text()
		i, err := strconv.Atoi(line)
		if err != nil {
			return "", fmt.Errorf("part %v: %w", part, err)
		}
		ints = append(ints, i)
	}
	sort.Ints(ints)
	i, j := 0, len(ints)-1
	for sum := ints[i] + ints[j]; sum != 2020; sum = ints[i] + ints[j] {
		if sum < 2020 {
			i++
		} else {
			j--
		}
	}
	product := ints[i] * ints[j]
	return fmt.Sprint(product), nil
}
