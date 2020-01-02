package day25

import (
	"fmt"
	"io"
	"io/ioutil"
	"math/big"
	"regexp"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	row, col, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 25, part %d: %w", part, err)
	}
	a := big.NewInt(20151125)
	base := big.NewInt(252533)
	exp := big.NewInt(int64(rcToIndex(row, col)) - 1)
	mod := big.NewInt(33554393)
	b := big.NewInt(0)
	b.Exp(base, exp, mod)
	b.Mul(a, b)
	b.Mod(b, mod)
	return b.String(), nil
}

func parse(r io.Reader) (int, int, error) {
	re, err := regexp.Compile(`\d+`)
	if err != nil {
		return 0, 0, fmt.Errorf("parse: %w", err)
	}
	bytes, err := ioutil.ReadAll(r)
	if err != nil {
		return 0, 0, fmt.Errorf("parse: %w", err)
	}
	matches := re.FindAllString(string(bytes), -1)
	row, err := strconv.Atoi(matches[0])
	if err != nil {
		return 0, 0, fmt.Errorf("parse: %w", err)
	}
	col, err := strconv.Atoi(matches[1])
	if err != nil {
		return 0, 0, fmt.Errorf("parse: %w", err)
	}
	return row, col, nil
}

func rcToIndex(row, col int) int {
	return 1 + sumNatural(row+col-1) - row
}

func sumNatural(n int) int {
	return (n * (n + 1)) / 2
}
