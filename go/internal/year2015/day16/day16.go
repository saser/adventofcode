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
	sues, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 16, part 1: %w", err)
	}
	fmt.Println(sues)
	return "", errors.New("not implemented yet")
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
