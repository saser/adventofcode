package day12

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/Saser/adventofcode/internal/geo"
	"github.com/Saser/adventofcode/internal/linalg"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type guidedShip struct {
	waypoint *linalg.Vec2 // always relative to the ship
	ship     *linalg.Vec2
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

func (i instruction) ApplyToShip(t *geo.Traveller) {
	switch i.action {
	case "N", "E", "S", "W":
		i.applyMoveShip(t)
	case "L", "R":
		i.applyTurnShip(t)
	case "F":
		t.StepN(i.value)
	}
}

func (i instruction) applyMoveShip(t *geo.Traveller) {
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

func (i instruction) applyTurnShip(t *geo.Traveller) {
	var turn geo.Turn
	switch i.action {
	case "L":
		turn = geo.Left
	case "R":
		turn = geo.Right
	}
	t.Turn(turn, i.value)
}

func (i instruction) ApplyToGuidedShip(s *guidedShip) {
	switch i.action {
	case "N", "E", "S", "W":
		i.applyMoveGuidedShip(s)
	case "L", "R":
		i.applyTurnGuidedShip(s)
	case "F":
		for n := 0; n < i.value; n++ {
			s.ship.Add(s.waypoint)
		}
	}
}

func (i instruction) applyMoveGuidedShip(s *guidedShip) {
	dv := new(linalg.Vec2)
	switch i.action {
	case "N":
		dv.Y = 1
	case "E":
		dv.X = 1
	case "S":
		dv.Y = -1
	case "W":
		dv.X = -1
	}
	s.waypoint.Add(dv.Mul(i.value))
}

var rotLeft = linalg.Mat2{
	X1: 0, Y1: -1,
	X2: 1, Y2: 0,
}

func (i instruction) applyTurnGuidedShip(s *guidedShip) {
	var f int
	// Assume left turn.
	switch i.value % 360 {
	case 0:
		// do nothing
		return
	case 90:
		f = 1 // turn 90 degrees left (ccw)
	case 180:
		s.waypoint.Mul(-1) // invert
		return
	case 270:
		f = -1 // turn 270 degrees left = invert + turn 90 degrees left
	}
	if i.action == "R" {
		f *= -1 // do the inverse of a left turn
	}
	s.waypoint.MatMul(rotLeft).Mul(f)
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
	instrs := parse(input)
	switch part {
	case 1:
		t := &geo.Traveller{
			Position:  geo.Point{X: 0, Y: 0},
			Direction: geo.East,
		}
		for _, i := range instrs {
			i.ApplyToShip(t)
		}
		return fmt.Sprint(t.Position.ManhattanDistance()), nil
	case 2:
		s := &guidedShip{
			ship:     &linalg.Vec2{X: 0, Y: 0},
			waypoint: &linalg.Vec2{X: 10, Y: 1},
		}
		for _, i := range instrs {
			i.ApplyToGuidedShip(s)
		}
		return fmt.Sprint(geo.Point{X: s.ship.X, Y: s.ship.Y}.ManhattanDistance()), nil
	}
	return "", fmt.Errorf("invalid part: %v", part)
}
