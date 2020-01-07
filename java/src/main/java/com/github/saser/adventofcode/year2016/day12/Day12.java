package com.github.saser.adventofcode.year2016.day12;

import java.io.Reader;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.year2016.assembunny.VM;

public final class Day12 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        int a = 1;
        int b = 1;
        int d = 26;
        if (part == 2) {
            d += 7;
        }
        var result = fibonacci(a, b, d) + 19 * 11;
        return Result.ok(Integer.toString(result));
    }

    private static int fibonacci(int a, int b, int d) {
        do {
            int c = a;
            a += b;
            b = c;
            d--;
        } while (d != 0);
        return a;
    }
}
