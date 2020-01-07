package com.github.saser.adventofcode.year2016.day11;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Objects;
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
        if (part == 2) {
            var extraItems = new String[] {
                    "elerium generator",
                    "elerium microchip",
                    "dilithium generator",
                    "dilithium microchip",
            };
            state.items.get(1).addAll(List.of(extraItems));
        }
        var steps = moveToFourth(state);
        return Result.ok(Integer.toString(steps));
    }

    private static int moveToFourth(State start) {
        var queue = new LinkedList<Map.Entry<State, Integer>>();
        queue.add(Map.entry(start, 0));
        var visited = new HashSet<Integer>();
        while (!queue.isEmpty()) {
            var e = queue.remove();
            var state = e.getKey();
            var steps = e.getValue();
            var itemsBelow = 0;
            for (var floor = 1; floor < 4; floor++) {
                itemsBelow += state.items.get(floor).size();
            }
            if (itemsBelow == 0) {
                return steps;
            }
            var hash = state.characteristicHashCode();
            if (visited.contains(hash)) {
                continue;
            }
            visited.add(hash);
            for (var nextState : state.nextStates()) {
                queue.add(Map.entry(nextState, steps + 1));
            }
        }
        throw new IllegalArgumentException("invalid starting state");
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

        @Override
        public State clone() {
            var items = new HashMap<Integer, Set<String>>();
            for (var e : this.items.entrySet()) {
                items.put(e.getKey(), new HashSet<>(e.getValue()));
            }
            return new State(this.elevator, items);
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            State state = (State) o;
            return elevator == state.elevator &&
                    items.equals(state.items);
        }

        @Override
        public int hashCode() {
            return Objects.hash(elevator, items);
        }

        public int characteristicHashCode() {
            var nMicrochips = new int[4];
            var nGenerators = new int[4];
            for (var floor = 1; floor <= 4; floor++) {
                nMicrochips[floor - 1] = this.elements(floor, true).size();
                nGenerators[floor - 1] = this.elements(floor, false).size();
            }
            return Objects.hash(this.elevator, Arrays.hashCode(nMicrochips), Arrays.hashCode(nGenerators));
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

        public Set<State> nextStates() {
            var next = new HashSet<State>();
            next.addAll(this.nextStates(+1));
            next.addAll(this.nextStates(-1));
            return next;
        }

        private Set<State> nextStates(int delta) {
            var next = new HashSet<State>();
            var newFloor = this.elevator + delta;
            if (newFloor < 1 || newFloor > 4) {
                return next;
            }
            var moved = this.clone();
            moved.moveElevator(delta);
            var itemsHere = this.items.get(this.elevator).toArray(new String[0]);
            for (var i1 = 0; i1 < itemsHere.length; i1++) {
                var item1 = itemsHere[i1];
                var moved1 = moved.clone();
                moved1.moveItem(item1, delta);
                if (moved1.isSafe()) {
                    next.add(moved1);
                }
                for (var i2 = i1 + 1; i2 < itemsHere.length; i2++) {
                    var item2 = itemsHere[i2];
                    var moved2 = moved1.clone();
                    moved2.moveItem(item2, delta);
                    if (moved2.isSafe()) {
                        next.add(moved2);
                    }
                }
            }
            return next;
        }

        private void moveElevator(int delta) {
            this.elevator += delta;
        }

        private void moveItem(String item, int delta) {
            for (var floor = 1; floor <= 4; floor++) {
                if (this.items.get(floor).remove(item)) {
                    this.items.get(floor + delta).add(item);
                    break;
                }
            }
        }
    }
}
