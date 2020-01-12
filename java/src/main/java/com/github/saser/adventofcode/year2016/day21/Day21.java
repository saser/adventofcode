package com.github.saser.adventofcode.year2016.day21;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Arrays;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;

public final class Day21 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var chars = "abcdefgh".toCharArray();
        var instructions = new BufferedReader(r)
                .lines()
                .collect(Collectors.toUnmodifiableList());
        apply(chars, instructions);
        return Result.ok(new String(chars));
    }

    private static void apply(char[] chars, List<String> instructions) {
        var swapPositionRE = Pattern.compile("swap position (\\d+) with position (\\d+)");
        var swapLetterRE = Pattern.compile("swap letter (\\w) with letter (\\w)");
        var rotateStepsRE = Pattern.compile("rotate (left|right) (\\d+) steps?");
        var rotateLetterRE = Pattern.compile("rotate based on position of letter (\\w)");
        var reverseRE = Pattern.compile("reverse positions (\\d+) through (\\d+)");
        var moveRE = Pattern.compile("move position (\\d+) to position (\\d+)");
        for (var instruction : instructions) {
            Matcher matcher;
            matcher = swapPositionRE.matcher(instruction);
            if (matcher.matches()) {
                var x = Integer.parseInt(matcher.group(1));
                var y = Integer.parseInt(matcher.group(2));
                swap(chars, x, y);
                continue;
            }
            matcher = swapLetterRE.matcher(instruction);
            if (matcher.matches()) {
                var x = find(chars, matcher.group(1).charAt(0));
                var y = find(chars, matcher.group(2).charAt(0));
                swap(chars, x, y);
                continue;
            }
            matcher = rotateStepsRE.matcher(instruction);
            if (matcher.matches()) {
                var right = matcher.group(1).equals("right");
                var steps = Integer.parseInt(matcher.group(2));
                rotate(chars, right, steps);
                continue;
            }
            matcher = rotateLetterRE.matcher(instruction);
            if (matcher.matches()) {
                var index = find(chars, matcher.group(1).charAt(0));
                var steps = 1 + index;
                if (index >= 4) {
                    steps++;
                }
                rotate(chars, true, steps);
                continue;
            }
            matcher = reverseRE.matcher(instruction);
            if (matcher.matches()) {
                var x = Integer.parseInt(matcher.group(1));
                var y = Integer.parseInt(matcher.group(2));
                reverse(chars, x, y);
                continue;
            }
            matcher = moveRE.matcher(instruction);
            if (matcher.matches()) {
                var x = Integer.parseInt(matcher.group(1));
                var y = Integer.parseInt(matcher.group(2));
                move(chars, x, y);
                continue;
            }
            throw new IllegalArgumentException(String.format("invalid instruction: %s", instruction));
        }
    }

    private static void swap(char[] chars, int x, int y) {
        var temp = chars[x];
        chars[x] = chars[y];
        chars[y] = temp;
    }

    private static void rotate(char[] chars, boolean right, int steps) {
        var copy = chars.clone();
        var n = chars.length;
        var delta = right ? steps : n - steps;
        for (var i = 0; i < n; i++) {
            chars[(i + delta) % n] = copy[i];
        }
    }

    private static void reverse(char[] chars, int x, int y) {
        for (int i = x, j = y; i < j; i++, j--) {
            swap(chars, i, j);
        }
    }

    private static void move(char[] chars, int from, int to) {
        var copy = chars.clone();
        var c = chars[from];
        int low, high, d;
        if (from < to) {
            low = from;
            high = to;
            d = 1;
        } else {
            low = to + 1;
            high = from + 1;
            d = -1;
        }
        for (var i = 0; i < chars.length; i++) {
            if (i == to) {
                chars[i] = c;
                continue;
            }
            var ci = i;
            if (i >= low && i < high) {
                ci += d;
            }
            chars[i] = copy[ci];
        }
    }

    private static int find(char[] chars, char c) {
        for (var i = 0; i < chars.length; i++) {
            if (chars[i] == c) {
                return i;
            }
        }
        return -1;
    }
}
