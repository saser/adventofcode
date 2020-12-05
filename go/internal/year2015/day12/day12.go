package day12

import (
	"encoding/json"
	"fmt"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	var j interface{}
	if err := json.Unmarshal([]byte(input), &j); err != nil {
		return "", fmt.Errorf("year 2015, day 12, part 2: %w", err)
	}
	return fmt.Sprint(sumJSON(j, part)), nil
}

func sumJSON(j interface{}, part int) int {
	switch v := j.(type) {
	case float64:
		return int(v)
	case string:
		return 0
	case []interface{}:
		sum := 0
		for _, k := range v {
			sum += sumJSON(k, part)
		}
		return sum
	case map[string]interface{}:
		sum := 0
		for _, k := range v {
			if s, ok := k.(string); ok && part == 2 && s == "red" {
				return 0
			}
			sum += sumJSON(k, part)
		}
		return sum
	}
	panic(fmt.Sprintf("invalid JSON value: %v (%T)", j, j))
}
