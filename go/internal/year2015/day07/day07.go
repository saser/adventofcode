package day07

import (
	"bufio"
	"fmt"
	"io"
	"regexp"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	wires, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 07, part 1: %w", err)
	}
	c := circuit{
		wires:  wires,
		values: make(map[string]uint16),
	}
	v, err := c.Eval("a")
	if err != nil {
		return "", fmt.Errorf("year 2015, day 07, part 1: %w", err)
	}
	s := fmt.Sprint(v)
	if part == 1 {
		return s, nil
	}
	c.wires["b"] = &nonary{r: s}
	c.values = make(map[string]uint16)
	v, err = c.Eval("a")
	if err != nil {
		return "", fmt.Errorf("year 2015, day 07, part 1: %w", err)
	}
	return fmt.Sprint(v), nil
}

func parse(r io.Reader) (_ map[string]wire, rErr error) {
	defer func() {
		if rErr != nil {
			rErr = fmt.Errorf("parse: %w", rErr)
		}
	}()
	nonaryRE, err := regexp.Compile(`^(\w+)$`)
	if err != nil {
		return nil, err
	}
	shiftRE, err := regexp.Compile(`^(\w+) (L|R)SHIFT (\d+)$`)
	if err != nil {
		return nil, err
	}
	notRE, err := regexp.Compile(`^NOT (\w+)$`)
	if err != nil {
		return nil, err
	}
	binaryRE, err := regexp.Compile(`^(\w+) (AND|OR) (\w+)$`)
	if err != nil {
		return nil, err
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	wires := make(map[string]wire)
	for sc.Scan() {
		line := sc.Text()
		parts := strings.Split(line, " -> ")
		spec := parts[0]
		name := parts[1]
		var w wire
		if matches := nonaryRE.FindStringSubmatch(spec); matches != nil {
			w = &nonary{
				r: matches[1],
			}
		} else if matches := shiftRE.FindStringSubmatch(spec); matches != nil {
			r := matches[1]
			d, err := strconv.Atoi(matches[3])
			if err != nil {
				return nil, err
			}
			var op unaryOp
			switch matches[2] {
			case "L":
				op = leftShiftBy(d)
			case "R":
				op = rightShiftBy(d)
			}
			w = &unary{
				r:  r,
				op: op,
			}
		} else if matches := notRE.FindStringSubmatch(spec); matches != nil {
			w = &unary{
				r:  matches[1],
				op: not,
			}
		} else if matches := binaryRE.FindStringSubmatch(spec); matches != nil {
			r1 := matches[1]
			r2 := matches[3]
			var op binaryOp
			switch matches[2] {
			case "AND":
				op = and
			case "OR":
				op = or
			}
			w = &binary{
				r1: r1,
				r2: r2,
				op: op,
			}
		} else {
			return nil, fmt.Errorf("could not parse spec: %s", spec)
		}
		wires[name] = w
	}
	return wires, nil
}

type wire interface {
	wire()
}

type nonary struct {
	r string
}

func (n *nonary) wire() {}

type unaryOp func(uint16) uint16

type unary struct {
	r  string
	op unaryOp
}

func (u *unary) wire() {}

type binaryOp func(uint16, uint16) uint16

type binary struct {
	r1 string
	r2 string
	op binaryOp
}

func (b *binary) wire() {}

func leftShiftBy(d int) unaryOp {
	return func(v uint16) uint16 {
		return v << d
	}
}

func rightShiftBy(d int) unaryOp {
	return func(v uint16) uint16 {
		return v >> d
	}
}

func not(v uint16) uint16 {
	return ^v
}

func and(v1, v2 uint16) uint16 {
	return v1 & v2
}

func or(v1, v2 uint16) uint16 {
	return v1 | v2
}

type circuit struct {
	wires  map[string]wire
	values map[string]uint16
}

func (c *circuit) Eval(e string) (v uint16, err error) {
	// Check if there already exists a memoized value for `s`.
	if v, ok := c.values[e]; ok {
		return v, nil
	}
	// Memoize values that were calculated correctly.
	defer func() {
		if err == nil {
			c.values[e] = v
		}
	}()
	// If we are evaluating a constant expression, simply return it.
	if v, err := strconv.ParseUint(e, 10, 16); err == nil {
		return uint16(v), nil
	}
	// We are not evaluating a constant expression, which means that we are evaluating a reference to another
	// wire. Therefore, look up the corresponding wire.
	wire, ok := c.wires[e]
	if !ok {
		return 0, fmt.Errorf("circuit value: invalid: %s", e)
	}
	switch w := wire.(type) {
	case *nonary:
		return c.Eval(w.r)
	case *unary:
		v, err := c.Eval(w.r)
		if err != nil {
			return 0, err
		}
		return w.op(v), nil
	case *binary:
		v1, err := c.Eval(w.r1)
		if err != nil {
			return 0, err
		}
		v2, err := c.Eval(w.r2)
		if err != nil {
			return 0, err
		}
		return w.op(v1, v2), nil
	default:
		return 0, fmt.Errorf("circuit value: unknown wire type: %T", w)
	}
}
