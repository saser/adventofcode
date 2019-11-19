package day18

import (
	"bufio"
	"fmt"
	"io"

	"github.com/Saser/adventofcode/internal/solution"
)

const (
	Iterations = 100
	GridSize   = 100
)

func Part1(iterations int, gridSize int) solution.Solution {
	return solve(iterations, gridSize, 1)
}

func Part2(iterations int, gridSize int) solution.Solution {
	return solve(iterations, gridSize, 2)
}

func solve(iterations int, gridSize int, part int) solution.Solution {
	return func(r io.Reader) (string, error) {
		grid, err := parse(r, gridSize)
		if err != nil {
			return "", fmt.Errorf("year 2015, day 18, part 1: %w", err)
		}
		grid.part = part
		for i := 0; i < iterations; i++ {
			grid.step()
		}
		return fmt.Sprint(grid.countOn()), nil
	}
}

func parse(r io.Reader, gridSize int) (*grid, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	g := make([][]bool, 0, gridSize)
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
				return nil, fmt.Errorf("parse: invalid rune: %s", string(r))
			}
			row = append(row, state)
		}
		g = append(g, row)
	}
	grid := &grid{
		g: g,
	}
	return grid, nil
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
