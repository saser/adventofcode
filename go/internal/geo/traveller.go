package geo

import "fmt"

type Traveller struct {
	Position  Point
	Direction Direction
}

func (t *Traveller) Step() {
	switch t.Direction {
	case North:
		t.Position.Y++
	case East:
		t.Position.X++
	case South:
		t.Position.Y--
	case West:
		t.Position.X--
	}
}

func (t *Traveller) StepN(n int) {
	for i := 0; i < n; i++ {
		t.Step()
	}
}

type Turn int

const (
	Right Turn = iota
	Left
)

func (t *Traveller) Turn(turn Turn, deg int) {
	switch turn {
	case Right:
		t.turnRight(deg)
	case Left:
		t.turnLeft(deg)
	}
}

func (t *Traveller) turnRight(deg int) {
	switch deg % 360 {
	case 0:
		// do nothing
	case 90:
		switch t.Direction {
		case North:
			t.Direction = East
		case East:
			t.Direction = South
		case South:
			t.Direction = West
		case West:
			t.Direction = North
		}
	case 180:
		t.turnAround()
	case 270:
		t.turnLeft(90)
	default:
		panic(fmt.Errorf("invalid deg: %v", deg))
	}
}

func (t *Traveller) turnLeft(deg int) {
	switch deg % 360 {
	case 0:
		// do nothing
	case 90:
		switch t.Direction {
		case North:
			t.Direction = West
		case East:
			t.Direction = North
		case South:
			t.Direction = East
		case West:
			t.Direction = South
		}
	case 180:
		t.turnAround()
	case 270:
		t.turnRight(90)
	default:
		panic(fmt.Errorf("invalid deg: %v", deg))
	}
}

func (t *Traveller) turnAround() {
	switch t.Direction {
	case North:
		t.Direction = South
	case East:
		t.Direction = West
	case South:
		t.Direction = North
	case West:
		t.Direction = East
	}
}
