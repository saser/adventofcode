package day24

import (
	"bufio"
	"container/heap"
	"errors"
	"fmt"
	"io"
	"math/bits"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	items, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 24, part %d: %w", part, err)
	}
	target := uint(0)
	for _, item := range items {
		target += item
	}
	if part == 1 {
		target /= 3
	} else {
		target /= 4
	}
	qe, err := search(items, target)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 24, part %d: %w", part, err)
	}
	return fmt.Sprint(qe), nil
}

func parse(r io.Reader) ([]uint, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	var weights []uint
	for sc.Scan() {
		weight, err := strconv.Atoi(sc.Text())
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		weights = append(weights, uint(weight))
	}
	return weights, nil
}

func set(mask uint, i int) uint {
	return mask | (1 << i)
}

func isSet(mask uint, i int) bool {
	return mask&(1<<i) != 0
}

type inventory []uint

func (i inventory) eval(mask uint) (int, uint, uint) {
	n := bits.OnesCount(mask)
	sum := uint(0)
	qe := uint(1)
	for idx := 0; idx < bits.Len(mask); idx++ {
		if isSet(mask, idx) {
			item := i[idx]
			sum += item
			qe *= item
		}
	}
	return n, sum, qe
}

type priorityQueue struct {
	inventory inventory
	masks     []uint
}

func (pq *priorityQueue) Len() int {
	return len(pq.masks)
}

func (pq *priorityQueue) Less(i, j int) bool {
	iN, _, iQE := pq.inventory.eval(pq.masks[i])
	jN, _, jQE := pq.inventory.eval(pq.masks[j])
	if iN != jN {
		return iN < jN
	}
	return iQE < jQE
}

func (pq *priorityQueue) Swap(i, j int) {
	pq.masks[i], pq.masks[j] = pq.masks[j], pq.masks[i]
}

func (pq *priorityQueue) Push(x interface{}) {
	pq.masks = append(pq.masks, x.(uint))
}

func (pq *priorityQueue) Pop() interface{} {
	n := len(pq.masks)
	mask := pq.masks[n-1]
	pq.masks = pq.masks[0 : n-1]
	return mask
}

func search(inventory inventory, target uint) (uint, error) {
	pq := priorityQueue{
		inventory: inventory,
		masks:     make([]uint, 1),
	}
	pq.masks[0] = 0
	heap.Init(&pq)
	visited := make(map[uint]struct{})
	for pq.Len() > 0 {
		mask := heap.Pop(&pq).(uint)
		_, sum, qe := inventory.eval(mask)
		if sum == target {
			return qe, nil
		}
		if sum > target {
			continue
		}
		if _, ok := visited[mask]; ok {
			continue
		}
		visited[mask] = struct{}{}
		for i, _ := range inventory {
			if isSet(mask, i) {
				continue
			}
			heap.Push(&pq, set(mask, i))
		}
	}
	return 0, errors.New("no way to organize packages")
}
