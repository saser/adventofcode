package day16

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

type rule struct {
	name                     string
	low1, high1, low2, high2 int
}

func parseRule(line string) rule {
	var r rule
	parts := strings.Split(line, ": ")
	r.name = parts[0]
	if _, err := fmt.Sscanf(parts[1], "%d-%d or %d-%d", &r.low1, &r.high1, &r.low2, &r.high2); err != nil {
		panic(err)
	}
	return r
}

func (r rule) Matches(n int) bool {
	return (n >= r.low1 && n <= r.high1) || (n >= r.low2 && n <= r.high2)
}

type ruleSet map[string]rule // rule name -> rule

func parseRules(paragraph string) ruleSet {
	lines := strings.Split(strings.TrimSpace(paragraph), "\n")
	rs := make(ruleSet, len(lines))
	for _, line := range lines {
		r := parseRule(line)
		rs[r.name] = r
	}
	return rs
}

func (rs ruleSet) MatchesNone(n int) bool {
	for _, r := range rs {
		if r.Matches(n) {
			return false
		}
	}
	return true
}

type ticket []int

func parseTicket(line string) ticket {
	parts := strings.Split(line, ",")
	t := make(ticket, len(parts))
	for i, part := range parts {
		n, err := strconv.Atoi(part)
		if err != nil {
			panic(err)
		}
		t[i] = n
	}
	return t
}

func (t ticket) Invalid(rs ruleSet) []int {
	var invalid []int
	for _, n := range t {
		if rs.MatchesNone(n) {
			invalid = append(invalid, n)
		}
	}
	return invalid
}

func parseTickets(paragraph string) []ticket {
	lines := strings.Split(strings.TrimSpace(paragraph), "\n")[1:] // skip header line
	tickets := make([]ticket, len(lines))
	for i, line := range lines {
		tickets[i] = parseTicket(line)
	}
	return tickets
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	paragraphs := strings.Split(input, "\n\n")
	rs := parseRules(paragraphs[0])
	nearby := parseTickets(paragraphs[2])
	sum := 0
	for _, t := range nearby {
		for _, n := range t.Invalid(rs) {
			sum += n
		}
	}
	return fmt.Sprint(sum), nil
}
