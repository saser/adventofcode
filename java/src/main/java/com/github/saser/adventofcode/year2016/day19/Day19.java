package com.github.saser.adventofcode.year2016.day19;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Arrays;
import java.util.Optional;

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
        var presents = Day19.play(n);
        var winner = Day19.next(presents, 0) + 1;
        return Result.ok(Integer.toString(winner));
    }

    private static int[] play(int n) {
        var presents = new int[n];
        Arrays.fill(presents, 1);
        var i = 0;
        while (true) {
            var next = Day19.next(presents, i);
            if (next == i) {
                break;
            }
            presents[i] += presents[next];
            presents[next] = 0;
            i = Day19.next(presents, i);
        }
        return presents;
    }

    private static int next(int[] presents, int start) {
        var n = presents.length;
        for (var i = (start + 1) % n; i != start; i = (i + 1) % n) {
            if (presents[i] > 0) {
                return i;
            }
        }
        return start;
    }
}
