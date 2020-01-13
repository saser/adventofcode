package com.github.saser.adventofcode.year2016.day25;

import java.io.Reader;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.year2016.assembunny.VM;

public final class Day25 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    // My initial attempt consisted of trying to find a loop in the states of
    // the VM, based on the pattern `{0, 1}`. However, I expected the loop to be
    // exactly as long as the pattern, i.e., 2 states.
    //
    // After some initial failures, I just ran the program for a couple of
    // inputs and noted the values of the registers at different times, as well
    // as what was output. Turns out that the output is the binary
    // representation of a number, in reverse order, not limited to a certain
    // number of bits. In other words, we do not care about leading zeroes.
    //
    // The number, which I decided to call `bound`, is equal to what we choose
    // as `a`, plus a product of the values of `b` and `c` after an initial
    // setup. This setup consists of 3 instructions, which is why the first
    // three instructions are executed below.
    //
    // After we have found the bound, then we simply have to find the lowest
    // integer that is at least as large as bound, and whose binary
    // representation (without leading zeroes) is `10` repeating. To do that, we
    // simply start by setting `a` to zero, and as long as a is less than
    // `bound`, we shift `a` left two times, and add `10` as the new least
    // significant bits. When we have found `a`, the answer is `a - bound`.
    private static Result solve(Reader r, int part) {
        var vm = VM.from(r);
        for (var i = 0; i < 3; i++) {
            vm.run();
        }
        var bound = vm.b() * vm.c();
        var a = 0;
        while (a < bound) {
            a <<= 2;
            a |= 0b10;
        }
        return Result.ok(Integer.toString(a - bound));
    }
}
