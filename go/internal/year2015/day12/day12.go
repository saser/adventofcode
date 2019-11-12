package day12

import (
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
)

func Part1(r io.Reader) (string, error) {
	decoder := json.NewDecoder(r)
	var sum int64
	for {
		token, err := decoder.Token()
		if err == io.EOF {
			break
		}
		if v, ok := token.(float64); ok {
			sum += int64(v)
		}
	}
	return fmt.Sprint(sum), nil
}

func Part2(r io.Reader) (string, error) {
	var j interface{}
	bytes, err := ioutil.ReadAll(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 12, part 2: %w", err)
	}
	if err := json.Unmarshal(bytes, &j); err != nil {
		return "", fmt.Errorf("year 2015, day 12, part 2: %w", err)
	}
	return fmt.Sprint(sumJSON(j, 2)), nil
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
