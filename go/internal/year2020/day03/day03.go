package day03

import (
	"bytes"
	"fmt"
)

type slope struct {
	Right, Down int
}

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	var slopes []slope
	switch part {
	case 1:
		slopes = []slope{
			{Right: 3, Down: 1},
		}
	case 2:
		slopes = []slope{
			{Right: 1, Down: 1},
			{Right: 3, Down: 1},
			{Right: 5, Down: 1},
			{Right: 7, Down: 1},
			{Right: 1, Down: 2},
		}
	}
	grid := []byte(input)
	colCount := bytes.IndexByte(grid, '\n')
	treeCounts := make([]int, len(slopes))
	for i, s := range slopes {
		treeCounts[i] = countTrees(grid, colCount, s)
	}
	return fmt.Sprint(product(treeCounts)), nil
}

func product(ints []int) int {
	switch len(ints) {
	case 0:
		return 0
	case 1:
		return ints[0]
	default:
		p := ints[0]
		for _, i := range ints[1:] {
			p *= i
		}
		return p
	}
}

func countTrees(grid []byte, colCount int, s slope) int {
	treeCount := 0
	skipRow := colCount + 1 // pass over all columns, and the newline
	row, col := 0, 0
	i := 0
	for {
		i += skipRow*s.Down + s.Right
		row += s.Down
		newCol := (col + s.Right) % colCount
		if newCol < col {
			i -= skipRow - 1 // back up one line, and take the newline into account
		}
		col = newCol

		if i >= len(grid) {
			break
		}

		if grid[i] == '#' {
			treeCount++
		}
	}
	return treeCount
}
