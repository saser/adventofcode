package com.github.saser.adventofcode.year2016.day04;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Comparator;
import java.util.HashMap;
import java.util.Map.Entry;
import java.util.regex.Pattern;
import java.util.stream.Collector;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;

public final class Day04 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var br = new BufferedReader(r);
        var infos = br.lines()
            .map(RoomInfo::parse)
            .collect(Collectors.toList());
        if (part == 1) {
            var sum = infos.stream()
                .filter(RoomInfo::isReal)
                .mapToInt((info) -> info.sectorID)
                .sum();
            return Result.ok(Long.toString(sum));
        }
        var northPoleRoom = infos.stream()
            .filter((info) -> info.decrypt().contains("northpole"))
            .findFirst()
            .get()
            .sectorID;
        return Result.ok(Integer.toString(northPoleRoom));
    }

    private static class RoomInfo {
        public final String s;
        public final int sectorID;
        public final String checksum;

        public RoomInfo(String s, int sectorID, String checksum) {
            this.s = s;
            this.sectorID = sectorID;
            this.checksum = checksum;
        }

        public static RoomInfo parse(String line) {
            var pattern = Pattern.compile("([a-z\\-]+)-(\\d+)\\[([a-z]+)\\]");
            var matcher = pattern.matcher(line);
            if (!matcher.matches()) {
                throw new IllegalArgumentException(String.format("invalid line: %s", line));
            }
            var s = matcher.group(1);
            var sectorID = Integer.parseInt(matcher.group(2));
            var checksum = matcher.group(3);
            return new RoomInfo(s, sectorID, checksum);
        }

        public boolean isReal() {
            var counts = new HashMap<Character, Integer>();
            for (var c : this.s.toCharArray()) {
                if (c == '-') {
                    continue;
                }
                counts.merge(c, 1, (count, one) -> count + one);
            }
            var comparator = Entry.<Character, Integer>comparingByValue().reversed()
                .thenComparing(Entry.<Character, Integer>comparingByKey());
            var checksum = counts.entrySet()
                .stream()
                .sorted(comparator)
                .limit(5)
                .map(Entry::getKey)
                .collect(Collector.of(StringBuilder::new,
                                      StringBuilder::append,
                                      StringBuilder::append,
                                      StringBuilder::toString));
            return this.checksum.equals(checksum);
        }

        public char decryptChar(char c) {
            if (c == '-') {
                return ' ';
            } else {
                c -= 'a';
                c += this.sectorID;
                c %= 26;
                c += 'a';
                return c;
            }
        }

        public String decrypt() {
            return this.s
                .chars()
                .mapToObj((c) -> this.decryptChar((char) c))
                .collect(Collector.of(StringBuilder::new,
                                      StringBuilder::append,
                                      StringBuilder::append,
                                      StringBuilder::toString));
        }
    }
}
