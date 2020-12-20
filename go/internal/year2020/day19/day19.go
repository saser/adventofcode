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

// part2Matcher is a hacky solution for part 2. It rests on the assumption that
// rule 0 will always be rule 8 followed by rule 11. Rule 8 means "rule 42 k
// times, k > 0" and rule 11 means "rule 42 m times followed by rule 31 m times,
// m > 0". Combined it becomes "rule 42 (k + m) times followed by rule 31 m
// times, k > 0, m > 0".
type part2Matcher struct {
	rule42, rule31 matcher
}

func (m part2Matcher) Match(s string) (bool, string) {
	n42 := 0
	ok := true
	rest := s
	for ok, rest = m.rule42.Match(rest); ok; ok, rest = m.rule42.Match(rest) {
		n42++
	}
	n31 := 0
	for ok, rest = m.rule31.Match(rest); ok; ok, rest = m.rule31.Match(rest) {
		n31++
	}
	return n42 > 0 && n31 > 0 && n42 > n31, rest
}

func solve(input string, part int) (string, error) {
	paragraphs := strings.Split(strings.TrimSpace(input), "\n\n")
	r := parse(paragraphs[0])
	if part == 2 {
		rule42 := r.get(42)
		rule31 := r.get(31)
		r.cache[0] = part2Matcher{rule42: rule42, rule31: rule31}
	}
	n := 0
	for _, line := range strings.Split(paragraphs[1], "\n") {
		if ok, rest := r.Match(line); ok && rest == "" {
			n++
		}
	}
	return fmt.Sprint(n), nil
}
