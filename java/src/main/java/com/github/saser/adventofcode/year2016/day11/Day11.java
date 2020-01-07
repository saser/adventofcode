package com.github.saser.adventofcode.year2016.day11;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;

public final class Day11 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var state = State.parse(r);
        for (var floor = 1; floor <= 4; floor++) {
            System.out.printf("%d: ", floor);
            for (var item : state.items.entrySet()) {
                System.out.printf("%s, ", item);
            }
            System.out.println();
        }
        System.out.printf("is safe: %b\n", state.isSafe());
        return Result.err("not implemented yet");
    }

    private static class State {
        public int elevator;
        public Map<Integer, Set<String>> items;

        public State(int elevator, Map<Integer, Set<String>> items) {
            this.elevator = elevator;
            this.items = items;
        }

        public static State parse(Reader r) {
            var br = new BufferedReader(r);
            var items = new HashMap<Integer, Set<String>>();
            var it = br.lines().iterator();
            var floorRE = Pattern.compile("(first|second|third|fourth)");
            var microchipRE = Pattern.compile("(\\w+)-compatible microchip");
            var generatorRE = Pattern.compile("(\\w+) generator");
            while (it.hasNext()) {
                var line = it.next();
                var floorMatcher = floorRE.matcher(line);
                if (!floorMatcher.find()) {
                    throw new IllegalArgumentException(String.format("invalid line: %s", line));
                }
                var floor = 0;
                switch (floorMatcher.group(1)) {
                    case "first":
                        floor = 1;
                        break;
                    case "second":
                        floor = 2;
                        break;
                    case "third":
                        floor = 3;
                        break;
                    case "fourth":
                        floor = 4;
                        break;
                }
                items.putIfAbsent(floor, new HashSet<>());
                var microchipMatcher = microchipRE.matcher(line);
                while (microchipMatcher.find()) {
                    items.get(floor).add(microchipMatcher.group(1) + " microchip");
                }
                var generatorMatcher = generatorRE.matcher(line);
                while (generatorMatcher.find()) {
                    items.get(floor).add(generatorMatcher.group(1) + " generator");
                }
            }
            return new State(1, items);
        }

        public boolean isSafe() {
            if (this.elevator < 1 || this.elevator > 4) {
                return false;
            }
            if (this.items.get(this.elevator).isEmpty()) {
                return false;
            }
            for (var floor = 1; floor <= 4; floor++) {
                if (!this.floorIsSafe(floor)) {
                    return false;
                }
            }
            return true;
        }

        private boolean floorIsSafe(int floor) {
            var generatorsHere = this.elements(floor, false);
            var microchipsHere = this.elements(floor, true);
            return generatorsHere.size() == 0 || generatorsHere.containsAll(microchipsHere);
        }

        private Set<String> elements(int floor, boolean wantMicrochip) {
            return this.items
                    .get(floor)
                    .stream()
                    .filter((item) -> item.contains("microchip") == wantMicrochip)
                    .map((item) -> item.split(" ")[0])
                    .collect(Collectors.toSet());
        }
    }
}
