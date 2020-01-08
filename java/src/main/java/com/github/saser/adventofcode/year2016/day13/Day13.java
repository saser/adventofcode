package com.github.saser.adventofcode.year2016.day13;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.Map;

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
            if (part == 1) {
                return Result.ok(Integer.toString(steps.get(target)));
            }
            var within50 = steps.values()
                    .stream()
                    .filter((length) -> length <= 50)
                    .count();
            return Result.ok(Long.toString(within50));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static Map<Point2D, Integer> steps(Point2D from, Point2D target, int favorite) {
        var queue = new LinkedList<Tuple2<Point2D, Integer>>();
        queue.add(new Tuple2<>(from, 0));
        var visited = new HashMap<Point2D, Integer>();
        while (!queue.isEmpty()) {
            var tuple = queue.remove();
            var point = tuple.v1;
            var steps = tuple.v2;
            if (visited.containsKey(point)) {
                continue;
            }
            visited.put(point, steps);
            if (point.equals(target)) {
                break;
            }
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
        return visited;
    }

    private static boolean isWall(Point2D point, int favorite) {
        var x = point.x;
        var y = point.y;
        return Integer.bitCount((x*x + 3*x + 2*x*y + y + y*y) + favorite) % 2 == 1;
    }
}
