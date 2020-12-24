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

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	d := parse(input)
	flipped := make(map[point3]bool)
	for _, dirs := range d {
		p := point3{
			X: 0,
			Y: 0,
			Z: 0,
		}
		for _, dir := range dirs {
			p.Move(dir)
		}
		if !flipped[p] {
			flipped[p] = true
		} else {
			delete(flipped, p)
		}
	}
	return fmt.Sprint(len(flipped)), nil
}
