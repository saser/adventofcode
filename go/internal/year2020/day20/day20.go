package day20

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

type direction int

const (
	top direction = iota
	right
	bottom
	left
)

type border int64

func newBorder(bits []bool) border {
	b := 0
	for _, bit := range bits {
		b <<= 1
		if bit {
			b |= 1
		}
	}
	return border(b)
}

func (b border) Reverse() border {
	r := 0
	for i := 0; i < 10; i++ {
		if b&(1<<i) != 0 {
			r |= 1 << (9 - i)
		}
	}
	return border(r)
}

func (b border) BitString() string {
	var sb strings.Builder
	for i := 9; i >= 0; i-- {
		bit := b&(1<<i) != 0
		if bit {
			sb.WriteRune('#')
		} else {
			sb.WriteRune('.')
		}
	}
	return sb.String()
}

type grid [][]bool

func parseGrid(lines []string) grid {
	rowCount := len(lines)
	colCount := len(lines[0])
	g := make([][]bool, rowCount)
	for row, line := range lines {
		g[row] = make([]bool, colCount)
		for col, r := range line {
			g[row][col] = r == '#'
		}
	}
	return g
}

func (g grid) rowCount() int {
	return len(g)
}

func (g grid) colCount() int {
	return len(g[0])
}

func (g grid) Row(i int) []bool {
	return g[i]
}

func (g grid) Col(i int) []bool {
	col := make([]bool, g.rowCount())
	for row := 0; row < g.rowCount(); row++ {
		col[row] = g[row][i]
	}
	return col
}

func (g grid) Top() border {
	return newBorder(g.Row(0))
}

func (g grid) Right() border {
	return newBorder(g.Col(g.colCount() - 1))
}

func (g grid) Bottom() border {
	return newBorder(g.Row(g.rowCount() - 1)).Reverse()
}

func (g grid) Left() border {
	return newBorder(g.Col(0)).Reverse()
}

func (g *grid) RotateRight() {
	g2 := make([][]bool, g.colCount())
	for col := 0; col < g.colCount(); col++ {
		g2[col] = g.Col(col)
	}
	*g = g2
}

func (g *grid) FlipVertically() {
	for row := 0; row < g.rowCount()/2; row++ {
		opposite := g.rowCount() - row - 1
		(*g)[row], (*g)[opposite] = (*g)[opposite], (*g)[row]
	}
}

type tile struct {
	id int64
	g  grid
}

func parseTile(paragraph string) tile {
	lines := strings.Split(paragraph, "\n")
	idLine := lines[0]
	var id int64
	if _, err := fmt.Sscanf(idLine, "Tile %d:", &id); err != nil {
		panic(err)
	}
	g := parseGrid(lines[1:])
	return tile{
		id: id,
		g:  g,
	}
}

func (t tile) top() border {
	return t.g.Top()
}

func (t tile) right() border {
	return t.g.Right()
}

func (t tile) bottom() border {
	return t.g.Bottom()
}

func (t tile) left() border {
	return t.g.Left()
}

func (t tile) contains(b border) bool {
	switch b {
	case t.top(), t.right(), t.bottom(), t.left():
		return true
	default:
		return false
	}
}

func (t tile) border(dir direction) border {
	switch dir {
	case top:
		return t.top()
	case right:
		return t.right()
	case bottom:
		return t.bottom()
	case left:
		return t.left()
	}
	panic("unreachable")
}

func (t *tile) Orient(b border, dir direction) {
	if !t.contains(b) {
		// In its current configuration, the tile does not contain the
		// border at all. This means that the border we are looking for
		// are one of the reverses. Flipping (either horizontally or
		// vertically) will result in all current borders being reversed
		// (but not necessarily in their original position).
		t.g.FlipVertically()
	}
	for t.border(dir) != b {
		t.g.RotateRight()
	}
}

type puzzle struct {
	matches map[border][]tile // border -> tile IDs
}

func newPuzzle(tiles []tile) puzzle {
	p := puzzle{
		matches: make(map[border][]tile),
	}
	for _, t := range tiles {
		for _, b := range []border{
			t.top(),
			t.right(),
			t.bottom(),
			t.left(),
			t.top().Reverse(),
			t.right().Reverse(),
			t.bottom().Reverse(),
			t.left().Reverse(),
		} {
			p.matches[b] = append(p.matches[b], t)
		}
	}
	return p
}

// UnmatchedSides returns the number of tile borders for which there is no other
// tile in the puzzle that matches.
func (p puzzle) UnmatchedSides(t tile) int {
	count := 0
	for _, b := range []border{
		t.top(),
		t.right(),
		t.bottom(),
		t.left(),
	} {
		if len(p.matches[b]) > 1 {
			count++
		}
	}
	return count
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	paragraphs := strings.Split(strings.TrimSpace(input), "\n\n")
	tiles := make([]tile, len(paragraphs))
	for i, p := range paragraphs {
		tiles[i] = parseTile(p)
	}
	p := newPuzzle(tiles)
	var prod int64 = 1
	for _, t := range tiles {
		if p.UnmatchedSides(t) == 2 {
			prod *= t.id
		}
	}
	return fmt.Sprint(prod), nil
}
