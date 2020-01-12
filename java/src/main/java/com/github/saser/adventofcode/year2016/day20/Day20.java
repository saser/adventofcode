package com.github.saser.adventofcode.year2016.day20;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.ArrayList;
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
        var available = toAvailable(intervals);
        if (part == 1) {
            var first = available.get(0);
            return Result.ok(Long.toString(first.v1));
        }
        var sum = available.stream()
                .mapToLong(interval -> interval.v2 - interval.v1)
                .sum();
        return Result.ok(Long.toString(sum));
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

    private static List<Tuple2<Long, Long>> toAvailable(List<Tuple2<Long, Long>> blocked) {
        var sorted = blocked.stream()
                .flatMap(interval -> Stream.of(new Tuple2<>(interval.v1, false), new Tuple2<>(interval.v2 + 1, true)))
                .sorted(Comparator.<Tuple2<Long, Boolean>, Long>comparing(edge -> edge.v1)
                        .thenComparing(edge -> edge.v2));
        var it = sorted.iterator();
        var available = new ArrayList<Tuple2<Long, Long>>();
        long start = 0;
        var depth = 0;
        while (it.hasNext()) {
            var edge = it.next();
            var value = edge.v1;
            var isEnd = edge.v2;
            if (isEnd) {
                depth--;
                if (depth == 0) {
                    start = value;
                }
            } else {
                depth++;
                if (depth == 1) {
                    available.add(new Tuple2<>(start, value));
                }
            }
        }
        available.add(new Tuple2<>(start, (1L << 32) - 1));
        return available.stream()
                .filter(interval -> interval.v2 > interval.v1)
                .collect(Collectors.toUnmodifiableList());
    }
}
