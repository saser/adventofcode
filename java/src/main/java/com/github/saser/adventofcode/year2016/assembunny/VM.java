package com.github.saser.adventofcode.year2016.assembunny;

import java.io.BufferedReader;
import java.io.Reader;
import java.io.StringReader;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

public class VM {
    public final String[] program;
    public int pc;
    public Map<String, Integer> registers;

    public VM(String[] program, int pc, int a, int b, int c, int d) {
        this.program = program;
        this.pc = pc;
        this.registers = new HashMap<>(Map.of("a", a, "b", b, "c", c, "d", d));
    }

    public VM(String[] program) {
        this(program, 0, 0, 0, 0, 0);
    }

    public VM() {
        this(new String[0]);
    }

    public static VM from(Reader r) {
        return new VM(new BufferedReader(r).lines().toArray(String[]::new));
    }

    public static VM from(String program) {
        return VM.from(new StringReader(program));
    }

    public int a() {
        return this.registerValueOf("a").get();
    }

    public int b() {
        return this.registerValueOf("b").get();
    }

    public int c() {
        return this.registerValueOf("c").get();
    }

    public int d() {
        return this.registerValueOf("d").get();
    }

    public void a(int i) {
        this.registers.put("a", i);
    }

    public void b(int i) {
        this.registers.put("b", i);
    }

    public void c(int i) {
        this.registers.put("c", i);
    }

    public void d(int i) {
        this.registers.put("d", i);
    }

    public void runAll() {
        while (this.pc < this.program.length) {
            this.run();
        }
    }

    public void run() {
        var instruction = this.program[this.pc];
        var parts = instruction.split(" ", 2);
        var op = parts[0];
        var params = parts[1].split(" ");
        var delta = 1;
        switch (op) {
            case "cpy":
                this.registers.put(params[1], valueOf(params[0]));
                break;
            case "inc":
            case "dec":
                this.registers.merge(params[0], op.equals("inc") ? 1 : -1, Integer::sum);
                break;
            case "jnz":
                if (valueOf(params[0]) != 0) {
                    delta = valueOf(params[1]);
                }
                break;
            default:
                throw new IllegalArgumentException(String.format("invalid op: %s", op));
        }
        this.pc += delta;
    }

    private int valueOf(String x) {
        var immediate = this.immediateValueOf(x);
        if (immediate.isPresent()) {
            return immediate.get();
        }
        var register = this.registerValueOf(x);
        if (register.isPresent()) {
            return register.get();
        }
        throw new IllegalArgumentException(String.format("invalid parameter: %s", x));
    }

    private Optional<Integer> immediateValueOf(String x) {
        try {
            var i = Integer.parseInt(x);
            return Optional.of(i);
        } catch (NumberFormatException e) {
            return Optional.empty();
        }
    }

    private Optional<Integer> registerValueOf(String x) {
        return Optional.ofNullable(this.registers.get(x));
    }
}
