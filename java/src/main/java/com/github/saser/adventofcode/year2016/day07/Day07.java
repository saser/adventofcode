package com.github.saser.adventofcode.year2016.day07;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

import com.github.saser.adventofcode.Result;

public final class Day07 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var br = new BufferedReader(r);
        var count = br.lines()
                .map(Address::parse)
                .filter(part == 1 ? Address::supportsTLS : Address::supportsSSL)
                .count();
        return Result.ok(Long.toString(count));
    }

    private static class Address {
        public final List<String> supernets;
        public final List<String> hypernets;

        public Address(List<String> supernets, List<String> hypernets) {
            this.supernets = supernets;
            this.hypernets = hypernets;
        }

        public static Address parse(String line) {
            var parts = line
                    .replace("[", "/_")
                    .replace(']', '/')
                    .split("/");
            var split = Arrays.stream(parts)
                    .collect(Collectors.partitioningBy((s) -> s.startsWith("_")));
            var supernets = split.get(false);
            var hypernets = split.get(true);
            return new Address(supernets, hypernets);
        }

        public boolean supportsTLS() {
            var hypernetsContainsABBA = this.hypernets.stream().anyMatch(Address::containsABBA);
            var supernetsContainsABBA = this.supernets.stream().anyMatch(Address::containsABBA);
            return !hypernetsContainsABBA && supernetsContainsABBA;
        }

        public boolean supportsSSL() {
            return this.supernets
                    .stream()
                    .flatMap((s) -> findABAs(s).stream())
                    .anyMatch((ab) -> {
                        var bab = new String(new char[] {ab[1], ab[0], ab[1]});
                        return this.hypernets
                                .stream()
                                .anyMatch((hypernet) -> hypernet.contains(bab));
                    });
        }

        private static ArrayList<char[]> findABAs(String s) {
            var abas = new ArrayList<char[]>();
            var chars = s.toCharArray();
            for (var i = 0; i < chars.length - 2; i++) {
                var a = chars[i];
                var b = chars[i + 1];
                if (a != b && chars[i + 2] == a) {
                    abas.add(new char[] {a, b});
                }
            }
            return abas;
        }

        private static boolean containsABBA(String s) {
            var chars = s.toCharArray();
            for (var i = 0; i < chars.length - 3; i++) {
                if (chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1]) {
                    return true;
                }
            }
            return false;
        }
    }
}
