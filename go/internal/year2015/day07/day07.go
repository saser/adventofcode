package day07

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"regexp"
	"strconv"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	wires, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 07, part 1: %w", err)
	}
	fmt.Println(wires)
	return "", errors.New("not yet implemented")
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
