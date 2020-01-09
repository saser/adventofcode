package com.github.saser.adventofcode.year2016.day16;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Arrays;

import com.github.saser.adventofcode.Result;

public final class Day16 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        try {
            var state = Day16.parse(r);
            state = Day16.generate(state, part == 1 ? 272 : 35651584);
            var checksum = Day16.checksum(state);
            return Result.ok(Day16.toString(checksum));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static boolean[] parse(Reader r) throws Exception {
        var line = new BufferedReader(r).readLine();
        var state = new boolean[line.length()];
        for (var i = 0; i < line.length(); i++) {
            state[i] = line.charAt(i) == '1';
        }
        return state;
    }

    private static String toString(boolean[] state) {
        var sb = new StringBuilder(state.length);
        for (var bit : state) {
            sb.append(bit ? '1' : '0');
        }
        return sb.toString();
    }

    private static boolean[] generate(boolean[] state) {
        var n = 1 + state.length * 2;
        var newState = new boolean[n];
        for (var i = 0; i < state.length; i++) {
            newState[i] = state[i];
            newState[n - (i + 1)] = !state[i];
        }
        return newState;
    }

    private static boolean[] generate(boolean[] state, int size) {
        while (state.length < size) {
            state = generate(state);
        }
        return Arrays.copyOf(state, size);
    }

    private static boolean[] checksum(boolean[] state) {
        if (state.length % 2 == 1) {
            return state;
        }
        var newState = new boolean[state.length / 2];
        for (var i = 0; i < newState.length; i++) {
            newState[i] = state[2 * i] == state[2 * i + 1];
        }
        return checksum(newState);
    }
}
