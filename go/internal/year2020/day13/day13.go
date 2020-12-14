package day13

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type bus struct {
	idx int
	id  int
}

func parse(input string) (int, []bus) {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	ts, err := strconv.Atoi(lines[0])
	if err != nil {
		panic(err)
	}
	parts := strings.Split(lines[1], ",")
	var buses []bus
	for i, part := range parts {
		if part == "x" {
			continue
		}
		id, err := strconv.Atoi(part)
		if err != nil {
			panic(err)
		}
		buses = append(buses, bus{idx: i, id: id})
	}
	return ts, buses
}

func findBestBus(ts int, buses []bus) (int, int) {
	minWait := 60
	var bestBus int
	for _, bus := range buses {
		next := ((ts / bus.id) + 1) * bus.id
		if wait := next - ts; wait < minWait {
			minWait = wait
			bestBus = bus.id
		}
	}
	return bestBus, minWait
}

type eq struct {
	rem, mod int
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

// solveEqs solves the given set of congruence equations, i.e., it
// finds x such that x = eq.rem (mod eq.mod) for all eq in eqs. It
// assumes that all moduli are distinct prime numbers.
func solveEqs(eqs []eq) int {
	// This solution deserves some explanation. It can most easily
	// be explained using an induction-style argument.
	//
	// Define the following, for some value of k:
	//     a_k = eq[k].rem
	//     n_k = eq[k].mod.
	//     N_k = n_0 * ... * n_k
	//     t_k = a_i (mod n_i) for all i = 0, ..., k
	// Now we want to find t_[k+1] such that t_[k+1] = a_[k+1] (mod n_[k+1]).
	//
	// We know that t_[k+1] must be of the form t_k + x * N_k,
	// where x is an integer. This is to guarantee that t_[k+1]
	// still fulfills the first k equations.
	//
	// How can we find x?
	//     t_k + x * N_k = a_[k+1] (mod n_[k+1])
	//           x * N_k = a_[k+1] - t_k (mod n_[k+1])
	//                 x = inv(N_k) * (a_[k+1] - t_k) (mod n_[k+1])
	//
	// Since we assume that all moduli are distinct prime numbers,
	// we can use Fermat's little theorem to calculate inv(N_k) (mod n_[k+1]).
	//
	// Fermat's little theorem can be stated as:
	//     a^(p-1) = 1 (mod p)
	// where p is a prime number and a is an integer not divided
	// by p.
	//
	// Specifically, this means that
	//     a * a^(p-2) = 1 (mod p)
	//     => inv(a) = a^(p-2) (mod p)
	//
	// Since N_k = n_0 * ... * n_k, where n_i are prime, then we
	// know that N_k is not divided by n_[k+1].
	// We can therefore calculate
	//     inv(N_k) = (N_k)^(n_[k+1] - 2) (mod n_[k+1]).
	// This can be efficiently calculated using modular exponentiation.
	// Now that we have calculated x = inv(N_k) * (a_[k+1] - t_k) (mod n_[k+1]),
	// we can directly calculate
	//     t_[k+1] = t_k + x * N_k (mod n_[k+1]).
	// We can continue this iterated calculation until we have
	// calculated t_M, where M is the number of equations.
	tk := eqs[0].rem // t_k
	Nk := eqs[0].mod // N_k
	for _, eq := range eqs[1:] {
		// kp1 = k plus 1
		akp1 := eq.rem                       // a_[k+1]
		nkp1 := eq.mod                       // n_[k+1]
		inv := modExp(Nk%nkp1, nkp1-2, nkp1) // = inv(N_k) (mod n_[k+1])
		diff := (akp1 - tk) % nkp1           // = a_[k+1] - t_k (mod n_[k+1])
		if diff < 0 {
			diff += eq.mod
		}
		x := (inv * diff) % eq.mod // = inv(N_k) * (a_[k+1] - t_k) (mod n_[k+1])
		tk += x * Nk
		Nk *= nkp1
	}
	return tk
}

func earliest(buses []bus) int {
	var eqs []eq
	for _, bus := range buses {
		rem := (bus.id - bus.idx) % bus.id
		if rem < 0 {
			rem += bus.id
		}
		eqs = append(eqs, eq{
			rem: rem,
			mod: bus.id,
		})
	}
	return solveEqs(eqs)
}

func solve(input string, part int) (string, error) {
	ts, buses := parse(input)
	switch part {
	case 1:
		bestBus, minWait := findBestBus(ts, buses)
		return fmt.Sprint(bestBus * minWait), nil
	case 2:
		return fmt.Sprint(earliest(buses)), nil
	}
	return "", fmt.Errorf("invalid part: %v", part)
}
