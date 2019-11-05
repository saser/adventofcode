package day07

import (
	"errors"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return "", errors.New("not yet implemented")
}

type wire interface {
	wire()
}

type nonary struct {
	r string
}

func (n *nonary) Wire() {}

type unaryOp func(uint16) uint16

type unary struct {
	r  string
	op unaryOp
}

func (u *unary) Wire() {}

type binaryOp func(uint16, uint16) uint16

type binary struct {
	r1 string
	r2 string
	op binaryOp
}

func (b *binary) Wire() {}

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
