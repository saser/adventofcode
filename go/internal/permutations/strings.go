package permutations

func Strings(strings []string) [][]string {
	n := len(strings)
	if n == 1 {
		return [][]string{strings}
	}
	perms := make([][]string, 0, factorial(n))
	for i := 0; i < n; i++ {
		rest := make([]string, 0, n-1)
		rest = append(rest, strings[:i]...)
		rest = append(rest, strings[i+1:]...)
		for _, subperm := range Strings(rest) {
			perm := make([]string, 1, n)
			perm[0] = strings[i]
			perm = append(perm, subperm...)
			perms = append(perms, perm)
		}
	}
	return perms
}
