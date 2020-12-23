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
	start *ring.Ring
	nodes []*ring.Ring // number -> node in ring
	max   int
}

func parse(input string) *ring.Ring {
	trimmed := strings.TrimSpace(input)
	r := ring.New(len(trimmed))
	for _, c := range trimmed {
		v := int(c - '0')
		r.Value = v
		r = r.Next()
	}
	return r
}

func newCups(r *ring.Ring) *cups {
	n := r.Len()
	m2 := make([]*ring.Ring, n)
	node := r
	for i := 0; i < n; i++ {
		v := node.Value.(int)
		m2[v-1] = node
		node = node.Next()
	}
	return &cups{
		start: r,
		nodes: m2,
		max:   n,
	}
}

func (c *cups) String() string {
	var sb strings.Builder
	c.nodes[0].Do(func(v interface{}) {
		n := v.(int)
		if n == 1 {
			return
		}
		sb.WriteString(fmt.Sprint(n))
	})
	return sb.String()
}

func (c *cups) DebugString() string {
	var ss []string
	c.start.Do(func(v interface{}) {
		n := v.(int)
		if len(ss) == 0 {
			ss = append(ss, fmt.Sprintf("(%s)", fmt.Sprint(n)))
			return
		}
		ss = append(ss, fmt.Sprint(n))
	})
	return strings.Join(ss, " ")
}

func (c *cups) Part2Product() int64 {
	var prod int64 = 1
	node := c.nodes[0].Next()
	prod *= int64(node.Value.(int))
	prod *= int64(node.Next().Value.(int))
	return prod
}

func (c *cups) Move() {
	curr := c.start.Value.(int)
	held := c.start.Unlink(3)
	h1 := held
	h2 := h1.Next()
	h3 := h2.Next()
	forbidden := [...]int{
		h1.Value.(int),
		h2.Value.(int),
		h3.Value.(int),
	}
	dest := curr - 1
	for {
		if dest < 1 {
			dest = c.max
		}
		if !(dest == forbidden[0] || dest == forbidden[1] || dest == forbidden[2]) {
			break
		}
		dest--
	}
	c.nodes[dest-1].Link(held)
	c.start = c.start.Next()
}

func solve(input string, part int) (string, error) {
	r := parse(input)
	moveCount := 100
	if part == 2 {
		n := r.Len()
		rest := ring.New(1000000 - n)
		for i := n + 1; i <= 1000000; i++ {
			rest.Value = i
			rest = rest.Next()
		}
		r.Prev().Link(rest)
		moveCount = 10000000
	}
	c := newCups(r)
	for i := 0; i < moveCount; i++ {
		c.Move()
	}
	if part == 1 {
		return c.String(), nil
	}
	return fmt.Sprint(c.Part2Product()), nil
}
