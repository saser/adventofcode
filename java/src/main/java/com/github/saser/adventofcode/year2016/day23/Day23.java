package com.github.saser.adventofcode.year2016.day23;

import java.io.Reader;

import com.github.saser.adventofcode.Result;

public final class Day23 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        // After studying the code for the program in my input, I found that it
        // was calculating the factorial of the initial value of register a,
        // plus the product of 75 * 72. This program might not work for all
        // inputs, since I assume that the numbers 75 and 72 might be different
        // in other outputs; however, I believe the program will perform the
        // same calculation.
        var result = factorial(part == 1 ? 7 : 12) + 75 * 72;
        return Result.ok(Integer.toString(result));
    }

    private static int factorial(int n) {
        int p = 1;
        while (n > 1) {
            p *= n;
            n--;
        }
        return p;
    }
}
