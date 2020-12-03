package day03

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
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	var grid [][]bool
	for sc.Scan() {
		line := sc.Text()
		row := make([]bool, len(line))
		for i, r := range line {
			if r == '#' {
				row[i] = true
			}
		}
		grid = append(grid, row)
	}
	return fmt.Sprint(countTrees(grid, 3, 1)), nil
}

func countTrees(grid [][]bool, right, down int) int {
	col := 0
	treeCount := 0
	for row := down; row < len(grid); row += down {
		col = (col + right) % len(grid[0])
		if grid[row][col] {
			treeCount++
		}
	}
	return treeCount
}
