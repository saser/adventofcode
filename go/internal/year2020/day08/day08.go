package day08

import (
	"fmt"

	"github.com/Saser/adventofcode/internal/year2020/vm"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	prg, err := vm.ParseProgram(input)
	if err != nil {
		return "", fmt.Errorf("part %v: %w", part, err)
	}
	vm := vm.VM{Program: prg}
	seen := make([]bool, len(prg))
	seen[0] = true
	for {
		vm.Step()
		if seen[vm.PC] {
			break
		}
		seen[vm.PC] = true
	}
	return fmt.Sprint(vm.Acc), nil
}
