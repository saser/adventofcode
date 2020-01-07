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
        var vm = VM.from(r);
        vm.c(part == 2 ? 1 : 0);
        vm.runAll();
        return Result.ok(Integer.toString(vm.a()));
    }
}
