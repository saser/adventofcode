package day{{.PaddedDay}}

import (
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
	return "", fmt.Errorf("solution not implemented for part %v", part)
}
