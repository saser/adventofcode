package day07

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

// parseLine parses a single line into the bag being described, and what it can
// contain.
func parseLine(line string) (string, map[string]int) {
	// The lines look like this:
	//
	//     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
	//
	// The general parsing idea is as follows:
	// 1. Split around the string " contain ", resulting in these two parts:
	//        "shiny gold bags"                        // key
	//        "1 dark olive bag, 2 vibrant plum bags." // contents
	// 2. For key, trim off the suffix " bags". The result is the return
	//    value of key.
	// 3. For contents, trim off the trailing period. After that, contents
	//    can be split around ", ", resulting in these two parts:
	//        "1 dark olive bag"    // item1
	//        "2 vibrant plum bags" // item2
	// 4. For all itemN:
	//     a. Trim off any "bag(s)" suffix.
	//     b. Then, split the string around the first space. The first part
	//        of that split is then the number, and the second part is the
	//        bag description. In the contents return value, store a mapping
	//        from the bag description to the number.

	// Step 1.
	parts := strings.Split(line, " contain ")
	// Step 2.
	key := strings.TrimSuffix(parts[0], " bags")
	// Step 3.
	contentsStr := parts[1]
	if contentsStr == "no other bags." {
		return key, nil
	}
	items := strings.Split(strings.TrimSuffix(parts[1], "."), ", ")
	// Step 4.
	contents := make(map[string]int)
	for _, item := range items {
		// Step 4a.
		if item[len(item)-1] == 's' {
			item = item[:len(item)-1]
		}
		item = strings.TrimSuffix(item, " bag")
		// Step 4b.
		parts := strings.SplitN(item, " ", 2)
		number, err := strconv.Atoi(parts[0])
		if err != nil {
			panic(err)
		}
		bagDesc := parts[1]
		contents[bagDesc] = number
	}
	return key, contents
}

// parse returns a mapping from a bag to its contents. containedBy has its name
// given by the fact that, for example, containedBy["shiny gold"] gives the list
// of bags that are contained in a shiny gold bag.
func parse(input string) (containedBy map[string]map[string]int) {
	m := make(map[string]map[string]int)
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		key, contents := parseLine(line)
		m[key] = contents
	}
	return m
}

// reverse reverses the mapping in containedBy, such that for each bag inner in
// containedBy[outer], it holds that outer appears in contains[inner].
func reverse(containedBy map[string]map[string]int) (contains map[string][]string) {
	m := make(map[string][]string)
	for outer, contents := range containedBy {
		for inner := range contents {
			m[inner] = append(m[inner], outer)
		}
	}
	return m
}

func countReachable(contains map[string][]string) int {
	visited := make(map[string]bool)
	var visit func(bag string)
	visit = func(bag string) {
		if visited[bag] {
			return
		}
		visited[bag] = true
		for _, outer := range contains[bag] {
			visit(outer)
		}
	}
	visit("shiny gold")
	return len(visited) - 1 // -1 since "shiny gold" itself shouldn't be counted
}

func sumReachable(containedBy map[string]map[string]int) int {
	sums := make(map[string]int)
	var visit func(bag string) int
	visit = func(bag string) int {
		sum, ok := sums[bag]
		if ok {
			return sum
		}
		sum = 1
		for inner, count := range containedBy[bag] {
			sum += count * visit(inner)
		}
		sums[bag] = sum
		return sum
	}
	return visit("shiny gold") - 1 // -1 since "shiny gold" itself shouldn't be counted
}

func solve(input string, part int) (string, error) {
	containedBy := parse(input)
	if part == 1 {
		return fmt.Sprint(countReachable(reverse(containedBy))), nil
	} else {
		return fmt.Sprint(sumReachable(containedBy)), nil
	}
}
