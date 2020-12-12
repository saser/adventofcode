package day12

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/Saser/adventofcode/internal/geo"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type instruction struct {
	action string
	value  int
}

func parseLine(line string) instruction {
	action := line[:1]
	value, err := strconv.Atoi(line[1:])
	if err != nil {
		panic(err)
	}
	return instruction{
		action: action,
		value:  value,
	}
}

func (i instruction) ApplyTo(t *geo.Traveller) {
	switch i.action {
	case "N", "E", "S", "W":
		i.applyMove(t)
	case "L", "R":
		i.applyTurn(t)
	case "F":
		t.StepN(i.value)
	}
}

func (i instruction) applyMove(t *geo.Traveller) {
	defer func(reset geo.Direction) {
		t.Direction = reset
	}(t.Direction)
	switch i.action {
	case "N":
		t.Direction = geo.North
	case "E":
		t.Direction = geo.East
	case "S":
		t.Direction = geo.South
	case "W":
		t.Direction = geo.West
	}
	t.StepN(i.value)
}

func (i instruction) applyTurn(t *geo.Traveller) {
	var turn geo.Turn
	switch i.action {
	case "L":
		turn = geo.Left
	case "R":
		turn = geo.Right
	}
	t.Turn(turn, i.value)
}

func parse(input string) []instruction {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	instrs := make([]instruction, len(lines))
	for i, line := range lines {
		instrs[i] = parseLine(line)
	}
	return instrs
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	t := &geo.Traveller{
		Position:  geo.Point{X: 0, Y: 0},
		Direction: geo.East,
	}
	for _, i := range parse(input) {
		i.ApplyTo(t)
	}
	return fmt.Sprint(t.Position.ManhattanDistance()), nil
}
