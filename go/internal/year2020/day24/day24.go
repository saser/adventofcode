package day24

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
	dirEast direction = iota
	dirSouthEast
	dirSouthWest
	dirWest
	dirNorthWest
	dirNorthEast
)

func (d direction) String() string {
	switch d {
	case dirEast:
		return "e"
	case dirSouthEast:
		return "se"
	case dirSouthWest:
		return "sw"
	case dirWest:
		return "w"
	case dirNorthWest:
		return "nw"
	case dirNorthEast:
		return "ne"
	default:
		return fmt.Sprintf("<invalid direction: %d>", d)
	}
}

func parseLine(line string) []direction {
	var dirs []direction
	for i := 0; i < len(line); i++ {
		var d direction
		switch line[i] {
		case 'e':
			d = dirEast
		case 'w':
			d = dirWest
		case 's':
			i++
			switch line[i] {
			case 'e':
				d = dirSouthEast
			case 'w':
				d = dirSouthWest
			}
		case 'n':
			i++
			switch line[i] {
			case 'e':
				d = dirNorthEast
			case 'w':
				d = dirNorthWest
			}
		}
		dirs = append(dirs, d)
	}
	return dirs
}

func parse(input string) [][]direction {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	d := make([][]direction, len(lines))
	for i, line := range lines {
		d[i] = parseLine(line)
	}
	return d
}

type point3 struct {
	X, Y, Z int
}

func (p *point3) Move(dir direction) {
	switch dir {
	case dirEast:
		p.Y--
		p.X++
	case dirSouthEast:
		p.Z--
		p.X++
	case dirSouthWest:
		p.Z--
		p.Y++
	case dirWest:
		p.X--
		p.Y++
	case dirNorthWest:
		p.X--
		p.Z++
	case dirNorthEast:
		p.Y--
		p.Z++
	}
}

func (p point3) Adjacent() [6]point3 {
	var adj [6]point3
	for i, dir := range []direction{
		dirEast,
		dirSouthEast,
		dirSouthWest,
		dirWest,
		dirNorthWest,
		dirNorthEast,
	} {
		n := p
		n.Move(dir)
		adj[i] = n
	}
	return adj
}

type tileSet map[point3]bool

func (t tileSet) Flip(p point3) {
	if t[p] {
		delete(t, p)
	} else {
		t[p] = true
	}
}

type gameOfTiles struct {
	blackTiles tileSet
}

func newGameOfTiles(blackTiles tileSet) *gameOfTiles {
	return &gameOfTiles{
		blackTiles: blackTiles,
	}
}

func (g *gameOfTiles) Step() {
	adjCount := make(map[point3]int)
	for p := range g.blackTiles {
		if _, ok := adjCount[p]; !ok {
			adjCount[p] = 0
		}
		for _, n := range p.Adjacent() {
			adjCount[n] = adjCount[n] + 1
		}
	}
	for p, count := range adjCount {
		if (g.blackTiles[p] && (count == 0 || count > 2)) || (!g.blackTiles[p] && count == 2) {
			g.blackTiles.Flip(p)
		}
	}
}

func (g *gameOfTiles) BlackTileCount() int {
	return len(g.blackTiles)
}

func solve(input string, part int) (string, error) {
	d := parse(input)
	blackTiles := make(tileSet)
	for _, dirs := range d {
		p := point3{
			X: 0,
			Y: 0,
			Z: 0,
		}
		for _, dir := range dirs {
			p.Move(dir)
		}
		blackTiles.Flip(p)
	}
	if part == 1 {
		return fmt.Sprint(len(blackTiles)), nil
	}
	g := newGameOfTiles(blackTiles)
	for i := 0; i < 100; i++ {
		g.Step()
	}
	return fmt.Sprint(g.BlackTileCount()), nil
}
