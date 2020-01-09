package com.github.saser.adventofcode.year2016.day15;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Set;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.tuple.Tuple2;
import com.github.saser.adventofcode.tuple.Tuple3;

public final class Day15 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var discs = Day15.parse(r);
        var equations = discs.stream()
                .map(tuple -> {
                    var disc = tuple.v1;
                    var mod = tuple.v2;
                    var position = tuple.v3;
                    var a = Day15.posMod(-(position + disc), mod);
                    return new Tuple2<>(a, mod);
                })
                .collect(Collectors.toSet());
        var x = Day15.crt(equations);
        return Result.ok(Long.toString(x));
    }

    private static Set<Tuple3<Long, Long, Long>> parse(Reader r) {
        var re = Pattern.compile("Disc #(\\d+) has (\\d+) positions; at time=0, it is at position (\\d+).");
        return new BufferedReader(r)
                .lines()
                .map(line -> {
                    var matcher = re.matcher(line);
                    if (!matcher.matches()) {
                        throw new IllegalArgumentException(String.format("invalid line: %s", line));
                    }
                    var disc = Long.parseLong(matcher.group(1));
                    var mod = Long.parseLong(matcher.group(2));
                    var position = Long.parseLong(matcher.group(3));
                    return new Tuple3<>(disc, mod, position);
                })
                .collect(Collectors.toSet());
    }

    private static long crt(Set<Tuple2<Long, Long>> equations) {
        var it = equations.iterator();
        var acc = it.next();
        while (it.hasNext()) {
            var next = it.next();
            acc = Day15.crt(acc, next);
        }
        var x = acc.v1;
        var n = acc.v2;
        return Day15.posMod(x, n);
    }

    private static Tuple2<Long, Long> crt(Tuple2<Long, Long> an1, Tuple2<Long, Long> an2) {
        var a1 = an1.v1;
        var n1 = an1.v2;
        var a2 = an2.v1;
        var n2 = an2.v2;
        var m1m2 = Day15.eea(n1, n2);
        var m1 = m1m2.v2;
        var m2 = m1m2.v3;
        var x = a1 * m2 * n2 + a2 * m1 * n1;
        var n1n2 = n1 * n2;
        return new Tuple2<>(Day15.posMod(x, n1n2), n1n2);
    }

    private static Tuple3<Long, Long, Long> eea(long a, long b) {
        return eea(a, b, 1, 0, 0, 1);
    }

    private static Tuple3<Long, Long, Long> eea(long r0, long r1, long s0, long s1, long t0, long t1) {
        if (r1 == 0) {
            return new Tuple3<>(r0, s0, t0);
        }
        var q = r0 / r1;
        return eea(r1, r0 - q * r1, s1, s0 - q * s1, t1, t0 - q * t1);
    }

    private static long posMod(long a, long n) {
        var r = a % n;
        if (r < 0) {
            r += n;
        }
        return r;
    }
}
