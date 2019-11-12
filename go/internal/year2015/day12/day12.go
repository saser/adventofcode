package day12

import (
	"encoding/json"
	"fmt"
	"io"
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
