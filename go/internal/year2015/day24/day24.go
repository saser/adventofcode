package day24

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	items, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 24, part %d: %w", part, err)
	}
	target := uint64(0)
	for _, item := range items {
		target += item
	}
	if part == 1 {
		target /= 3
	} else {
		target /= 4
	}
	qe := search(items, target)
	return fmt.Sprint(qe), nil
}

func parse(r io.Reader) ([]uint64, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	var weights []uint64
	for sc.Scan() {
		weight, err := strconv.ParseUint(sc.Text(), 10, 64)
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		weights = append(weights, weight)
	}
	return weights, nil
}

// search implements a pseudo-polynomial time dynamic programming algorithm for
// the subset sum problem, which this is a variation of. The algorithm was taken
// from the following Wikipedia page:
// https://en.wikipedia.org/wiki/Subset_sum_problem#Pseudo-polynomial_time_dynamic_programming_solution
func search(items []uint64, target uint64) uint64 {
	type value struct {
		n  uint64
		qe uint64
	}
	min := func(v1, v2 value) value {
		var none value
		if v1 == none {
			return v2
		}
		if v2 == none {
			return v1
		}
		if v1.n < v2.n || v1.n == v2.n && v1.qe < v2.qe {
			return v1
		}
		return v2
	}
	q := make(map[uint64]map[uint64]value)
	for i := uint64(0); i < uint64(len(items)); i++ {
		q[i] = make(map[uint64]value)
		item := items[i]
		for s := uint64(0); s <= target; s++ {
			var v value
			if item == s {
				v = value{
					n:  1,
					qe: item,
				}
			}
			if i != 0 {
				if vv, ok := q[i-1][s]; ok {
					v = min(v, vv)
				}
				if s >= item {
					if vv, ok := q[i-1][s-item]; ok {
						vv.n++
						vv.qe *= item
						v = min(v, vv)
					}
				}
			}
			if v.n != 0 {
				q[i][s] = v
			}
		}
	}
	return q[uint64(len(items)-1)][target].qe
}
