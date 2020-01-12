package com.github.saser.adventofcode.year2016.day20;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.tuple.Tuple2;

public final class Day20 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var intervals = parse(r);
        var answer = firstAvailable(intervals);
        return Result.ok(Long.toString(answer));
    }

    private static List<Tuple2<Long, Long>> parse(Reader r) {
        return new BufferedReader(r)
                .lines()
                .map(line -> {
                    var parts = line.split("-");
                    var start = Long.parseLong(parts[0]);
                    var end = Long.parseLong(parts[1]);
                    return new Tuple2<>(Math.min(start, end), Math.max(start, end));
                })
                .collect(Collectors.toUnmodifiableList());
    }

    private static long firstAvailable(List<Tuple2<Long, Long>> intervals) {
        var sorted = intervals.stream()
                .flatMap(tuple -> {
                    var start = tuple.v1;
                    var end = tuple.v2;
                    return Stream.of(new Tuple2<>(start, false), new Tuple2<>(end + 1, true));
                })
                .sorted(Comparator.<Tuple2<Long, Boolean>, Long>comparing(tuple -> tuple.v1)
                        .thenComparing(tuple -> tuple.v2));
        var it = sorted.iterator();
        if (it.next().v1 > 0) {
            return 0;
        }
        var depth = 1;
        while (it.hasNext()) {
            var element = it.next();
            if (element.v2) {
                depth--;
                if (depth == 0) {
                    return element.v1;
                }
            } else {
                depth++;
            }
        }
        return -1;
    }
}
