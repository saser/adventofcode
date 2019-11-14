package day15

import (
	"errors"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return "", errors.New("not yet implemented")
}

func distributions(sum int, parts int) [][]int {
	if parts == 1 {
		return [][]int{{sum}}
	}
	ds := make([][]int, 0)
	for i := 0; i <= sum; i++ {
		for _, sub := range distributions(sum-i, parts-1) {
			ds = append(ds, append([]int{i}, sub...))
		}
	}
	return ds
}
