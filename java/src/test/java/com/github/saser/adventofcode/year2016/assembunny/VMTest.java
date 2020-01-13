package com.github.saser.adventofcode.year2016.assembunny;

import java.io.InputStreamReader;
import java.util.List;

import org.junit.Assert;
import org.junit.Test;

public class VMTest {
    @Test
    public void testCpyImmediate() {
        var vm = VM.from("cpy 1 a");
        vm.runAll();
        Assert.assertEquals(1, vm.a());
    }

    @Test
    public void testCpyRegister() {
        var program = new String[] {
                "cpy 1 a",
                "cpy a b",
                "cpy b c",
                "cpy c d",
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(1, vm.a());
        Assert.assertEquals(1, vm.b());
        Assert.assertEquals(1, vm.c());
        Assert.assertEquals(1, vm.d());
    }

    @Test
    public void testIncOnce() {
        var vm = VM.from("inc a");
        vm.runAll();
        Assert.assertEquals(1, vm.a());
    }

    @Test
    public void testIncTwice() {
        var program = new String[] {
                "inc a",
                "inc a",
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(2, vm.a());
    }

    @Test
    public void testDecOnce() {
        var vm = VM.from("dec a");
        vm.runAll();
        Assert.assertEquals(-1, vm.a());
    }

    @Test
    public void testDecTwice() {
        var program = new String[] {
                "dec a",
                "dec a",
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(-2, vm.a());
    }

    @Test
    public void testJnzZero() {
        var program = new String[] {
                "jnz 0 3",
                "cpy 1 a",
                "jnz a 2",
                "cpy 2 a",
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(1, vm.a());
    }

    @Test
    public void testJnzNotZero() {
        var program = new String[] {
                "jnz 1 3",
                "cpy 1 a",
                "jnz a 2",
                "cpy 2 a",
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(2, vm.a());
    }

    @Test
    public void testTglOneArgument() {
        var program = new String[] {
                "tgl 1",
                "inc a", // will become `dec a`, producing a = -1
                "tgl 1",
                "dec b", // will become `inc b`, producing b = 1
                "tgl 1",
                "tgl c", // will become `inc c`, producing c = 1
                "tgl 1",
                "out d", // will become `inc d`, producing d = 1
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(-1, vm.a());
        Assert.assertEquals(1, vm.b());
        Assert.assertEquals(1, vm.c());
        Assert.assertEquals(1, vm.d());
    }

    @Test
    public void testTglTwoArguments() {
        var program = new String[] {
                "tgl 1",
                "jnz 1 a", // will become `cpy 1 a`, producing a = 1
                "tgl 1",
                "cpy 1 2", // will become `jnz 1 2`...
                "cpy 1 b", // which will skip this instruction...
                "cpy 2 b", // producing b = 2
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(1, vm.a());
        Assert.assertEquals(2, vm.b());
    }

    @Test
    public void testTglSelf() {
        var program = new String[] {
                "tgl a", // a == 0, so this will become `inc a`
                "dec a", // the previous instruction should be skipped, producing a = -1
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(-1, vm.a());
    }

    @Test
    public void testTglIllegal() {
        var program = new String[] {
                "tgl 1",
                "jnz 1 2", // will become `cpy 1 2`, which is illegal and should be skipped...
                "cpy 1 a", // producing a = 1
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(1, vm.a());
    }

    @Test
    public void testOutImmediate() {
        var vm = VM.from("out 1");
        var outputs = vm.runAll();
        Assert.assertEquals(List.of(1), outputs);
    }

    @Test
    public void testOutRegister() {
        var program = new String[] {
                "cpy 1 a",
                "out a",
        };
        var vm = new VM(program);
        var outputs = vm.runAll();
        Assert.assertEquals(List.of(1), outputs);
    }

    @Test
    public void testOutMultiple() {
        var program = new String[] {
                "out 1",
                "out 2",
                "out 3",
        };
        var vm = new VM(program);
        var outputs = vm.runAll();
        Assert.assertEquals(List.of(1, 2, 3), outputs);
    }

    @Test
    public void testSetRegisters() {
        var vm = new VM(new String[0][0], 0, 0, 0, 0, 0);
        vm.a(1);
        Assert.assertEquals(1, vm.a());
        vm.b(2);
        Assert.assertEquals(2, vm.b());
        vm.c(3);
        Assert.assertEquals(3, vm.c());
        vm.d(4);
        Assert.assertEquals(4, vm.d());
    }

    @Test
    public void testReset() {
        var program = new String[] {
                "cpy 1 a",
                "cpy 2 b",
                "cpy 3 c",
                "cpy 4 d",
        };
        var vm = new VM(program);
        vm.runAll();
        Assert.assertEquals(1, vm.a());
        Assert.assertEquals(2, vm.b());
        Assert.assertEquals(3, vm.c());
        Assert.assertEquals(4, vm.d());
        Assert.assertEquals(4, vm.pc);
        vm.reset();
        Assert.assertEquals(0, vm.a());
        Assert.assertEquals(0, vm.b());
        Assert.assertEquals(0, vm.c());
        Assert.assertEquals(0, vm.d());
        Assert.assertEquals(0, vm.pc);
    }

    @Test
    public void testDay12Example() {
        var r = new InputStreamReader(this.getClass().getResourceAsStream("day12example"));
        var vm = VM.from(r);
        vm.runAll();
        Assert.assertEquals(42, vm.a());
    }

    @Test
    public void testDay23Example() {
        var r = new InputStreamReader(this.getClass().getResourceAsStream("day23example"));
        var vm = VM.from(r);
        vm.runAll();
        Assert.assertEquals(3, vm.a());
    }
}
