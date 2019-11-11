package day11

import (
	"errors"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return "", errors.New("not yet implemented")
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
