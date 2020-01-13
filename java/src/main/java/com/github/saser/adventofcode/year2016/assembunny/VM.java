package com.github.saser.adventofcode.year2016.assembunny;

import java.io.BufferedReader;
import java.io.Reader;
import java.io.StringReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

public class VM {
    public final String[][] program;
    public int pc;
    public int[] registers;

    public VM(String[][] program, int pc, int a, int b, int c, int d) {
        this.program = program;
        this.pc = pc;
        this.registers = new int[] {a, b, c, d};
    }

    public VM(String[] program) {
        this(splitProgram(program), 0, 0, 0, 0, 0);
    }

    public static VM from(Reader r) {
        return new VM(new BufferedReader(r).lines().toArray(String[]::new));
    }

    public static VM from(String program) {
        return VM.from(new StringReader(program));
    }

    private static String[][] splitProgram(String[] program) {
        return Arrays.stream(program)
                .map(instruction -> instruction.split(" "))
                .toArray(String[][]::new);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        VM vm = (VM) o;
        return pc == vm.pc &&
                Arrays.equals(program, vm.program) &&
                Arrays.equals(registers, vm.registers);
    }

    @Override
    public int hashCode() {
        int result = Objects.hash(pc);
        result = 31 * result + Arrays.hashCode(program);
        result = 31 * result + Arrays.hashCode(registers);
        return result;
    }

    @Override
    public VM clone() {
        var program = new String[this.program.length][];
        for (var i = 0; i < program.length; i++) {
            program[i] = this.program[i].clone();
        }
        return new VM(program, this.pc, this.a(), this.b(), this.c(), this.d());
    }

    public int a() {
        return this.register("a");
    }

    public int b() {
        return this.register("b");
    }

    public int c() {
        return this.register("c");
    }

    public int d() {
        return this.register("d");
    }

    public void a(int i) {
        this.register("a", i);
    }

    public void b(int i) {
        this.register("b", i);
    }

    public void c(int i) {
        this.register("c", i);
    }

    public void d(int i) {
        this.register("d", i);
    }

    public List<Integer> runAll() {
        var output = new ArrayList<Integer>();
        while (this.pc < this.program.length) {
            this.run().ifPresent(output::add);
        }
        return output;
    }

    public Optional<Integer> run() {
        var instruction = this.program[this.pc];
        var op = instruction[0];
        var param1 = instruction[1];
        var value1 = this.valueOf(param1);
        var param2 = Optional.<String>empty();
        var value2 = Optional.<Integer>empty();
        if (instruction.length - 1 == 2) {
            param2 = Optional.of(instruction[2]);
            value2 = Optional.of(this.valueOf(param2.get()));
        }
        var delta = 1;
        var out = Optional.<Integer>empty();
        switch (op) {
            case "cpy":
                try {
                    this.register(param2.get(), value1);
                } catch (IllegalArgumentException e) {
                    // do nothing, this was most likely caused by a `tgl` instruction
                }
                break;
            case "inc":
            case "dec":
                try {
                    this.register(param1, value1 + (op.equals("inc") ? 1 : -1));
                } catch (IllegalArgumentException e) {
                    // do nothing, this was most likely caused by a `tgl` instruction
                }
                break;
            case "jnz":
                if (value1 != 0) {
                    delta = value2.get();
                }
                break;
            case "tgl": {
                var tglIdx = this.pc + value1;
                if (tglIdx >= this.program.length) {
                    break;
                }
                var tglOp = this.program[tglIdx][0];
                switch (this.program[tglIdx].length - 1) {
                    case 1: {
                        this.program[tglIdx][0] = tglOp.equals("inc") ? "dec" : "inc";
                        break;
                    }
                    case 2: {
                        this.program[tglIdx][0] = tglOp.equals("jnz") ? "cpy" : "jnz";
                        break;
                    }
                }
                break;
            }
            case "out":
                out = Optional.of(value1);
                break;
            default:
                throw new IllegalArgumentException(String.format("invalid op: %s", op));
        }
        this.pc += delta;
        return out;
    }

    private int registerOffset(String x) {
        var offset = x.charAt(0) - 'a';
        if (offset < 0 || offset > 3) {
            throw new IllegalArgumentException(String.format("invalid register: %s", x));
        }
        return offset;
    }

    private int register(String x) {
        return this.registers[this.registerOffset(x)];
    }

    private void register(String x, int i) {
        this.registers[this.registerOffset(x)] = i;
    }

    private int valueOf(String x) {
        try {
            return Integer.parseInt(x);
        } catch (NumberFormatException e) {
            return this.register(x);
        }
    }
}
