package com.github.saser.adventofcode.year2016.day18;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.stream.Stream;

import com.github.saser.adventofcode.Result;

public final class Day18 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var state = Day18.parse(r);
        var states = Stream.iterate(state, Day18::next);
        var sum = states
                .limit(40)
                .mapToInt(Day18::count)
                .sum();
        return Result.ok(Integer.toString(sum));
    }

    private static boolean[] parse(Reader r) {
        var line = new BufferedReader(r)
            .lines()
            .findFirst()
            .get()
            .toCharArray();
        var state = new boolean[line.length];
        for (var i = 0; i < state.length; i++) {
            state[i] = line[i] == '^';
        }
        return state;
    }

    private static int count(boolean[] state) {
        var count = 0;
        for (var trap : state) {
            if (!trap) {
                count++;
            }
        }
        return count;
    }

    private static boolean[] next(boolean[] state) {
        var next = new boolean[state.length];
        for (var i = 0; i < state.length; i++) {
            var left = i != 0 && state[i - 1];
            var right = i != state.length - 1 && state[i + 1];
            next[i] = !right == left;
        }
        return next;
    }
}
