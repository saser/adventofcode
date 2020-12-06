package geo

import (
	"fmt"
	"testing"
)

func TestPoint_Step(t *testing.T) {
	for _, tt := range []struct {
		start Point
		steps []int
		want  Point
	}{
		{start: Point{X: 0, Y: 0}, steps: []int{North}, want: Point{X: 0, Y: 1}},
		{start: Point{X: 0, Y: 0}, steps: []int{East}, want: Point{X: 1, Y: 0}},
		{start: Point{X: 0, Y: 0}, steps: []int{South}, want: Point{X: 0, Y: -1}},
		{start: Point{X: 0, Y: 0}, steps: []int{West}, want: Point{X: -1, Y: 0}},
	} {
		t.Run(fmt.Sprintf("start=%v,steps=%v", tt.start, tt.steps), func(t *testing.T) {
			p := tt.start
			for _, direction := range tt.steps {
				p.Step(direction)
			}
			if p != tt.want {
				t.Errorf("p = %v; want %v", p, tt.want)
			}
		})
	}
}
