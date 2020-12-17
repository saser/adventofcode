package day17

import (
	"fmt"
	"log"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type point struct {
	x, y, z int
}

type grid map[point]bool

type conwayCubes struct {
	g1, g2     grid
	curr, next *grid
}

func parse(input string) *conwayCubes {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	g1 := make(grid)
	for y := 0; y < len(lines); y++ {
		for x := 0; x < len(lines[0]); x++ {
			if lines[y][x] == '#' {
				g1[point{x, y, 0}] = true
			}
		}
	}
	g2 := make(grid, len(g1))
	return &conwayCubes{
		g1:   g1,
		g2:   g2,
		curr: &g1,
		next: &g2,
	}
}

func (cc conwayCubes) countNeighbors(p point) int {
	deltas := []int{-1, 0, 1}
	n := 0
	for _, dx := range deltas {
		for _, dy := range deltas {
			for _, dz := range deltas {
				if dx == 0 && dy == 0 && dz == 0 {
					continue
				}
				dp := point{
					x: p.x + dx,
					y: p.y + dy,
					z: p.z + dz,
				}
				if (*cc.curr)[dp] {
					n++
				}
			}
		}
	}
	return n
}

func (cc conwayCubes) bounds() (int, int, int, int, int, int) {
	var (
		xMin, xMax,
		yMin, yMax,
		zMin, zMax *int
	)
	for p := range *cc.curr {
		p := p
		if xMin == nil || p.x < *xMin {
			xMin = &p.x
		}
		if xMax == nil || p.x > *xMax {
			xMax = &p.x
		}

		if yMin == nil || p.y < *yMin {
			yMin = &p.y
		}
		if yMax == nil || p.y > *yMax {
			yMax = &p.y
		}

		if zMin == nil || p.z < *zMin {
			zMin = &p.z
		}
		if zMax == nil || p.z > *zMax {
			zMax = &p.z
		}
	}
	return *xMin, *xMax, *yMin, *yMax, *zMin, *zMax
}

func (cc *conwayCubes) Cycle() {
	// The below loop might seem slow, but it is of a special form
	// that is optimized by the compiler.
	for k := range *cc.next {
		delete(*cc.next, k)
	}
	xMin, xMax, yMin, yMax, zMin, zMax := cc.bounds()
	for x := xMin - 1; x <= xMax+1; x++ {
		for y := yMin - 1; y <= yMax+1; y++ {
			for z := zMin - 1; z <= zMax+1; z++ {
				p := point{x, y, z}
				neighbors := cc.countNeighbors(p)
				var next bool
				switch (*cc.curr)[p] {
				case true:
					next = neighbors == 2 || neighbors == 3
				case false:
					next = neighbors == 3
				}
				if next {
					(*cc.next)[p] = true
				}
			}
		}
	}
	cc.curr, cc.next = cc.next, cc.curr
}

func (cc conwayCubes) Log() {
	xMin, xMax, yMin, yMax, zMin, zMax := cc.bounds()
	for z := zMin; z <= zMax; z++ {
		log.Printf("z=%v", z)
		for y := yMin; y <= yMax; y++ {
			var sb strings.Builder
			for x := xMin; x <= xMax; x++ {
				p := point{x, y, z}
				b := '.'
				if (*cc.curr)[p] {
					b = '#'
				}
				sb.WriteRune(b)
			}
			log.Println(sb.String())
		}
		log.Println()
	}
}

func (cc conwayCubes) CountActive() int {
	return len(*cc.curr)
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	cc := parse(input)
	for i := 0; i < 6; i++ {
		cc.Cycle()
	}
	return fmt.Sprint(cc.CountActive()), nil
}
