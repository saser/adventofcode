package solution

import "io"

type Solution func(io.Reader) (string, error)

type Solution2 func(string) (string, error)
