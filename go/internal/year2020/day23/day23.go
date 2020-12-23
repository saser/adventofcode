package day23

import (
	"container/ring"
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
	r *ring.Ring
	m map[rune]*ring.Ring // number -> node in ring
}

func parse(input string) *cups {
	r := ring.New(9)
	m := make(map[rune]*ring.Ring)
	for _, c := range strings.TrimSpace(input) {
		v := c - '0'
		r.Value = v
		m[v] = r
		r = r.Next()
	}
	return &cups{
		r: r,
		m: m,
	}
}

func (c *cups) String() string {
	var sb strings.Builder
	c.m[1].Do(func(v interface{}) {
		n := v.(rune)
		if n == 1 {
			return
		}
		sb.WriteRune(n + '0')
	})
	return sb.String()
}

func (c *cups) DebugString() string {
	var ss []string
	c.r.Do(func(v interface{}) {
		n := v.(rune)
		if len(ss) == 0 {
			ss = append(ss, fmt.Sprintf("(%s)", fmt.Sprint(n)))
			return
		}
		ss = append(ss, fmt.Sprint(n))
	})
	return strings.Join(ss, " ")
}

func (c *cups) Move() {
	curr := c.r.Value.(rune)
	held := c.r.Unlink(3)
	forbidden := make(map[rune]bool, 3)
	held.Do(func(v interface{}) {
		forbidden[v.(rune)] = true
	})
	dest := curr - 1
	for {
		if dest < 1 {
			dest = 9
		}
		if !forbidden[dest] {
			break
		}
		dest--
	}
	c.m[dest].Link(held)
	c.r = c.r.Next()
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	c := parse(input)
	for i := 0; i < 100; i++ {
		c.Move()
	}
	return c.String(), nil
}
