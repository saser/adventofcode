package day17

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strconv"

	"github.com/Saser/adventofcode/internal/solution"
)

func Part1(target int) solution.Solution {
	return func(r io.Reader) (string, error) {
		parts, err := parse(r)
		if err != nil {
			return "", fmt.Errorf("year 2015, day 17, part 1: %w", err)
		}
		proper := combinations(target, parts)
		return fmt.Sprint(len(proper)), nil
	}
}

func Part2(target int) solution.Solution {
	return func(r io.Reader) (string, error) {
		return "", errors.New("not implemented yet")
	}
}

func parse(r io.Reader) ([]int, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	parts := make([]int, 0)
	for sc.Scan() {
		part, err := strconv.Atoi(sc.Text())
		if err != nil {
			return nil, fmt.Errorf("parse: invalid line: %s", sc.Text())
		}
		parts = append(parts, part)
	}
	return parts, nil
}

func combinations(target int, parts []int) (proper [][]int) {
	if target < 0 {
		return [][]int{}
	}
	i := parts[0]
	if i == target {
		proper = append(proper, []int{i})
	}
	if len(parts) == 1 {
		return
	}
	rest := parts[1:]
	for _, combination := range combinations(target-i, rest) {
		proper = append(proper, append([]int{i}, combination...))
	}
	proper = append(proper, combinations(target, rest)...)
	return
}
