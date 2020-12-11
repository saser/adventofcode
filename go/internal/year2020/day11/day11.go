package day11

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	g := newGameOfSeats(input)
	for g.Step(part) {
		// Nothing needs to be done other than stepping.
	}
	return fmt.Sprint(g.CountOccupied()), nil
}

type gameOfSeats struct {
	grid1, grid2  []byte
	current, next *[]byte
	rows, cols    int
}

func newGameOfSeats(input string) *gameOfSeats {
	grid1 := []byte(strings.ReplaceAll(input, "\n", ""))
	grid2 := make([]byte, len(grid1))
	copy(grid2, grid1)
	cols := strings.Index(input, "\n")
	rows := len(grid1) / cols
	return &gameOfSeats{
		grid1: grid1, grid2: grid2,
		current: &grid1, next: &grid2,
		cols: cols, rows: rows,
	}
}

func (g *gameOfSeats) index(row, col int) (int, bool) {
	if row < 0 || row >= g.rows || col < 0 || col >= g.cols {
		return 0, false
	}
	return row*g.cols + col, true
}

func (g *gameOfSeats) occupied(row, col int) (bool, bool) {
	i, ok := g.index(row, col)
	return ok && (*g.current)[i] == '#', ok
}

func (g *gameOfSeats) occupiedAdjacent(row, col int) int {
	d := []int{-1, 0, 1}
	n := 0
	for _, drow := range d {
		for _, dcol := range d {
			if drow == 0 && dcol == 0 {
				continue
			}
			if occ, ok := g.occupied(row+drow, col+dcol); occ && ok {
				n++
			}
		}
	}
	return n
}

func (g *gameOfSeats) occupiedVisible(row, col int) int {
	d := []int{-1, 0, 1}
	n := 0
	for _, drow := range d {
		for _, dcol := range d {
			if drow == 0 && dcol == 0 {
				continue
			}
			steps := 1
			for {
				i, ok := g.index(row+steps*drow, col+steps*dcol)
				if !ok {
					// outside grid
					break
				}
				c := (*g.current)[i]
				if c == '.' {
					// inside grid, but empty space
					steps++
					continue
				}
				// inside grid, non-empty space
				if c == '#' {
					n++
				}
				break
			}
		}
	}
	return n
}

func (g *gameOfSeats) Step(part int) bool {
	changed := false
	copy(*g.next, *g.current)
	for row := 0; row < g.rows; row++ {
		for col := 0; col < g.cols; col++ {
			i, _ := g.index(row, col)
			var adj, threshold int
			switch part {
			case 1:
				adj = g.occupiedAdjacent(row, col)
				threshold = 4
			case 2:
				adj = g.occupiedVisible(row, col)
				threshold = 5
			}
			c := (*g.current)[i]
			switch {
			case c == 'L' && adj == 0:
				(*g.next)[i] = '#'
				changed = true
			case c == '#' && adj >= threshold:
				(*g.next)[i] = 'L'
				changed = true
			}
		}
	}
	g.current, g.next = g.next, g.current
	return changed
}

func (g *gameOfSeats) CountOccupied() int {
	n := 0
	for _, c := range *g.current {
		if c == '#' {
			n++
		}
	}
	return n
}

func (g *gameOfSeats) String() string {
	var sb strings.Builder
	for row := 0; row < g.rows; row++ {
		for col := 0; col < g.cols; col++ {
			i, _ := g.index(row, col)
			sb.WriteByte((*g.current)[i])
		}
		sb.WriteByte('\n')
	}
	return sb.String()
}
