package day15

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	ingredientMap, err := parse(input)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 15, part %d: %w", part, err)
	}
	ingredients := make([]ingredient, 0, len(ingredientMap))
	for _, ingredient := range ingredientMap {
		ingredients = append(ingredients, ingredient)
	}
	maxScore := 0
	for _, distribution := range distributions(100, len(ingredients)) {
		s := score(distribution, ingredients, part)
		if s > maxScore {
			maxScore = s
		}
	}
	return fmt.Sprint(maxScore), nil
}

type ingredient struct {
	capacity   int
	durability int
	flavor     int
	texture    int
	calories   int
}

func parse(input string) (map[string]ingredient, error) {
	re, err := regexp.Compile(`^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$`)
	if err != nil {
		return nil, fmt.Errorf("parse: %w", err)
	}
	m := make(map[string]ingredient)
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
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

func score(distribution []int, ingredients []ingredient, part int) int {
	capacity := 0
	durability := 0
	flavor := 0
	texture := 0
	calories := 0
	for i, part := range distribution {
		ingredient := ingredients[i]
		capacity += ingredient.capacity * part
		durability += ingredient.durability * part
		flavor += ingredient.flavor * part
		texture += ingredient.texture * part
		calories += ingredient.calories * part
	}
	if part == 2 && calories != 500 {
		return 0
	}
	if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
		return 0
	}
	return capacity * durability * flavor * texture
}
