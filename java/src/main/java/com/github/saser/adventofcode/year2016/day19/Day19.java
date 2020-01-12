package com.github.saser.adventofcode.year2016.day19;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

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
        List<Integer> elves = new ArrayList<>(n);
        for (var i = 1; i <= n; i++) {
            elves.add(i);
        }
        while (elves.size() > 1) {
            while (elves.size() % 3 != 0) {
                elves.remove(elves.size() / 2);
                elves.add(elves.remove(0));
            }
            elves = IntStream.rangeClosed(0, elves.size())
                    .filter(idx -> idx % 3 == 2)
                    .mapToObj(elves::get)
                    .collect(Collectors.toList());
        }
        return elves.get(0);
    }
}
