package day13

import (
	"errors"
	"fmt"
	"io"
	"regexp"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	return "", errors.New("not yet implemented")
}

type preference struct {
	from   string
	to     string
	change int
}

func parsePreference(s string) (preference, error) {
	re, err := regexp.Compile(`^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.$`)
	if err != nil {
		return preference{}, fmt.Errorf("parse preference: %w", err)
	}
	matches := re.FindStringSubmatch(s)
	if matches == nil {
		return preference{}, fmt.Errorf("parse preference: invalid preference string: %s", s)
	}
	var sign int
	switch matches[2] {
	case "gain":
		sign = 1
	case "lose":
		sign = -1
	}
	change, err := strconv.Atoi(matches[3])
	if err != nil {
		return preference{}, fmt.Errorf("parse preference: %w", err)
	}
	change *= sign
	p := preference{
		from:   matches[1],
		to:     matches[4],
		change: change,
	}
	return p, nil
}
