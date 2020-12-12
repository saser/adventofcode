package geo

type Point struct {
	X, Y int
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func (p Point) ManhattanDistance() int {
	return abs(p.X) + abs(p.Y)
}
