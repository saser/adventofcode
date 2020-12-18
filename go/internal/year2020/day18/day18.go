package day18

import (
	"fmt"
	"strconv"
	"strings"
	"unicode"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type tokenKind int

const (
	number tokenKind = iota
	open
	close
	plus
	mult
)

type token struct {
	kind  tokenKind
	value int // only set if kind == number
}

func (t token) String() string {
	switch t.kind {
	case number:
		return fmt.Sprintf("number(%v)", t.value)
	case open:
		return "open"
	case close:
		return "close"
	case plus:
		return "plus"
	case mult:
		return "mult"
	default:
		return "<invalid token>"
	}
}

var (
	openToken  = token{kind: open}
	closeToken = token{kind: close}
	plusToken  = token{kind: plus}
	multToken  = token{kind: mult}
)

func tokenize(line string) []token {
	var tokens []token
	for i := 0; i < len(line); i++ {
		r := rune(line[i])
		switch {
		case unicode.IsDigit(r):
			idx := strings.IndexFunc(line[i:], func(r rune) bool { return !unicode.IsDigit(r) })
			end := i + idx
			if idx == -1 {
				end = len(line)
			}
			n, err := strconv.Atoi(line[i:end])
			if err != nil {
				panic(err)
			}
			tokens = append(tokens, token{
				kind:  number,
				value: n,
			})
			i += end - i - 1
		case r == '(':
			tokens = append(tokens, openToken)
		case r == ')':
			tokens = append(tokens, closeToken)
		case r == '+':
			tokens = append(tokens, plusToken)
		case r == '*':
			tokens = append(tokens, multToken)
		case r == ' ':
			// do nothing
		}
	}
	return tokens
}

type op int

const (
	opPlus op = iota
	opMult
)

type evaluator struct {
	valStack []int
	opStack  []op
}

func newEvaluator() *evaluator {
	return &evaluator{
		valStack: []int{0},
		opStack:  []op{opPlus},
	}
}

func (e *evaluator) pushVal(v int) {
	e.valStack = append(e.valStack, v)
}

func (e *evaluator) popVal() int {
	v := e.valStack[len(e.valStack)-1]
	e.valStack = e.valStack[:len(e.valStack)-1]
	return v
}

func (e *evaluator) pushOp(o op) {
	e.opStack = append(e.opStack, o)
}

func (e *evaluator) popOp() op {
	o := e.opStack[len(e.opStack)-1]
	e.opStack = e.opStack[:len(e.opStack)-1]
	return o
}

func (e *evaluator) fold() {
	v1 := e.popVal()
	v2 := e.popVal()
	topOp := e.popOp()
	var v3 int
	switch topOp {
	case opPlus:
		v3 = v1 + v2
	case opMult:
		v3 = v1 * v2
	}
	e.pushVal(v3)
}

func eval(tokens []token) int {
	e := newEvaluator()
	for _, t := range tokens {
		switch t.kind {
		case number:
			e.pushVal(t.value)
			e.fold()
		case open:
			e.pushVal(0)
			e.pushOp(opPlus)
		case close:
			e.fold()
		case plus:
			e.pushOp(opPlus)
		case mult:
			e.pushOp(opMult)
		}
	}
	return e.popVal()
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	lines := strings.Split(strings.TrimSpace(input), "\n")
	sum := 0
	for _, line := range lines {
		sum += eval(tokenize(line))
	}
	return fmt.Sprint(sum), nil
}
