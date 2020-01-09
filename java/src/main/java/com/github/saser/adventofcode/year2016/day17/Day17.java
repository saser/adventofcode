package com.github.saser.adventofcode.year2016.day17;

import java.io.BufferedReader;
import java.io.Reader;
import java.security.MessageDigest;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Set;

import com.github.saser.adventofcode.Result;
import com.github.saser.adventofcode.geo.Point2D;
import com.github.saser.adventofcode.tuple.Tuple2;

public final class Day17 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        try {
            var passcode = new BufferedReader(r).readLine();
            var maze = new Maze(passcode);
            var paths = maze.findVault(new Point2D(3, 3));
            String answer;
            if (part == 1) {
                answer = paths.v1;
            } else {
                answer = Integer.toString(paths.v2.length());
            }
            return Result.ok(answer);
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static class Maze {
        public final String passcode;
        private final MessageDigest md;

        public Maze(String passcode) throws Exception {
            this.passcode = passcode;
            this.md = MessageDigest.getInstance("MD5");
        }

        public Tuple2<String, String> findVault(Point2D target) {
            var queue = new LinkedList<Tuple2<Point2D, String>>();
            queue.add(new Tuple2<>(new Point2D(0, 0), ""));
            var shortestPath = "";
            var longestPath = "";
            while (!queue.isEmpty()) {
                var tuple = queue.remove();
                var point = tuple.v1;
                var path = tuple.v2;
                if (point.equals(target)) {
                    if (shortestPath.equals("")) {
                        shortestPath = path;
                    }
                    longestPath = path;
                    continue;
                }
                for (var nextStep : this.nextSteps(path)) {
                    var delta = new Point2D(0, 0);
                    switch (nextStep) {
                        case 'U':
                            delta.y = -1;
                            break;
                        case 'D':
                            delta.y = 1;
                            break;
                        case 'L':
                            delta.x = -1;
                            break;
                        case 'R':
                            delta.x = 1;
                            break;
                    }
                    var nextPoint = point.plus(delta);
                    if (nextPoint.x < 0 || nextPoint.x > 3 || nextPoint.y < 0 || nextPoint.y > 3) {
                        continue;
                    }
                    queue.add(new Tuple2<>(nextPoint, path + nextStep));
                }
            }
            return new Tuple2<>(shortestPath, longestPath);
        }

        private Set<Character> nextSteps(String path) {
            var toHash = this.passcode + path;
            var hash = Day17.hexString(md.digest((this.passcode + path).getBytes()));
            var doors = "UDLR".toCharArray();
            var openChars = "bcdef";
            var openDoors = new HashSet<Character>();
            for (var i = 0; i < 4; i++) {
                if (openChars.contains(hash.substring(i, i + 1))) {
                    openDoors.add(doors[i]);
                }
            }
            return openDoors;
        }
    }

    private static String hexString(byte[] bytes) {
        final var HEX_CHARS = "0123456789abcdef".toCharArray();
        var chars = new char[2 * bytes.length];
        for (var i = 0; i < bytes.length; i++) {
            var b = bytes[i] & 0xFF;
            chars[2 * i] = HEX_CHARS[b >>> 4]; // upper nibble
            chars[2 * i + 1] = HEX_CHARS[b & 0x0F]; // lower nibble;
        }
        return new String(chars);
    }
}
