package day03

import (
	"bufio"
	"fmt"

	"github.com/Saser/adventofcode/internal/geo"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	// The part number just so happens to be the same as the numbers of travelers:
	// In part 1, there is 1 (Santa), and in part 2, there are 2 (Santa and Robo-Santa).
	travelers := make([]geo.Point, part)
	currentTraveler := 0
	visited := map[geo.Point]struct{}{
		geo.Point{X: 0, Y: 0}: struct{}{},
	}

	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanRunes)
	for sc.Scan() {
		tok := sc.Text()
		var direction int
		switch tok {
		case "^":
			direction = geo.North
		case ">":
			direction = geo.East
		case "v":
			direction = geo.South
		case "<":
			direction = geo.West
		default:
			return "", fmt.Errorf("invalid direction: %s", tok)
		}
		travelers[currentTraveler].Step(direction)
		visited[travelers[currentTraveler]] = struct{}{}
		currentTraveler = (currentTraveler + 1) % len(travelers)
	}
	return fmt.Sprint(len(visited)), nil
}
