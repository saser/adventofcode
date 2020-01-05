package com.github.saser.adventofcode.year2016.day06;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Arrays;
import java.util.Map;
import java.util.stream.Collector;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import com.github.saser.adventofcode.Result;

public final class Day06 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var br = new BufferedReader(r);
        var message = transpose(br.lines())
                .map(Day06::count)
                .map(part == 1 ? Day06::maxByValue : Day06::minByValue)
                .collect(Collector.of( StringBuilder::new, StringBuilder::append, StringBuilder::append, StringBuilder::toString));
        return Result.ok(message);
    }

    private static Stream<String> transpose(Stream<String> lines) {
        var m = lines.map(String::toCharArray).collect(Collectors.toList()).toArray(new char[0][0]);
        var nRows = m.length;
        var nCols = m[0].length;
        var mt = new char[nCols][nRows];
        for (var row = 0; row < nRows; row++) {
            for (var col = 0; col < nCols; col++) {
                mt[col][row] = m[row][col];
            }
        }
        return Arrays.stream(mt).map(String::new);
    }

    private static Map<Character, Long> count(String s) {
        return s.chars()
                .mapToObj((c) -> (char) c)
                .collect(Collectors.groupingBy((c) -> c, Collectors.counting()));
    }

    private static <K, V extends Comparable<? super V>> K maxByValue(Map<K, V> map) {
        return map.entrySet()
                .stream()
                .max(Map.Entry.<K, V>comparingByValue())
                .get()
                .getKey();
    }

    private static <K, V extends Comparable<? super V>> K minByValue(Map<K, V> map) {
        return map.entrySet()
                .stream()
                .min(Map.Entry.<K, V>comparingByValue())
                .get()
                .getKey();
    }
}
