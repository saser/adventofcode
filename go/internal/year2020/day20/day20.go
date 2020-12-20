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

func newBorder(bits [10]bool) border {
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

type grid [10][10]bool

func parseGrid(lines []string) grid {
	var g grid
	for row, line := range lines {
		for col, r := range line {
			g[row][col] = r == '#'
		}
	}
	return g
}

func (g grid) Row(i int) [10]bool {
	return g[i]
}

func (g grid) Col(i int) [10]bool {
	var col [10]bool
	for row := 0; row < 10; row++ {
		col[row] = g[row][i]
	}
	return col
}

func (g grid) Top() border {
	return newBorder(g.Row(0))
}

func (g grid) Right() border {
	return newBorder(g.Col(9))
}

func (g grid) Bottom() border {
	return newBorder(g.Row(9)).Reverse()
}

func (g grid) Left() border {
	return newBorder(g.Col(0)).Reverse()
}

type tile struct {
	id                       int64
	top, right, bottom, left border
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
		id:     id,
		top:    g.Top(),
		right:  g.Right(),
		bottom: g.Bottom(),
		left:   g.Left(),
	}
}

func (t tile) contains(b border) bool {
	switch b {
	case t.top, t.right, t.bottom, t.right:
		return true
	default:
		return false
	}
}

func (t tile) border(dir direction) border {
	switch dir {
	case top:
		return t.top
	case right:
		return t.right
	case bottom:
		return t.bottom
	case left:
		return t.left
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
		t.flipHorizontally()
	}
	for t.border(dir) != b {
		t.rotateRight()
	}
}

func (t *tile) rotateRight() {
	//       t                     l
	//      -->                   -->
	//     +---+                 +---+
	//   ^ |   | |             ^ |   | |
	// l | |   | | r   -->   b | |   | | t
	//   | |   | v             | |   | v
	//     +---+                 +---+
	//      <--                   <--
	//       b                     r
	t.top, t.right, t.bottom, t.left = t.left, t.top, t.right, t.bottom
}

func (t *tile) rotateLeft() {
	//       t                     r
	//      -->                   -->
	//     +---+                 +---+
	//   ^ |   | |             ^ |   | |
	// l | |   | | r   -->   t | |   | | b
	//   | |   | v             | |   | v
	//     +---+                 +---+
	//      <--                   <--
	//       b                     l
	t.top, t.right, t.bottom, t.left = t.right, t.bottom, t.left, t.top
}

func (t *tile) flipHorizontally() {
	//       t                     t                      t'
	//      -->                   <--                    -->
	//     +---+                 +---+                  +---+
	//   ^ |   | |             | |   | ^              ^ |   | |
	// l | |   | | r   -->   r | |   | | l   ===   r' | |   | | l'
	//   | |   | v             v |   | |              | |   | v
	//     +---+                 +---+                  +---+
	//      <--                   -->                    <--
	//       b                     b                      b'
	t.top, t.right, t.bottom, t.left = t.top.Reverse(), t.left.Reverse(), t.bottom.Reverse(), t.right.Reverse()
}

func (t *tile) flipVertically() {
	//       t                     b                      b'
	//      -->                   <--                    -->
	//     +---+                 +---+                  +---+
	//   ^ |   | |             | |   | ^              ^ |   | |
	// l | |   | | r   -->   l | |   | | r   ===   l' | |   | | r'
	//   | |   | v             v |   | |              | |   | v
	//     +---+                 +---+                  +---+
	//      <--                   -->                    <--
	//       b                     t                      t'
	t.top, t.right, t.bottom, t.left = t.bottom.Reverse(), t.right.Reverse(), t.top.Reverse(), t.left.Reverse()
}

func solve(input string, part int) (string, error) {
	return "", fmt.Errorf("solution not implemented for part %v", part)
}
