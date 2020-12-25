package day23

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type cups struct {
	curr int
	next []int // number -> next number
}

func newCups(numbers []int) *cups {
	n := len(numbers)
	c := &cups{
		curr: numbers[0],
		next: make([]int, n),
	}
	for i := 0; i < n-1; i++ {
		c.setNext(numbers[i], numbers[i+1])
	}
	c.setNext(numbers[n-1], numbers[0])
	return c
}

func (c *cups) Next(n int) int {
	return c.next[n-1]
}

func (c *cups) setNext(n, next int) {
	c.next[n-1] = next
}

func (c *cups) Move() {
	// curr -> n1 -> n2 -> n3 -> next
	n1 := c.Next(c.curr)
	n2 := c.Next(n1)
	n3 := c.Next(n2)
	next := c.Next(n3)

	// Find the destination cup.
	dest := c.curr - 1
	for {
		if dest < 1 {
			dest = len(c.next)
		}
		if !(dest == n1 || dest == n2 || dest == n3) {
			break
		}
		dest--
	}
	dest2 := c.Next(dest)
	// curr -> n1 -> n2 -> n3 -> next -> ... -> dest -> dest2 -> ...

	// "Unlink" by pointing curr directly to next.
	//
	//     curr -> next -> ... -> dest -> dest2 -> ...
	//              ^
	//              |
	// n1 -> n2 -> n3
	c.setNext(c.curr, next)

	// "Link" by pointing dest to n1 ...
	//
	//                              curr -> next
	//                                       ^
	//                                       |
	// (next -> ... ->) dest -> n1 -> n2 -> n3
	c.setNext(dest, n1)
	// ... and then pointing n3 to dest2.
	//
	// curr -> next -> ... -> dest -> n1 -> n2 -> n3 -> dest2 -> ...
	c.setNext(n3, dest2)

	// Finally, update c.curr.
	c.curr = next
}

func parse(input string, numbers []int) {
	for i, r := range input {
		numbers[i] = int(r - '0')
	}
}

func solve(input string, part int) (string, error) {
	trimmed := strings.TrimSpace(input)
	var (
		numbers   []int
		moveCount int
	)
	switch part {
	case 1:
		numbers = make([]int, len(trimmed))
		moveCount = 100
	case 2:
		numbers = make([]int, 1000000)
		for i := 1; i <= 1000000; i++ {
			numbers[i-1] = i
		}
		moveCount = 10000000
	}
	parse(trimmed, numbers)
	c := newCups(numbers)
	for i := 0; i < moveCount; i++ {
		c.Move()
	}
	if part == 1 {
		var sb strings.Builder
		for n := c.Next(1); n != 1; n = c.Next(n) {
			sb.WriteRune(rune(n) + '0')
		}
		return sb.String(), nil
	}
	p1 := c.Next(1)
	p2 := c.Next(p1)
	return fmt.Sprint(p1 * p2), nil
}
