package inputs

import (
	"fmt"
	"os"
	"path/filepath"
)

var InputsDir = filepath.Join("..", "..", "..", "inputs")

func Open(year, day int) (*os.File, error) {
	path := filepath.Join(InputsDir, fmt.Sprint(year), fmt.Sprintf("%02d", day))
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("open input: %w", err)
	}
	return file, nil
}
