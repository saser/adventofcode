package com.github.saser.adventofcode.{{.FullYear}}.{{.FullDay}};

import java.io.Reader;

import com.github.saser.adventofcode.Result;

public final class Day{{.PaddedDay}} {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        return Result.err("not implemented yet");
    }
}
