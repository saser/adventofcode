package com.github.saser.adventofcode.year2016.day01;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Comparator;
import java.util.TreeSet;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.geo.Point2D;

public final class Day01 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        try {
            var point = new Point2D(0, 0);
            var direction = new Point2D(0, 1);
            var comparator = Comparator
                .comparingInt((Point2D p) -> p.x)
                .thenComparingInt((Point2D p) -> p.y);
            var visited = new TreeSet<>(comparator);
            visited.add(point.clone());
            var br = new BufferedReader(r);
            var instructions = br.readLine();
            outerloop:
            for (var instruction : instructions.split(", ")) {
                switch (instruction.charAt(0)) {
                case 'L':
                    rotateLeft(direction);
                    break;
                case 'R':
                    rotateRight(direction);
                    break;
                }
                var steps = Integer.parseInt(instruction.substring(1));
                for (var i = 0; i < steps; i++) {
                    point.add(direction);
                    if (part == 2) {
                        if (visited.contains(point)) {
                            break outerloop;
                        }
                        visited.add(point.clone());
                    }
                }
            }
            var distance = point.manhattanDistance();
            return Result.ok(Integer.toString(distance));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static void rotateLeft(Point2D p) {
        var temp = p.x;
        p.x = p.y;
        p.y = temp;
        p.x *= -1;
    }

    private static void rotateRight(Point2D p) {
        var temp = p.x;
        p.x = p.y;
        p.y = temp;
        p.y *= -1;
    }
}
