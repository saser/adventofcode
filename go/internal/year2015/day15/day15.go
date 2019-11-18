package day15

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"regexp"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	ingredientMap, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 15, part 1: %w", err)
	}
	fmt.Println(ingredientMap)
	return "", errors.New("not yet implemented")
}

type ingredient struct {
	capacity   int
	durability int
	flavor     int
	texture    int
	calories   int
}

func parse(r io.Reader) (map[string]ingredient, error) {
	re, err := regexp.Compile(`^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$`)
	if err != nil {
		return nil, fmt.Errorf("parse: %w", err)
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	m := make(map[string]ingredient)
	for sc.Scan() {
		line := sc.Text()
		matches := re.FindStringSubmatch(line)
		if matches == nil {
			return nil, fmt.Errorf("parse: invalid line: %s", line)
		}
		capacity, err := strconv.Atoi(matches[2])
		if err != nil {
			return nil, fmt.Errorf("parse: invalid capacity: %s", matches[2])
		}
		durability, err := strconv.Atoi(matches[3])
		if err != nil {
			return nil, fmt.Errorf("parse: invalid durability: %s", matches[3])
		}
		flavor, err := strconv.Atoi(matches[4])
		if err != nil {
			return nil, fmt.Errorf("parse: invalid flavor: %s", matches[4])
		}
		texture, err := strconv.Atoi(matches[5])
		if err != nil {
			return nil, fmt.Errorf("parse: invalid texture: %s", matches[5])
		}
		calories, err := strconv.Atoi(matches[6])
		if err != nil {
			return nil, fmt.Errorf("parse: invalid calories: %s", matches[6])
		}
		m[matches[1]] = ingredient{
			capacity:   capacity,
			durability: durability,
			flavor:     flavor,
			texture:    texture,
			calories:   calories,
		}
	}
	return m, nil
}

func distributions(sum int, parts int) [][]int {
	if parts == 1 {
		return [][]int{{sum}}
	}
	ds := make([][]int, 0)
	for i := 0; i <= sum; i++ {
		for _, sub := range distributions(sum-i, parts-1) {
			ds = append(ds, append([]int{i}, sub...))
		}
	}
	return ds
}
