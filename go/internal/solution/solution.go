package solution

import "io"

type Solution func(io.Reader) (string, error)
