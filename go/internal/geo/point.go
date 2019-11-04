package geo

type Point struct {
	X, Y int
}

const (
	North = iota
	East
	South
	West
)

func (p *Point) Step(direction int) {
	switch direction {
	case North:
		p.Y++
	case East:
		p.X++
	case South:
		p.Y--
	case West:
		p.X--
	}
}
