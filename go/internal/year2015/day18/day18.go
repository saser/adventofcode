package day18

import (
	"bufio"
	"errors"
	"fmt"
	"io"

	"github.com/Saser/adventofcode/internal/solution"
)

const (
	Iterations = 100
	GridSize   = 100
)

func Part1(iterations int, gridSize int) solution.Solution {
	return func(r io.Reader) (string, error) {
		grid, err := parse(r, gridSize)
		if err != nil {
			return "", fmt.Errorf("year 2015, day 18, part 1: %w", err)
		}
		fmt.Println(grid)
		return "", errors.New("not implemented yet")
	}
}

func parse(r io.Reader, gridSize int) ([][]bool, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	grid := make([][]bool, 0, gridSize)
	for sc.Scan() {
		row := make([]bool, 0, gridSize)
		for _, r := range sc.Text() {
			var state bool
			switch r {
			case '.':
				state = false
			case '#':
				state = true
			default:
				return grid, fmt.Errorf("parse: invalid rune: %s", string(r))
			}
			row = append(row, state)
		}
		grid = append(grid, row)
	}
	return grid, nil
}
