package day21

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type stringSet map[string]bool

func newStringSet(elems ...string) stringSet {
	set := make(stringSet)
	for _, elem := range elems {
		set[elem] = true
	}
	return set
}

func (set stringSet) Clone() stringSet {
	set2 := make(stringSet, len(set))
	for elem := range set {
		set2[elem] = true
	}
	return set2
}

func (set stringSet) Any() string {
	for elem := range set {
		return elem
	}
	return ""
}

func (set stringSet) Add(elems ...string) {
	for _, elem := range elems {
		set[elem] = true
	}
}

func (set stringSet) AddAll(other stringSet) {
	for elem := range other {
		set[elem] = true
	}
}

func (set stringSet) Delete(elems ...string) {
	for _, elem := range elems {
		delete(set, elem)
	}
}

func (set stringSet) Retain(other stringSet) {
	for elem := range set {
		if !other[elem] {
			delete(set, elem)
		}
	}
}

func parseLine(line string) (ingredients stringSet, allergens stringSet) {
	parts := strings.Split(line, " (contains ")
	ingredientStrs := strings.Split(parts[0], " ")
	allergenStrs := strings.Split(strings.TrimSuffix(parts[1], ")"), ", ")
	return newStringSet(ingredientStrs...), newStringSet(allergenStrs...)
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	occurrences := make(map[string]int)                // ingredient -> number of lines it occurs on
	candidateIngredients := make(map[string]stringSet) // allergen -> possible corresponding ingredients
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		ingredients, allergens := parseLine(line)
		for ingr := range ingredients {
			occurrences[ingr]++
		}
		for all := range allergens {
			if _, ok := candidateIngredients[all]; !ok {
				candidateIngredients[all] = ingredients.Clone()
				continue
			}
			candidateIngredients[all].Retain(ingredients)
		}
	}
	translation := make(map[string]string) // ingredient -> allergen
	for len(candidateIngredients) > 0 {
		var all, ingr string
		for a, is := range candidateIngredients {
			if len(is) == 1 {
				all = a
				ingr = is.Any()
				break
			}
		}
		delete(candidateIngredients, all)
		translation[ingr] = all
		for _, is := range candidateIngredients {
			is.Delete(ingr)
		}
	}
	count := 0
	for ingr, n := range occurrences {
		if _, ok := translation[ingr]; !ok {
			count += n
		}
	}
	return fmt.Sprint(count), nil
}
