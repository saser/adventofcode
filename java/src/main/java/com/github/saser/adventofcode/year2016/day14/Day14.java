package com.github.saser.adventofcode.year2016.day14;

import java.io.BufferedReader;
import java.io.Reader;
import java.security.MessageDigest;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.NavigableMap;
import java.util.Optional;
import java.util.Set;
import java.util.TreeMap;
import java.util.stream.IntStream;

import com.github.saser.adventofcode.Result;

public final class Day14 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        try {
            var salt = new BufferedReader(r).readLine();
            var hasher = new Hasher(salt);
            var key64 = hasher.keyStream()
                    .skip(63)
                    .findFirst()
                    .getAsInt();
            return Result.ok(Integer.toString(key64));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static class Hasher {
        public final String salt;
        private final MessageDigest md;
        private final NavigableMap<Integer, Character> triples;
        private final NavigableMap<Integer, Set<Character>> fivetuples;
        private int nextHash;

        public Hasher(String salt) throws Exception {
            this.salt = salt;
            this.md = MessageDigest.getInstance("MD5");
            this.triples = new TreeMap<>();
            this.fivetuples = new TreeMap<>();
            this.nextHash = 0;
        }

        private void hash(int index) {
            var hash = Day14.hexString(md.digest((this.salt + index).getBytes()));
            var groups = Day14.groups(hash);
            for (var group : groups) {
                var c = group.charAt(0);
                if (group.length() >= 3 && !this.triples.containsKey(index)) {
                    this.triples.put(index, c);
                }
                if (group.length() >= 5) {
                    if (!this.fivetuples.containsKey(index)) {
                        this.fivetuples.put(index, new HashSet<>());
                    }
                    this.fivetuples.get(index).add(c);
                }
            }
        }

        private void hashNext() {
            this.hash(this.nextHash);
            this.nextHash++;
        }

        private int nextIndex(int start, NavigableMap<Integer, ?> map) {
            Optional<Integer> index;
            do {
                index = this.nextIndex(start, this.nextHash + 1, map);
            } while (index.isEmpty());
            return index.get();
        }

        private Optional<Integer> nextIndex(int start, int limit, NavigableMap<Integer, ?> map) {
            while (this.nextHash < limit) {
                this.hashNext();
                if (map.ceilingKey(start) != null) {
                    break;
                }
            }
            return Optional.ofNullable(map.ceilingKey(start));
        }

        private int nextTriple(int start) {
            return this.nextIndex(start, this.triples);
        }

        private Optional<Integer> nextFivetuple(int start, int limit) {
            return this.nextIndex(start, limit, this.fivetuples);
        }

        private boolean hasFivetuple(int triple) {
            var c = this.triples.get(triple);
            var start = triple + 1;
            var limit = start + 1000;
            while (start < limit) {
                var optFivetuple = this.nextFivetuple(start, limit);
                if (optFivetuple.isEmpty()) {
                    break;
                }
                var fivetuple = optFivetuple.get();
                if (this.fivetuples.get(fivetuple).contains(c)) {
                    return true;
                }
                start = fivetuple + 1;
            }
            return false;
        }

        private int nextKey(int start) {
            var triple = this.nextTriple(start);
            while (true) {
                if (this.hasFivetuple(triple)) {
                    return triple;
                }
                triple = this.nextTriple(triple + 1);
            }
        }

        public IntStream keyStream() {
            return IntStream.iterate(this.nextKey(0), key -> this.nextKey(key + 1));
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

    private static List<String> groups(String s) {
        var chars = s.toCharArray();
        var groups = new ArrayList<String>(s.length());
        var start = 0;
        var c = chars[start];
        var end = 1;
        while (end < chars.length) {
            if (chars[end] != c) {
                groups.add(s.substring(start, end));
                c = chars[end];
                start = end;
            }
            end++;
        }
        groups.add(s.substring(start, end));
        return groups;
    }
}
