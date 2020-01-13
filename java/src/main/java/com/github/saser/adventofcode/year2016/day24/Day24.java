package com.github.saser.adventofcode.year2016.day24;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Comparator;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Set;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.geo.Point2D;
import com.github.saser.adventofcode.tuple.Tuple2;
import com.github.saser.adventofcode.tuple.Tuple3;

public final class Day24 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var grid = parse(r);
        var keys = findKeys(grid);
        var start = findStart(grid, keys);
        var distances = findDistances(grid, keys);
        var steps = collectKeys(start, keys, distances);
        return Result.ok(Integer.toString(steps));
    }

    private static char[][] parse(Reader r) {
        return new BufferedReader(r)
                .lines()
                .map(line -> {
                    var row = new char[line.length()];
                    for (var i = 0; i < line.length(); i++) {
                        row[i] = line.charAt(i);
                    }
                    return row;
                }).toArray(char[][]::new);
    }

    private static Point2D findStart(char[][] grid, Set<Point2D> keys) {
        for (var key : keys) {
            if (grid[key.x][key.y] == '0') {
                return key;
            }
        }
        throw new IllegalArgumentException("no start found");
    }

    private static Set<Point2D> findKeys(char[][] grid) {
        var keys = new HashSet<Point2D>();
        for (var x = 0; x < grid.length; x++) {
            for (var y = 0; y < grid[x].length; y++) {
                var c = grid[x][y];
                if (c == '#' || c == '.') {
                    continue;
                }
                keys.add(new Point2D(x, y));
            }
        }
        return keys;
    }

    private static Map<Point2D, Map<Point2D, Integer>> findDistances(char[][] grid, Set<Point2D> keys) {
        var distances = new HashMap<Point2D, Map<Point2D, Integer>>();
        for (var key : keys) {
            var queue = new LinkedList<Tuple2<Point2D, Integer>>();
            queue.add(new Tuple2<>(key, 0));
            var visited = new HashSet<Point2D>();
            while (!queue.isEmpty()) {
                var tuple = queue.remove();
                var point = tuple.v1;
                var steps = tuple.v2;
                if (visited.contains(point)) {
                    continue;
                }
                visited.add(point);
                if (keys.contains(point)) {
                    if (!distances.containsKey(key)) {
                        distances.put(key, new HashMap<>());
                    }
                    distances.get(key).put(point, steps);
                    if (!distances.containsKey(point)) {
                        distances.put(point, new HashMap<>());
                    }
                    distances.get(point).put(key, steps);
                    if (distances.get(key).keySet().containsAll(keys)) {
                        break;
                    }
                }
                var neighbors = new Point2D[] {
                        point.plus(new Point2D(1, 0)),
                        point.plus(new Point2D(-1, 0)),
                        point.plus(new Point2D(0, 1)),
                        point.plus(new Point2D(0, -1)),
                };
                for (var neighbor : neighbors) {
                    if (grid[neighbor.x][neighbor.y] == '#') {
                        continue;
                    }
                    queue.add(new Tuple2<>(neighbor, steps + 1));
                }
            }
        }
        return distances;
    }

    private static int collectKeys(Point2D start, Set<Point2D> keys, Map<Point2D, Map<Point2D, Integer>> distances) {
        var queue = new PriorityQueue<Tuple3<Point2D, Set<Point2D>, Integer>>(Comparator.comparing(tuple -> tuple.v3));
        queue.add(new Tuple3<>(start, Set.of(start), 0));
        var visited = new HashSet<Tuple2<Point2D, Set<Point2D>>>();
        while (!queue.isEmpty()) {
            var tuple = queue.remove();
            var point = tuple.v1;
            var collected = tuple.v2;
            var steps = tuple.v3;
            var state = new Tuple2<>(point, collected);
            if (visited.contains(state)) {
                continue;
            }
            visited.add(state);
            if (collected.containsAll(keys)) {
                return steps;
            }
            for (var entry : distances.get(point).entrySet()) {
                var nextKey = entry.getKey();
                if (collected.contains(nextKey)) {
                    continue;
                }
                var distance = entry.getValue();
                var nextCollected = new HashSet<>(collected);
                nextCollected.add(nextKey);
                var nextSteps = steps + distance;
                queue.add(new Tuple3<>(nextKey, nextCollected, nextSteps));
            }
        }
        return -1;
    }
}
