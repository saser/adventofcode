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
	x, y, z, w int
}

func (p point) Neighbors(part int) []point {
	deltas := []int{-1, 0, 1}
	var wDeltas []int
	switch part {
	case 1:
		wDeltas = []int{0}
	case 2:
		wDeltas = []int{-1, 0, 1}
	}
	var points []point
	for _, dx := range deltas {
		for _, dy := range deltas {
			for _, dz := range deltas {
				for _, dw := range wDeltas {
					if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
						continue
					}
					points = append(points, point{
						x: p.x + dx,
						y: p.y + dy,
						z: p.z + dz,
						w: p.w + dw,
					})
				}
			}
		}
	}
	return points
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
				g1[point{x, y, 0, 0}] = true
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

func (cc conwayCubes) bounds() (int, int, int, int, int, int, int, int) {
	var (
		xMin, xMax,
		yMin, yMax,
		zMin, zMax,
		wMin, wMax *int
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

		if wMin == nil || p.w < *wMin {
			wMin = &p.w
		}
		if wMax == nil || p.w > *wMax {
			wMax = &p.w
		}
	}
	return *xMin, *xMax, *yMin, *yMax, *zMin, *zMax, *wMin, *wMax
}

func (cc *conwayCubes) Cycle(part int) {
	// The below loop might seem slow, but it is of a special form
	// that is optimized by the compiler.
	for k := range *cc.next {
		delete(*cc.next, k)
	}
	neighborCounts := make(map[point]int)
	for p := range *cc.curr {
		for _, neighbor := range p.Neighbors(part) {
			neighborCounts[neighbor]++
		}
	}
	for p, count := range neighborCounts {
		switch active := (*cc.curr)[p]; {
		case active && (count == 2 || count == 3):
			(*cc.next)[p] = true
		case !active && count == 3:
			(*cc.next)[p] = true
		}
	}
	cc.curr, cc.next = cc.next, cc.curr
}

func (cc conwayCubes) Log() {
	xMin, xMax, yMin, yMax, zMin, zMax, wMin, wMax := cc.bounds()
	for w := wMin; w <= wMax; w++ {
		for z := zMin; z <= zMax; z++ {
			log.Printf("z=%v, w=%v", z, w)
			for y := yMin; y <= yMax; y++ {
				var sb strings.Builder
				for x := xMin; x <= xMax; x++ {
					p := point{x, y, z, w}
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
}

func (cc conwayCubes) CountActive() int {
	return len(*cc.curr)
}

func solve(input string, part int) (string, error) {
	cc := parse(input)
	for i := 0; i < 6; i++ {
		cc.Cycle(part)
	}
	return fmt.Sprint(cc.CountActive()), nil
}
