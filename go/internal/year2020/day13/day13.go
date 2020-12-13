package day13

import (
	"fmt"
	"math/big"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func parse(input string) (int, []int) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	ts, err := strconv.Atoi(lines[0])
	if err != nil {
		panic(err)
	}
	parts := strings.Split(lines[1], ",")
	buses := make([]int, len(parts))
	for i, part := range parts {
		var bus int
		if part == "x" {
			bus = -1
		} else {
			bus, err = strconv.Atoi(part)
			if err != nil {
				panic(err)
			}
		}
		buses[i] = bus
	}
	return ts, buses
}

func findBestBus(ts int, buses []int) (int, int) {
	minWait := 60
	var bestBus int
	for _, bus := range buses {
		if bus == -1 {
			continue
		}
		next := ((ts / bus) + 1) * bus
		if wait := next - ts; wait < minWait {
			minWait = wait
			bestBus = bus
		}
	}
	return bestBus, minWait
}

type eq struct {
	rem, mod *big.Int
}

// crt uses the Chinese remainder theorem to find a solution to the
// set of equations given b eqs.
func crt(eqs []eq) *big.Int {
	var (
		gcd            big.Int
		m1, m2         big.Int
		a2m1n1, a1m2n2 big.Int
		n1n2           big.Int
		x              big.Int
	)
	acc := eqs[0]
	for _, eq := range eqs[1:] {
		a1, n1 := acc.rem, acc.mod
		a2, n2 := eq.rem, eq.mod
		// GCD runs the extended Euclidean algorithm. We know
		// the result is 1, and we are only interested in
		// obtaining the values of m1 and m2.
		gcd.GCD(&m1, &m2, n1, n2)
		a2m1n1.Mul(a2, &m1).Mul(&a2m1n1, n1)
		a1m2n2.Mul(a1, &m2).Mul(&a1m2n2, n2)
		x.Add(&a2m1n1, &a1m2n2)
		n1n2.Mul(n1, n2)
		acc.rem.Mod(&x, &n1n2)
		acc.mod = &n1n2
	}
	return acc.rem
}

func earliestTimestamp(buses []int) *big.Int {
	var eqs []eq
	for i, bus := range buses {
		if bus == -1 {
			continue
		}
		rem := (bus - i) % bus
		if rem < 0 {
			rem += bus
		}
		eqs = append(eqs, eq{
			rem: big.NewInt(int64(rem)),
			mod: big.NewInt(int64(bus)),
		})
	}
	return crt(eqs)
}

func solve(input string, part int) (string, error) {
	ts, buses := parse(input)
	switch part {
	case 1:
		bestBus, minWait := findBestBus(ts, buses)
		return fmt.Sprint(bestBus * minWait), nil
	case 2:
		ts := earliestTimestamp(buses)
		return ts.String(), nil
	}
	return "", fmt.Errorf("invalid part: %v", part)
}
