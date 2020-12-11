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
	var (
		c         counter
		threshold int
	)
	switch part {
	case 1:
		c = adjacentCounter{}
		threshold = 4
	case 2:
		c = visibleCounter{}
		threshold = 5
	}
	g := newGameOfSeats(input, c, threshold)
	for g.Step(part) {
		// Nothing needs to be done other than stepping.
	}
	return fmt.Sprint(g.CountOccupied()), nil
}

type tile byte

const (
	floor    tile = '.'
	empty         = 'L'
	occupied      = '#'
)

type grid struct {
	tiles      []tile
	rows, cols int
}

func newGrid(input string) grid {
	b := []byte(strings.ReplaceAll(input, "\n", ""))
	tiles := make([]tile, len(b))
	for i, c := range b {
		tiles[i] = tile(c)
	}
	cols := strings.Index(input, "\n")
	rows := len(b) / cols
	return grid{
		tiles: tiles,
		rows:  rows,
		cols:  cols,
	}
}

func (g grid) Copy() grid {
	tiles := make([]tile, len(g.tiles))
	copy(tiles, g.tiles)
	return grid{
		tiles: tiles,
		rows:  g.rows,
		cols:  g.cols,
	}
}

func (g grid) Index(row, col int) (int, bool) {
	if row < 0 || row >= g.rows || col < 0 || col >= g.cols {
		return 0, false
	}
	return row*g.cols + col, true
}

func (g grid) Tile(idx int) tile {
	return g.tiles[idx]
}

func (g grid) Flip(idx int) {
	var next tile
	switch g.tiles[idx] {
	case floor:
		next = floor
	case empty:
		next = occupied
	case occupied:
		next = empty
	}
	g.tiles[idx] = next
}

type counter interface {
	count(g grid, row, col int) int
}

type adjacentCounter struct{}

func (adjacentCounter) count(g grid, row, col int) int {
	delta := []int{-1, 0, 1}
	count := 0
	for _, deltaRow := range delta {
		for _, deltaCol := range delta {
			if deltaRow == 0 && deltaCol == 0 {
				continue
			}
			idx, ok := g.Index(row+deltaRow, col+deltaCol)
			if !ok {
				continue
			}
			if g.Tile(idx) == occupied {
				count++
			}
		}
	}
	return count
}

type visibleCounter struct{}

func (c visibleCounter) count(g grid, row, col int) int {
	count := 0
	var visible []int
	delta := []int{-1, 0, 1}
	for _, deltaRow := range delta {
		for _, deltaCol := range delta {
			if deltaRow == 0 && deltaCol == 0 {
				continue
			}
			r := row + deltaRow
			c := col + deltaCol
			var (
				idx int
				ok  bool
			)
			for idx, ok = g.Index(r, c); ok && g.Tile(idx) == floor; idx, ok = g.Index(r, c) {
				r += deltaRow
				c += deltaCol
			}
			if !ok {
				continue // ended up outside grid
			}
			visible = append(visible, idx) // ended up inside grid, cache the index
			if g.Tile(idx) == occupied {
				count++
			}
		}
	}
	return count
}

type gameOfSeats struct {
	g1, g2     grid
	curr, next *grid
	c          counter
	threshold  int
}

func newGameOfSeats(input string, c counter, threshold int) *gameOfSeats {
	g1 := newGrid(input)
	g2 := g1.Copy()
	return &gameOfSeats{
		g1:        g1,
		g2:        g2,
		curr:      &g1,
		next:      &g2,
		c:         c,
		threshold: threshold,
	}
}

func (g *gameOfSeats) Step(part int) bool {
	changed := false
	*g.next = g.curr.Copy()
	for row := 0; row < g.curr.rows; row++ {
		for col := 0; col < g.curr.cols; col++ {
			idx, _ := g.curr.Index(row, col)
			adj := g.c.count(*g.curr, row, col)
			t := g.curr.Tile(idx)
			if (t == empty && adj == 0) || (t == occupied && adj >= g.threshold) {
				g.next.Flip(idx)
				changed = true
			}
		}
	}
	g.curr, g.next = g.next, g.curr
	return changed
}

func (g *gameOfSeats) CountOccupied() int {
	count := 0
	for row := 0; row < g.curr.rows; row++ {
		for col := 0; col < g.curr.cols; col++ {
			idx, _ := g.curr.Index(row, col)
			if g.curr.Tile(idx) == occupied {
				count++
			}
		}
	}
	return count
}
