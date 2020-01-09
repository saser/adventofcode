package com.github.saser.adventofcode.year2016.day19;

import java.io.BufferedReader;
import java.io.Reader;

import com.github.saser.adventofcode.Result;

public final class Day19 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var input = new BufferedReader(r)
                .lines()
                .findFirst()
                .get();
        var n = Integer.parseInt(input);
        var winner = play(n);
        return Result.ok(Integer.toString(winner));
    }

    private static int play(int n) {
        var start = 1;
        var delta = 2;
        while (n > 1) {
            start += (n % 2) * delta;
            delta *= 2;
            n /= 2;
        }
        return start;
    }
}
