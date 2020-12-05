package day11

import (
	"strings"
)

const (
	digits = "abcdefghijklmnopqrstuvwxyz"
	base   = len(digits)
)

type requirement func(string) bool

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, count int) (string, error) {
	password := strings.TrimSpace(input)
	requirements := []requirement{
		hasIncreasing,
		hasNoIOL,
		hasTwoPairs,
	}
	nextPassword := password
	for i := 0; i < count; i++ {
		nextPassword = findNext(nextPassword, requirements)
	}
	return nextPassword, nil
}

func findNext(password string, requirements []requirement) string {
	nextPassword := password
outer:
	for {
		nextPassword = next(nextPassword)
		for _, requirement := range requirements {
			if !requirement(nextPassword) {
				continue outer
			}
		}
		break
	}
	return nextPassword
}

func digitsToInts(s string) []int {
	is := make([]int, len(s))
	for i, r := range s {
		is[i] = strings.IndexRune(digits, r)
	}
	return is
}

func intsToDigits(is []int) string {
	var sb strings.Builder
	for _, i := range is {
		sb.WriteByte(digits[i])
	}
	return sb.String()
}

func next(s string) string {
	is := digitsToInts(s)
	if idx := strings.IndexAny(s, "iol"); idx != -1 {
		is[idx]++
		for i := idx + 1; i < len(is); i++ {
			is[i] = 0
		}
		return intsToDigits(is)
	}
	increment := true
	for i := len(is) - 1; i >= 0; i-- {
		if increment {
			is[i] = (is[i] + 1) % base
			increment = is[i] == 0
		} else {
			break
		}
	}
	if increment {
		is = append([]int{0}, is...)
	}
	return intsToDigits(is)
}

func hasIncreasing(s string) bool {
	for i := 0; i < len(s)-2; i++ {
		r := s[i]
		if s[i+1] == r+1 && s[i+2] == r+2 {
			return true
		}
	}
	return false
}

func hasNoIOL(s string) bool {
	for _, r := range s {
		switch r {
		case 'i', 'o', 'l':
			return false
		}
	}
	return true
}

func hasTwoPairs(s string) bool {
	pairs := 0
	for i := 0; i < len(s)-1; i++ {
		if s[i] == s[i+1] {
			pairs++
			i++
		}
		if pairs == 2 {
			return true
		}
	}
	return false
}
