package day18

import (
	"fmt"
	"strings"
)

var (
	Iterations = 100
	GridSize   = 100
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	grid, err := parse(input)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 18, part 1: %w", err)
	}
	grid.part = part
	for i := 0; i < Iterations; i++ {
		grid.step()
	}
	return fmt.Sprint(grid.countOn()), nil
}

func parse(input string) (*grid, error) {
	g := make([][]bool, 0, GridSize)
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		row := make([]bool, 0, GridSize)
		for _, r := range line {
			var state bool
			switch r {
			case '.':
				state = false
			case '#':
				state = true
			default:
				return nil, fmt.Errorf("parse: invalid rune: %s", string(r))
			}
			row = append(row, state)
		}
		g = append(g, row)
	}
	return &grid{
		g: g,
	}, nil
}

func neighbors(row, col int) [8][2]int {
	var coords [8][2]int
	i := 0
	for _, rowI := range []int{row - 1, row, row + 1} {
		for _, colI := range []int{col - 1, col, col + 1} {
			if rowI == row && colI == col {
				continue
			}
			coords[i][0] = rowI
			coords[i][1] = colI
			i++
		}
	}
	return coords
}

type grid struct {
	g    [][]bool
	part int
}

func (g *grid) isCorner(row, col int) bool {
	n := len(g.g) - 1
	firstRow := row == 0
	lastRow := row == n
	firstCol := col == 0
	lastCol := col == n
	return (firstRow && firstCol) || (firstRow && lastCol) || (lastRow && firstCol) || (lastRow && lastCol)
}

func (g *grid) v(row, col int) bool {
	n := len(g.g) - 1
	if row < 0 || row > n || col < 0 || col > n {
		return false
	}
	if g.part == 2 && g.isCorner(row, col) {
		return true
	}
	return g.g[row][col]
}

func (g *grid) step() {
	type update struct {
		row, col int
		state    bool
	}
	updates := make([]update, 0)
	for rowI, row := range g.g {
		for colI, state := range row {
			if g.part == 2 && g.isCorner(rowI, colI) {
				continue
			}
			count := 0
			for _, coord := range neighbors(rowI, colI) {
				if g.v(coord[0], coord[1]) {
					count++
				}
			}
			if state {
				if !(count == 2 || count == 3) {
					updates = append(updates, update{row: rowI, col: colI, state: false})
				}
			} else {
				if count == 3 {
					updates = append(updates, update{row: rowI, col: colI, state: true})
				}
			}
		}
	}
	for _, u := range updates {
		g.g[u.row][u.col] = u.state
	}
}

func (g *grid) countOn() int {
	count := 0
	for rowI, row := range g.g {
		for colI, state := range row {
			if state || (g.part == 2 && g.isCorner(rowI, colI)) {
				count++
			}
		}
	}
	return count
}
