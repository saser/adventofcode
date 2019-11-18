package day17

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	parts, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 17, part 1: %w", err)
	}
	fmt.Println(parts)
	return "", errors.New("not implemented yet")
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
