package day03

import (
	"bufio"
	"fmt"
	"io"
)

type slope struct {
	Right, Down int
}

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
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
	treeCounts := make([]int, len(slopes))
	for i, s := range slopes {
		treeCounts[i] = countTrees(grid, s)
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

func countTrees(grid [][]bool, s slope) int {
	col := 0
	treeCount := 0
	for row := s.Down; row < len(grid); row += s.Down {
		col = (col + s.Right) % len(grid[0])
		if grid[row][col] {
			treeCount++
		}
	}
	return treeCount
}
