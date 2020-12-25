package day25

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	n := 20201227
	pk1, pk2 := parse(input)
	// pk1 = 7 ^ s1 (mod n)
	s1 := dlog(7, pk1, n)
	// encryption key = pk2 ^ s1 (mod n)
	k := modExp(pk2, s1, n)
	return fmt.Sprint(k), nil
}

func parse(input string) (int, int) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	pk1, err := strconv.Atoi(lines[0])
	if err != nil {
		panic(err)
	}
	pk2, err := strconv.Atoi(lines[1])
	if err != nil {
		panic(err)
	}
	return pk1, pk2
}

// modExp returns b^e (mod n). It assumes that b, e, n >= 0, and that b < n.
func modExp(b, e, n int) int {
	if e == 1 {
		return b
	}
	if e%2 == 0 {
		m := modExp(b, e/2, n)
		return (m * m) % n
	}
	return (b * modExp(b, e-1, n)) % n
}

// dlog computes the discrete logarithm of b in base a, modulo n. In other
// words, it finds x such that a^x = b (mod n). It assumes that a, b < n.
func dlog(a, b, n int) int {
	// This is an implementation of the baby-step giant-step algorithm, as
	// described in
	// https://en.wikipedia.org/wiki/Baby-step_giant-step#The_algorithm.
	m := int(math.Ceil(math.Sqrt(float64(n))))
	table := make(map[int]int, m)
	f := 1
	for j := 0; j < m; j++ {
		table[f] = j
		f = (f * a) % n
	}
	inv := modExp(a, n-m-1, n)
	y := b
	for i := 0; i < m; i++ {
		if j, ok := table[y]; ok {
			return (i*m + j) % n
		}
		y = (y * inv) % n
	}
	panic("unreachable")
}
