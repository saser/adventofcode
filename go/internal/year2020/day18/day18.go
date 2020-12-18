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

type term int

const (
	termNumber term = iota
	termLPar
	termRPar
	termPlus
	termMult
)

type token struct {
	term  term
	value int // only set if kind == number
}

func (t token) String() string {
	switch t.term {
	case termNumber:
		return fmt.Sprintf("number(%v)", t.value)
	case termLPar:
		return "("
	case termRPar:
		return ")"
	case termPlus:
		return "plus"
	case termMult:
		return "mult"
	default:
		return "<invalid token>"
	}
}

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
				term:  termNumber,
				value: n,
			})
			i += end - i - 1
		case r == '(':
			tokens = append(tokens, token{term: termLPar})
		case r == ')':
			tokens = append(tokens, token{term: termRPar})
		case r == '+':
			tokens = append(tokens, token{term: termPlus})
		case r == '*':
			tokens = append(tokens, token{term: termMult})
		case r == ' ':
			// do nothing
		}
	}
	return tokens
}

type tokenStack []token

func (s *tokenStack) peek() (token, bool) {
	n := len(*s)
	if n == 0 {
		return token{}, false
	}
	return (*s)[n-1], true
}

func (s *tokenStack) push(t token) {
	*s = append(*s, t)
}

func (s *tokenStack) pop() (token, bool) {
	t, ok := s.peek()
	if ok {
		*s = (*s)[:len(*s)-1]
	}
	return t, ok
}

// parse uses the shunting-yard algorithm to parse the given tokens to
// a new list of tokens in reverse Polish notation. Operator
// precedences are given using the precedences map, where a higher
// number means higher precedence.
func parse(tokens []token, precedences map[term]int) []token {
	var out, ops tokenStack
	for _, t := range tokens {
		switch t.term {
		case termNumber:
			out.push(t)
		case termPlus, termMult:
			prec := precedences[t.term]
			for op, ok := ops.peek(); ok && precedences[op.term] >= prec && op.term != termLPar; op, ok = ops.peek() {
				out.push(op)
				ops.pop()
			}
			ops.push(t)
		case termLPar:
			ops.push(t)
		case termRPar:
			for op, ok := ops.peek(); ok && op.term != termLPar; op, ok = ops.peek() {
				out.push(op)
				ops.pop()
			}
			ops.pop() // discard the left parenthesis
		}
	}
	for op, ok := ops.pop(); ok; op, ok = ops.pop() {
		out.push(op)
	}
	return out
}

type valStack []int

func (s *valStack) push(v int) {
	*s = append(*s, v)
}

func (s *valStack) pop() int {
	last := len(*s) - 1
	v := (*s)[last]
	*s = (*s)[:last]
	return v
}

func (s *valStack) fold(op token) {
	v1 := s.pop()
	v2 := s.pop()
	var v3 int
	switch op.term {
	case termPlus:
		v3 = v1 + v2
	case termMult:
		v3 = v1 * v2
	}
	s.push(v3)
}

// eval evaluates the given expression in reverse Polish notation.
func eval(rpn []token) int {
	var vals valStack
	for _, t := range rpn {
		switch t.term {
		case termNumber:
			vals.push(t.value)
		case termPlus, termMult:
			vals.fold(t)
		}
	}
	return vals[0]
}

func solve(input string, part int) (string, error) {
	var precedences map[term]int
	switch part {
	case 1:
		precedences = map[term]int{
			termPlus: 0,
			termMult: 0,
		}
	case 2:
		precedences = map[term]int{
			termPlus: 1,
			termMult: 0,
		}
	}
	lines := strings.Split(strings.TrimSpace(input), "\n")
	sum := 0
	for _, line := range lines {
		tokens := tokenize(line)
		rpn := parse(tokens, precedences)
		sum += eval(rpn)
	}
	return fmt.Sprint(sum), nil
}
