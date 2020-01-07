package com.github.saser.adventofcode.year2016.assembunny;

import java.io.InputStreamReader;

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
    public void testSetRegisters() {
        var vm = new VM(new String[0], 0, 0, 0, 0, 0);
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
    public void testDay12Example() {
        var r = new InputStreamReader(this.getClass().getResourceAsStream("day12example"));
        var vm = VM.from(r);
        vm.runAll();
        Assert.assertEquals(42, vm.a());
    }
}
