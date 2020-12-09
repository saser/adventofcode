package vm

import (
	"fmt"
	"strconv"
	"strings"
)

type Op int

const (
	Nop Op = iota
	Acc
	Jmp
)

func (op Op) String() string {
	switch op {
	case Nop:
		return "nop"
	case Acc:
		return "acc"
	case Jmp:
		return "jmp"
	}
	panic(fmt.Sprintf("invalid op: %d", op))
}

type Arg int

func (arg Arg) String() string {
	return fmt.Sprintf("%+d", arg)
}

type Instruction struct {
	Op  Op
	Arg Arg
}

func ParseInstruction(s string) (Instruction, error) {
	sep := " "
	split := strings.Index(s, sep)
	opStr := s[:split]
	var op Op
	switch opStr {
	case "nop":
		op = Nop
	case "acc":
		op = Acc
	case "jmp":
		op = Jmp
	default:
		return Instruction{}, fmt.Errorf("invalid instruction: %q", s)
	}
	argStr := s[split+len(sep):]
	arg, err := strconv.Atoi(argStr)
	if err != nil {
		return Instruction{}, fmt.Errorf("invalid instruction: %q: %w", s, err)
	}
	return Instruction{
		Op:  op,
		Arg: Arg(arg),
	}, nil
}

func (i Instruction) String() string {
	return fmt.Sprintf("%v %v", i.Op, i.Arg)
}

type Program []Instruction

func ParseProgram(s string) (Program, error) {
	lines := strings.Split(strings.TrimSpace(s), "\n")
	instrs := make([]Instruction, len(lines))
	for i, line := range lines {
		instr, err := ParseInstruction(line)
		if err != nil {
			return nil, fmt.Errorf("parse program: %w", err)
		}
		instrs[i] = instr
	}
	return instrs, nil
}

func (p Program) Copy() Program {
	p2 := make(Program, len(p))
	copy(p2, p)
	return p2
}

type VM struct {
	Program Program
	Acc     int
	PC      int
}

func (vm *VM) Step() {
	switch instr := vm.Program[vm.PC]; instr.Op {
	case Nop:
		// do nothing
	case Acc:
		vm.Acc += int(instr.Arg)
	case Jmp:
		vm.PC += int(instr.Arg)
		return // pc should not be incremented
	}
	vm.PC++
}
