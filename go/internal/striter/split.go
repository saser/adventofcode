package striter

import "strings"

func Split(s, sep string, f func(int, string) bool) {
	i := 0
	start := 0
	for {
		offset := strings.Index(s[start:], sep)
		if offset == -1 {
			f(i, s[start:])
			break
		}
		end := start + offset
		if !f(i, s[start:end]) {
			break
		}
		start = end + len(sep)
		i++
	}
}
