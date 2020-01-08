package com.github.saser.adventofcode.year2016.day13;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.HashSet;
import java.util.LinkedList;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.geo.Point2D;
import com.github.saser.adventofcode.tuple.Tuple2;

public final class Day13 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        try {
            var br = new BufferedReader(r);
            var input = br.readLine();
            var favorite = Integer.parseInt(input);
            var from = new Point2D(1, 1);
            var target = new Point2D(31, 39);
            var steps = Day13.steps(from, target, favorite);
            return Result.ok(Integer.toString(steps));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static int steps(Point2D from, Point2D target, int favorite) {
        var queue = new LinkedList<Tuple2<Point2D, Integer>>();
        queue.add(new Tuple2<>(from, 0));
        var visited = new HashSet<Point2D>();
        while (!queue.isEmpty()) {
            var tuple = queue.remove();
            var point = tuple.v1;
            var steps = tuple.v2;
            if (point.equals(target)) {
                return steps;
            }
            if (visited.contains(point)) {
                continue;
            }
            visited.add(point);
            var neighbors = new Point2D[] {
                    point.plus(new Point2D(1, 0)),
                    point.plus(new Point2D(-1, 0)),
                    point.plus(new Point2D(0, 1)),
                    point.plus(new Point2D(0, -1)),
            };
            for (var neighbor : neighbors) {
                if (neighbor.x < 0 || neighbor.y < 0) {
                    continue;
                }
                if (!Day13.isWall(neighbor, favorite)) {
                    queue.add(new Tuple2<>(neighbor, steps + 1));
                }
            }
        }
        return 0;
    }

    private static boolean isWall(Point2D point, int favorite) {
        var x = point.x;
        var y = point.y;
        return Integer.bitCount((x*x + 3*x + 2*x*y + y + y*y) + favorite) % 2 == 1;
    }
}
