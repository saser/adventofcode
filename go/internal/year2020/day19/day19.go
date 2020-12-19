package day19

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

type matcher interface {
	Match(string) (bool, string)
}

type byteMatcher byte

func (m byteMatcher) Match(s string) (bool, string) {
	if len(s) == 0 {
		return false, s
	}
	return s[0] == byte(m), s[1:]
}

type seqMatcher []matcher

func (m seqMatcher) Match(s string) (bool, string) {
	ok := true
	rest := s
	for _, m2 := range m {
		ok, rest = m2.Match(rest)
		if !ok {
			break
		}
	}
	return ok, rest
}

type altMatcher []matcher

func (m altMatcher) Match(s string) (bool, string) {
	for _, m2 := range m {
		ok, rest := m2.Match(s)
		if ok {
			return true, rest
		}
	}
	return false, s
}

type ruleMatcher struct {
	terminals map[int]byte    // rule number -> single byte
	subrules  map[int][][]int // rule number -> list of alternatives (in the form of sequences of rule numbers)
	cache     map[int]matcher // rule number -> matcher
}

func parse(paragraph string) ruleMatcher {
	lines := strings.Split(strings.TrimSpace(paragraph), "\n")
	terminals := make(map[int]byte)
	subrules := make(map[int][][]int)
	for _, line := range lines {
		parts := strings.Split(line, ": ")
		rule, err := strconv.Atoi(parts[0])
		if err != nil {
			panic(err)
		}
		if parts[1][0] == '"' {
			terminals[rule] = parts[1][1]
			continue
		}
		seqStrings := strings.Split(parts[1], " | ")
		alternatives := make([][]int, len(seqStrings))
		for i, seqString := range seqStrings {
			rules := strings.Split(seqString, " ")
			seq := make([]int, len(rules))
			for i, s := range rules {
				n, err := strconv.Atoi(s)
				if err != nil {
					panic(err)
				}
				seq[i] = n
			}
			alternatives[i] = seq
		}
		subrules[rule] = alternatives
	}
	return ruleMatcher{
		terminals: terminals,
		subrules:  subrules,
		cache:     make(map[int]matcher, len(terminals)+len(subrules)),
	}
}

func (r ruleMatcher) get(rule int) (m matcher) {
	if m, ok := r.cache[rule]; ok {
		return m
	}
	defer func() {
		r.cache[rule] = m
	}()
	if term, ok := r.terminals[rule]; ok {
		return byteMatcher(term)
	}
	if alternatives, ok := r.subrules[rule]; ok {
		am := make(altMatcher, 0, len(alternatives))
		for _, seq := range alternatives {
			sm := make(seqMatcher, 0, len(seq))
			for _, i := range seq {
				sm = append(sm, r.get(i))
			}
			am = append(am, sm)
		}
		return am
	}
	panic(fmt.Errorf("unknown rule number: %v", rule))
}

func (r ruleMatcher) Match(s string) (bool, string) {
	return r.get(0).Match(s)
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	paragraphs := strings.Split(strings.TrimSpace(input), "\n\n")
	r := parse(paragraphs[0])
	n := 0
	for _, line := range strings.Split(paragraphs[1], "\n") {
		if ok, rest := r.Match(line); ok && rest == "" {
			n++
		}
	}
	return fmt.Sprint(n), nil
}
