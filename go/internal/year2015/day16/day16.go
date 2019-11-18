package day16

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"regexp"
	"strconv"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	sues, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 16, part %d: %w", part, err)
	}
	query := sue{
		children:    3,
		cats:        7,
		samoyeds:    2,
		pomeranians: 3,
		akitas:      0,
		vizslas:     0,
		goldfish:    5,
		trees:       3,
		cars:        2,
		perfumes:    1,
	}
	for i, s := range sues {
		if s.matches(query, part) {
			return fmt.Sprint(i + 1), nil
		}
	}
	return "", errors.New("no matching Sue")
}

type sue struct {
	children    int
	cats        int
	samoyeds    int
	pomeranians int
	akitas      int
	vizslas     int
	goldfish    int
	trees       int
	cars        int
	perfumes    int
}

func (s *sue) matches(query sue, part int) bool {
	type matcher func(int, int) bool
	equal := func(a, b int) bool {
		return a == b
	}
	greaterThan := func(a, b int) bool {
		return a > b
	}
	lessThan := func(a, b int) bool {
		return a < b
	}
	var catsMatch, treesMatch, pomeraniansMatch, goldfishMatch matcher
	switch part {
	case 1:
		catsMatch = equal
		treesMatch = equal
		pomeraniansMatch = equal
		goldfishMatch = equal
	case 2:
		catsMatch = greaterThan
		treesMatch = greaterThan
		pomeraniansMatch = lessThan
		goldfishMatch = lessThan
	}
	for _, tt := range []struct {
		si    int
		qi    int
		match matcher
	}{
		{si: s.children, qi: query.children, match: equal},
		{si: s.cats, qi: query.cats, match: catsMatch},
		{si: s.samoyeds, qi: query.samoyeds, match: equal},
		{si: s.pomeranians, qi: query.pomeranians, match: pomeraniansMatch},
		{si: s.akitas, qi: query.akitas, match: equal},
		{si: s.vizslas, qi: query.vizslas, match: equal},
		{si: s.goldfish, qi: query.goldfish, match: goldfishMatch},
		{si: s.trees, qi: query.trees, match: treesMatch},
		{si: s.cars, qi: query.cars, match: equal},
		{si: s.perfumes, qi: query.perfumes, match: equal},
	} {
		if tt.si != -1 && !tt.match(tt.si, tt.qi) {
			return false
		}
	}
	return true
}

func parse(r io.Reader) ([]sue, error) {
	outerRE, err := regexp.Compile(`^Sue (\d+): (.+)$`)
	if err != nil {
		return nil, fmt.Errorf("parse: %w", err)
	}
	attributeRE, err := regexp.Compile(`(\w+): (\d+)`)
	if err != nil {
		return nil, fmt.Errorf("parse: %w", err)
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	sues := make([]sue, 500)
	for sc.Scan() {
		line := sc.Text()
		outerMatches := outerRE.FindStringSubmatch(line)
		if outerMatches == nil {
			return nil, fmt.Errorf("parse: invalid line: %s", line)
		}
		s := sue{
			children:    -1,
			cats:        -1,
			samoyeds:    -1,
			pomeranians: -1,
			akitas:      -1,
			vizslas:     -1,
			goldfish:    -1,
			trees:       -1,
			cars:        -1,
			perfumes:    -1,
		}
		for _, part := range strings.Split(outerMatches[2], ", ") {
			attributeMatches := attributeRE.FindStringSubmatch(part)
			if attributeMatches == nil {
				return nil, fmt.Errorf("parse: invalid line: %s", line)
			}
			n, err := strconv.Atoi(attributeMatches[2])
			if err != nil {
				return nil, fmt.Errorf("parse: invalid %s: %s", attributeMatches[1], attributeMatches[2])
			}
			var attribute *int
			switch attributeMatches[1] {
			case "children":
				attribute = &s.children
			case "cats":
				attribute = &s.cats
			case "samoyeds":
				attribute = &s.samoyeds
			case "pomeranians":
				attribute = &s.pomeranians
			case "akitas":
				attribute = &s.akitas
			case "vizslas":
				attribute = &s.vizslas
			case "goldfish":
				attribute = &s.goldfish
			case "trees":
				attribute = &s.trees
			case "cars":
				attribute = &s.cars
			case "perfumes":
				attribute = &s.perfumes
			}
			*attribute = n
		}
		i, err := strconv.Atoi(outerMatches[1])
		if err != nil {
			return nil, fmt.Errorf("parse: invalid Sue number: %s", outerMatches[1])
		}
		sues[i-1] = s
	}
	return sues, nil
}
