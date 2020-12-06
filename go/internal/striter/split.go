package striter

import "strings"

func Split(s, sep string, f func(int, string) bool) {
	for i, ss := range strings.Split(s, sep) {
		if !f(i, ss) {
			break
		}
	}
}
