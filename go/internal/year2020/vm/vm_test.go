package vm

import (
	"reflect"
	"testing"
)

func TestOp_String(t *testing.T) {
	for _, tt := range []struct {
		op   Op
		want string
	}{
		{op: Nop, want: "nop"},
		{op: Acc, want: "acc"},
		{op: Jmp, want: "jmp"},
	} {
		if got := tt.op.String(); got != tt.want {
			t.Errorf("op.String() = %q; want %q", got, tt.want)
		}
	}
}

func TestArg_String(t *testing.T) {
	for _, tt := range []struct {
		arg  Arg
		want string
	}{
		{arg: 0, want: "+0"},
		{arg: 1, want: "+1"},
		{arg: 99, want: "+99"},
		{arg: -1, want: "-1"},
		{arg: -99, want: "-99"},
	} {
		if got := tt.arg.String(); got != tt.want {
			t.Errorf("arg.String() = %q; want %q", got, tt.want)
		}
	}
}

func TestParseInstruction(t *testing.T) {
	for _, tt := range []struct {
		s    string
		want Instruction
	}{
		{s: "nop +0", want: Instruction{Op: Nop, Arg: 0}},
		{s: "acc +1", want: Instruction{Op: Acc, Arg: 1}},
		{s: "jmp +4", want: Instruction{Op: Jmp, Arg: 4}},
		{s: "acc +3", want: Instruction{Op: Acc, Arg: 3}},
		{s: "jmp -3", want: Instruction{Op: Jmp, Arg: -3}},
		{s: "acc -99", want: Instruction{Op: Acc, Arg: -99}},
		{s: "acc +1", want: Instruction{Op: Acc, Arg: 1}},
		{s: "jmp -4", want: Instruction{Op: Jmp, Arg: -4}},
		{s: "acc +6", want: Instruction{Op: Acc, Arg: 6}},
	} {
		got, err := ParseInstruction(tt.s)
		if err != nil {
			t.Errorf("ParseInstruction(%q) err = %v", tt.s, err)
		}
		if got != tt.want {
			t.Errorf("ParseInstruction(%q) instr = %#v; want %#v", tt.s, got, tt.want)
		}
	}
}

func TestInstruction_String(t *testing.T) {
	for _, tt := range []struct {
		i    Instruction
		want string
	}{
		{i: Instruction{Op: Nop, Arg: 0}, want: "nop +0"},
		{i: Instruction{Op: Acc, Arg: 1}, want: "acc +1"},
		{i: Instruction{Op: Jmp, Arg: 4}, want: "jmp +4"},
		{i: Instruction{Op: Acc, Arg: 3}, want: "acc +3"},
		{i: Instruction{Op: Jmp, Arg: -3}, want: "jmp -3"},
		{i: Instruction{Op: Acc, Arg: -99}, want: "acc -99"},
		{i: Instruction{Op: Acc, Arg: 1}, want: "acc +1"},
		{i: Instruction{Op: Jmp, Arg: -4}, want: "jmp -4"},
		{i: Instruction{Op: Acc, Arg: 6}, want: "acc +6"},
	} {
		if got := tt.i.String(); got != tt.want {
			t.Errorf("i.String() = %q; want %q", got, tt.want)
		}
	}
}

func TestParseProgram(t *testing.T) {
	s := `nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6`
	want := Program{
		{Op: Nop, Arg: 0},
		{Op: Acc, Arg: 1},
		{Op: Jmp, Arg: 4},
		{Op: Acc, Arg: 3},
		{Op: Jmp, Arg: -3},
		{Op: Acc, Arg: -99},
		{Op: Acc, Arg: 1},
		{Op: Jmp, Arg: -4},
		{Op: Acc, Arg: 6},
	}
	got, err := ParseProgram(s)
	if err != nil {
		t.Errorf("ParseProgram() err = %v", err)
	}
	if !reflect.DeepEqual(got, want) {
		t.Errorf("ParseProgram() prog = %v; want %v", got, want)
	}
}

func TestVM_Step(t *testing.T) {
	for _, tt := range []struct {
		instr   Instruction
		wantPC  int
		wantAcc int
	}{
		{
			instr:   Instruction{Op: Nop, Arg: 0},
			wantPC:  1,
			wantAcc: 0,
		},
		{
			instr:   Instruction{Op: Acc, Arg: 10},
			wantPC:  1,
			wantAcc: 10,
		},
		{
			instr:   Instruction{Op: Acc, Arg: -10},
			wantPC:  1,
			wantAcc: -10,
		},
		{
			instr:   Instruction{Op: Jmp, Arg: 1},
			wantPC:  1,
			wantAcc: 0,
		},
		{
			instr:   Instruction{Op: Jmp, Arg: 2},
			wantPC:  2,
			wantAcc: 0,
		},
		{
			instr:   Instruction{Op: Jmp, Arg: 0},
			wantPC:  0,
			wantAcc: 0,
		},
		{
			instr:   Instruction{Op: Jmp, Arg: -1},
			wantPC:  -1,
			wantAcc: 0,
		},
	} {
		vm := VM{
			Program: Program{tt.instr},
			PC:      0,
			Acc:     0,
		}
		vm.Step()
		if vm.PC != tt.wantPC {
			t.Errorf("vm.PC = %v; want %v", vm.PC, tt.wantPC)
		}
		if vm.Acc != tt.wantAcc {
			t.Errorf("vm.Acc = %v; want %v", vm.Acc, tt.wantAcc)
		}
	}
}
