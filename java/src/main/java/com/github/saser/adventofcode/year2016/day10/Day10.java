package com.github.saser.adventofcode.year2016.day10;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.regex.Pattern;

import com.github.saser.adventofcode.Result;

public final class Day10 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var network = Network.parse(r);
        var paths = network.flow();
        if (part == 1) {
            var visited61 = new HashSet<>(paths.get(61));
            var visited17 = new HashSet<>(paths.get(17));
            visited61.retainAll(visited17);
            var bot = visited61.iterator().next();
            return Result.ok(Integer.toString(bot));
        }
        var product = 1;
        var found = new boolean[3];
        for (var entry : paths.entrySet()) {
            var list = entry.getValue();
            var output = list.get(list.size() - 1);
            if (output >= 0 && output <= 2) {
                if (found[output]) {
                    continue;
                }
                found[output] = true;
                product *= entry.getKey();
            }
            if (found[0] && found[1] && found[2]) {
                break;
            }
        }
        return Result.ok(Integer.toString(product));
    }

    private static class Network {
        public final Map<Integer, Sink> sources;
        public final Map<Integer, Sink> highSinks;
        public final Map<Integer, Sink> lowSinks;

        public Network(Map<Integer, Sink> sources, Map<Integer, Sink> highSinks, Map<Integer, Sink> lowSinks) {
            this.sources = sources;
            this.highSinks = highSinks;
            this.lowSinks = lowSinks;
        }

        public static Network parse(Reader r) {
            var sources = new HashMap<Integer, Sink>();
            var highSinks = new HashMap<Integer, Sink>();
            var lowSinks = new HashMap<Integer, Sink>();
            var br = new BufferedReader(r);
            var it = br.lines().iterator();
            var sourceRE = Pattern.compile("value (\\d+) goes to bot (\\d+)");
            var highLowRE = Pattern.compile("bot (\\d+) gives (low|high) to (output|bot) (\\d+) and (low|high) to (output|bot) (\\d+)");
            while (it.hasNext()) {
                var line = it.next();
                var sourceMatcher = sourceRE.matcher(line);
                if (sourceMatcher.matches()) {
                    var value = Integer.parseInt(sourceMatcher.group(1));
                    var bot = Integer.parseInt(sourceMatcher.group(2));
                    sources.put(value, new Sink(bot, false));
                    continue;
                }
                var highLowMatcher = highLowRE.matcher(line);
                if (highLowMatcher.matches()) {
                    var sourceBot = Integer.parseInt(highLowMatcher.group(1));
                    var isLow1 = highLowMatcher.group(2).equals("low");
                    var isOutput1 = highLowMatcher.group(3).equals("output");
                    var number1 = Integer.parseInt(highLowMatcher.group(4));
                    (isLow1 ? lowSinks : highSinks).put(sourceBot, new Sink(number1, isOutput1));
                    var isLow2 = highLowMatcher.group(5).equals("low");
                    var isOutput2 = highLowMatcher.group(6).equals("output");
                    var number2 = Integer.parseInt(highLowMatcher.group(7));
                    (isLow2 ? lowSinks : highSinks).put(sourceBot, new Sink(number2, isOutput2));
                    continue;
                }
                throw new IllegalArgumentException(String.format("invalid line: %s", line));
            }
            return new Network(sources, highSinks, lowSinks);
        }

        public Map<Integer, List<Integer>> flow() {
            var q = new LinkedList<Pair<Integer, Sink>>();
            for (var entry : this.sources.entrySet()) {
                q.add(new Pair<>(entry.getKey(), entry.getValue()));
            }
            var paths = new HashMap<Integer, List<Integer>>();
            var held = new HashMap<Integer, Integer>();
            while (!q.isEmpty()) {
                var entry = q.removeFirst();
                var value = entry.first;
                var sink = entry.second;
                if (!paths.containsKey(value)) {
                    paths.put(value, new LinkedList<>());
                }
                paths.get(value).add(sink.target);
                if (sink.isOutput) {
                    continue;
                }
                var bot = sink.target;
                if (!held.containsKey(bot)) {
                    held.put(bot, value);
                    continue;
                }
                var current = held.get(bot);
                var low = Math.min(current, value);
                var high = Math.max(current, value);
                q.addLast(new Pair<>(low, lowSinks.get(bot)));
                q.addLast(new Pair<>(high, highSinks.get(bot)));
            }
            return paths;
        }

        private static class Pair<T1, T2> {
            public final T1 first;
            public final T2 second;

            public Pair(T1 first, T2 second) {
                this.first = first;
                this.second = second;
            }
        }
    }

    private static class Sink {
        public final int target;
        public final boolean isOutput;

        public Sink(int target, boolean isOutput) {
            this.target = target;
            this.isOutput = isOutput;
        }
    }
}
