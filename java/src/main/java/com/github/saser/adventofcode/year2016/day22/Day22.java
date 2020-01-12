package com.github.saser.adventofcode.year2016.day22;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.geo.Point2D;

public final class Day22 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var grid = Grid.parse(r);
        var count = grid.findViable().size();
        return Result.ok(Integer.toString(count));
    }

    private static class Node {
        public final int used;
        public final int available;

        public Node(int used, int available) {
            this.used = used;
            this.available = available;
        }

        public static boolean viable(Node a, Node b) {
            if (a.used == 0) {
                return false;
            }
            return a.used <= b.available;
        }
    }

    private static class Grid {
        public final Map<Point2D, Node> nodes;

        public Grid(Map<Point2D, Node> nodes) {
            this.nodes = nodes;
        }

        public static Grid parse(Reader r) {
            var re = Pattern.compile("/dev/grid/node-x(\\d+)-y(\\d+)\\s+(\\d+)T\\s+(\\d+)T\\s+(\\d+)T\\s+(\\d+)%");
            var nodes = new HashMap<Point2D, Node>();
            var maxX = 0;
            var maxY = 0;
            var it = new BufferedReader(r)
                    .lines()
                    .iterator();
            while (it.hasNext()) {
                var line = it.next();
                var matcher = re.matcher(line);
                if (!matcher.matches()) {
                    continue;
                }
                var x = Integer.parseInt(matcher.group(1));
                maxX = Math.max(maxX, x);
                var y = Integer.parseInt(matcher.group(2));
                maxY = Math.max(maxY, y);
                var point = new Point2D(x, y);
                var used = Integer.parseInt(matcher.group(4));
                var available = Integer.parseInt(matcher.group(5));
                nodes.put(point, new Node(used, available));
            }
            return new Grid(nodes);
        }

        private Point2D findEmpty() {
            return this.nodes
                    .entrySet()
                    .stream()
                    .filter(entry -> entry.getValue().used == 0)
                    .map(Map.Entry::getKey)
                    .findFirst()
                    .get();
        }

        public Set<Point2D> findViable() {
            var empty = this.findEmpty();
            var emptyNode = this.nodes.get(empty);
            return this.nodes
                    .entrySet()
                    .stream()
                    .filter(entry -> !entry.getKey().equals(empty))
                    .filter(entry -> Node.viable(entry.getValue(), emptyNode))
                    .map(Map.Entry::getKey)
                    .collect(Collectors.toSet());
        }
    }
}
