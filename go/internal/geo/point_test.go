package geo

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestPoint_Step(t *testing.T) {
	for _, tt := range []struct {
		start Point
		steps []int
		end   Point
	}{
		{start: Point{X: 0, Y: 0}, steps: []int{North}, end: Point{X: 0, Y: 1}},
		{start: Point{X: 0, Y: 0}, steps: []int{East}, end: Point{X: 1, Y: 0}},
		{start: Point{X: 0, Y: 0}, steps: []int{South}, end: Point{X: 0, Y: -1}},
		{start: Point{X: 0, Y: 0}, steps: []int{West}, end: Point{X: -1, Y: 0}},
	} {
		t.Run(fmt.Sprintf("start=%v,steps=%v", tt.start, tt.steps), func(t *testing.T) {
			p := tt.start
			for _, direction := range tt.steps {
				p.Step(direction)
			}
			require.Equal(t, tt.end, p)
		})
	}
}
