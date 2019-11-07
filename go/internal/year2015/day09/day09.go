package day09

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"regexp"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	distances, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 09, part 1: %w", err)
	}
	places := make([]string, 0, len(distances))
	for k, _ := range distances {
		places = append(places, k)
	}
	routes := permutations(places)
	answer := minDistance(routes, distances)
	return fmt.Sprint(answer), nil
}

func Part2(r io.Reader) (string, error) {
	return "", errors.New("not yet implemented")
}

func parse(r io.Reader) (map[string]map[string]int, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)

	re, err := regexp.Compile(`^(\w+) to (\w+) = (\d+)$`)
	if err != nil {
		return nil, fmt.Errorf("parse: %w", err)
	}
	distances := make(map[string]map[string]int)
	for sc.Scan() {
		line := sc.Text()
		matches := re.FindStringSubmatch(line)
		if matches == nil {
			return nil, fmt.Errorf("parse: invalid line: %s", line)
		}
		from := matches[1]
		to := matches[2]
		distance, err := strconv.Atoi(matches[3])
		if err != nil {
			return nil, fmt.Errorf("parse: invalid distance: %s", matches[3])
		}
		if _, ok := distances[from]; !ok {
			distances[from] = make(map[string]int)
		}
		distances[from][to] = int(distance)
		if _, ok := distances[to]; !ok {
			distances[to] = make(map[string]int)
		}
		distances[to][from] = int(distance)
	}
	return distances, nil
}

func minDistance(routes [][]string, distances map[string]map[string]int) int {
	minCost := -1
	for _, route := range routes {
		cost := 0
		for i := 0; i < len(route)-1; i++ {
			from := route[i]
			to := route[i+1]
			cost += distances[from][to]
		}
		if minCost == -1 || cost < minCost {
			minCost = cost
		}
	}
	return minCost
}

func permutations(strings []string) [][]string {
	n := len(strings)
	if n == 1 {
		return [][]string{strings}
	}
	perms := make([][]string, 0, factorial(n))
	for i := 0; i < n; i++ {
		rest := make([]string, 0, n-1)
		rest = append(rest, strings[:i]...)
		rest = append(rest, strings[i+1:]...)
		for _, subperm := range permutations(rest) {
			perm := make([]string, 1, n)
			perm[0] = strings[i]
			perm = append(perm, subperm...)
			perms = append(perms, perm)
		}
	}
	return perms
}

func factorial(n int) int {
	if n == 0 {
		return 1
	} else {
		return n * factorial(n-1)
	}
}
