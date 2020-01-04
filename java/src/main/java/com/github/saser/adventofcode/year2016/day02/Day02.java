package com.github.saser.adventofcode.year2016.day02;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.geo.Point2D;

public final class Day02 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var point = new Point2D(0, 0);
        if (part == 2) {
            point.x = -2;
        }
        var br = new BufferedReader(r);
        var digits = br.lines().map((String line) -> {
                for (var direction : line.toCharArray()) {
                    step(point, direction, part);
                }
                return digit(point, part);
            }).collect(Collectors.toList());
        var code = "";
        for (var digit : digits) {
            code += digit;
        }
        return Result.ok(code);
    }

    private static void step(Point2D point, char direction, int part) {
        var delta = new Point2D(0, 0);
        switch (direction) {
        case 'U':
            delta.y = 1;
            break;
        case 'R':
            delta.x = 1;
            break;
        case 'D':
            delta.y = -1;
            break;
        case 'L':
            delta.x = -1;
            break;
        }
        var nextPoint = point.clone();
        nextPoint.add(delta);
        if (part == 1) {
            nextPoint.x = clamp(nextPoint.x, -1, 1);
            nextPoint.y = clamp(nextPoint.y, -1, 1);
        } else {
            if (nextPoint.manhattanDistance() > 2) {
                nextPoint = point;
            }
        }
        point.x = nextPoint.x;
        point.y = nextPoint.y;
    }

    private static int clamp(int x, int low, int high) {
        return Math.min(Math.max(x, low), high);
    }

    private static char digit(Point2D point, int part) {
        char[][] keypad;
        var index = point.clone();
        if (part == 1) {
            keypad = new char[][] {
                {'7', '8', '9'},
                {'4', '5', '6'},
                {'1', '2', '3'},
            };
            index.add(new Point2D(1, 1));
        } else {
            keypad = new char[][] {
                {'D'},
                {'A', 'B', 'C'},
                {'5', '6', '7', '8', '9'},
                {'2', '3', '4'},
                {'1'},
            };
            index.y += 2;
            index.x += 2 - Math.abs(point.y);
        }
        return keypad[index.y][index.x];
    }
}
