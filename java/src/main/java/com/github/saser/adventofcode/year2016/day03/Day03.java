package com.github.saser.adventofcode.year2016.day03;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;

public final class Day03 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var br = new BufferedReader(r);
        var numbers = br.lines()
            .map((String line) -> {
                    var s = new Scanner(line);
                    var list = new ArrayList<Integer>();
                    while (s.hasNextInt()) {
                        list.add(s.nextInt());
                    }
                    return list;
                })
            .collect(Collectors.toList());
        var count = 0;
        var horizontal = part == 1;
        for (var i = 0; i < numbers.size() * 3; i += 3) {
            int a = at(numbers, i, horizontal);
            int b = at(numbers, i + 1, horizontal);
            int c = at(numbers, i + 2, horizontal);
            if (correct(a, b, c)) {
                count++;
            }
        }
        return Result.ok(Integer.toString(count));
    }

    private static int at(List<? extends List<Integer>> numbers, int i, boolean horizontal) {
        int row;
        int col;
        if (horizontal) {
            row = i / 3;
            col = i % 3;
        } else {
            var nrows = numbers.size();
            row = i % nrows;
            col = i / nrows;
        }
        return numbers.get(row).get(col);
    }

    private static boolean correct(int a, int b, int c) {
        return (a + b > c) && (a + c > b) && (b + c > a);
    }
}
