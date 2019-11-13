package day14

import (
	"bufio"
	"fmt"
	"io"
	"regexp"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	deers, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 14, part 1: %w", err)
	}
	time := 2503
	funcs := make([]distanceFunc, len(deers))
	for i, deer := range deers {
		funcs[i] = deerDistanceFunc(deer)
	}
	switch part {
	case 1:
		var maxDistance int
		for _, f := range funcs {
			distance := f(time)
			if distance > maxDistance {
				maxDistance = distance
			}
		}
		return fmt.Sprint(maxDistance), nil
	case 2:
		scores := make(map[string]int)
		for second := 1; second <= time; second++ {
			var leader string
			var maxDistance int
			for i, deer := range deers {
				distance := funcs[i](second)
				if distance > maxDistance {
					maxDistance = distance
					leader = deer.name
				}
			}
			score, ok := scores[leader]
			if !ok {
				score = 0
			}
			scores[leader] = score + 1
		}
		var maxScore int
		for _, score := range scores {
			if score > maxScore {
				maxScore = score
			}
		}
		return fmt.Sprint(maxScore), nil
	default:
		return "", fmt.Errorf("year 2015, day 14: invalid part: %d", part)
	}
}

type distanceFunc func(int) int

type deer struct {
	name          string
	speed         int
	flyingPeriod  int
	restingPeriod int
}

func parse(r io.Reader) ([]deer, error) {
	re, err := regexp.Compile(`^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$`)
	if err != nil {
		return nil, fmt.Errorf("parse: %w", err)
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	deers := make([]deer, 0)
	for sc.Scan() {
		line := sc.Text()
		matches := re.FindStringSubmatch(line)
		if matches == nil {
			return nil, fmt.Errorf("parse: invalid line: %s", line)
		}
		speed, err := strconv.Atoi(matches[2])
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		flyingPeriod, err := strconv.Atoi(matches[3])
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		restingPeriod, err := strconv.Atoi(matches[4])
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		deers = append(deers, deer{
			name:          matches[1],
			speed:         speed,
			flyingPeriod:  flyingPeriod,
			restingPeriod: restingPeriod,
		})
	}
	return deers, nil
}

func deerDistanceFunc(d deer) distanceFunc {
	return func(x int) int {
		episode := d.flyingPeriod + d.restingPeriod
		nEpisodes := x / episode
		rest := x % episode
		if rest > d.flyingPeriod {
			rest = d.flyingPeriod
		}
		return d.speed * (d.flyingPeriod*nEpisodes + rest)
	}
}
