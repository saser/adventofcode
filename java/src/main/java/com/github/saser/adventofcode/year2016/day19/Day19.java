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
        int winner;
        if (part == 1) {
            winner = play1(n);
        } else {
            winner = play2(n);
        }
        return Result.ok(Integer.toString(winner));
    }

    private static int play1(int n) {
        int k = Integer.highestOneBit(n);
        return ((n - k) << 1) + 1;
    }

    private static int play2(int n) {
        var p = (int) (Math.pow(3, (int) (Math.log(n) / Math.log(3))));
        if (p == n) {
            return n;
        }
        return n - p + Math.max(n - 2 * p, 0);
    }
}
