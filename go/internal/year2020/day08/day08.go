package day08

import (
	"errors"
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
	prg, err := vm.ParseProgram(input)
	if err != nil {
		return "", fmt.Errorf("part %v: %w", part, err)
	}
	if part == 1 {
		v := vm.VM{Program: prg}
		run(&v)
		return fmt.Sprint(v.Acc), nil
	}
	for i, instr := range prg {
		if instr.Op == vm.Acc {
			continue
		}
		prg2 := prg.Copy()
		switch instr.Op {
		case vm.Jmp:
			prg2[i].Op = vm.Nop
		case vm.Nop:
			prg2[i].Op = vm.Jmp
		}
		v := vm.VM{Program: prg2}
		if run(&v) {
			return fmt.Sprint(v.Acc), nil
		}
	}
	return "", errors.New("part 2: no solution found")
}

// run executes vm.Step() repeatedly. It returns true if the program
// terminated, or false if the program loops infinitely.
func run(vm *vm.VM) bool {
	seen := make([]bool, len(vm.Program))
	seen[0] = true
	for {
		vm.Step()
		if vm.PC >= len(vm.Program) || seen[vm.PC] {
			break
		}
		seen[vm.PC] = true
	}
	return vm.PC >= len(vm.Program)
}
