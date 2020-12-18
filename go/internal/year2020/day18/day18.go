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
func parse(tokens []token, precedences map[tokenKind]int) []token {
	var out, ops tokenStack
	for _, t := range tokens {
		switch t.kind {
		case number:
			out.push(t)
		case plus, mult:
			prec := precedences[t.kind]
			for op, ok := ops.peek(); ok && precedences[op.kind] >= prec && op.kind != open; op, ok = ops.peek() {
				out.push(op)
				ops.pop()
			}
			ops.push(t)
		case open:
			ops.push(t)
		case close:
			for op, ok := ops.peek(); ok && op.kind != open; op, ok = ops.peek() {
				out.push(op)
				ops.pop()
			}
			ops.pop() // discard the open parenthesis
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
	switch op.kind {
	case plus:
		v3 = v1 + v2
	case mult:
		v3 = v1 * v2
	}
	s.push(v3)
}

// eval evaluates the given expression in reverse Polish notation.
func eval(rpn []token) int {
	var vals valStack
	for _, t := range rpn {
		switch t.kind {
		case number:
			vals.push(t.value)
		case plus, mult:
			vals.fold(t)
		}
	}
	return vals[0]
}

func solve(input string, part int) (string, error) {
	var precedences map[tokenKind]int
	switch part {
	case 1:
		precedences = map[tokenKind]int{
			plus: 0,
			mult: 0,
		}
	case 2:
		precedences = map[tokenKind]int{
			plus: 1,
			mult: 0,
		}
	}
	lines := strings.Split(strings.TrimSpace(input), "\n")
	c := make(chan int, len(lines))
	for _, line := range lines {
		go func(line string) {
			tokens := tokenize(line)
			rpn := parse(tokens, precedences)
			c <- eval(rpn)
		}(line)
	}
	sum := 0
	for i := 0; i < len(lines); i++ {
		sum += <-c
	}
	return fmt.Sprint(sum), nil
}
