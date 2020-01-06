package com.github.saser.adventofcode.year2016.day09;

import java.io.BufferedReader;
import java.io.Reader;

import com.github.saser.adventofcode.Result;

public final class Day09 {
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
            return Result.ok(Long.toString(decompressedLength(input)));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static long decompressedLength(String s) {
        var length = 0;
        for (var i = 0; i < s.length(); i++) {
            var markerStart = s.indexOf('(', i);
            if (markerStart == -1) {
                length += s.length() - i;
                break;
            }
            length += markerStart - i;
            var markerEnd = s.indexOf(')', markerStart);
            var marker = parseMarker(s.substring(markerStart + 1, markerEnd));
            var dataStart = markerEnd + 1;
            var dataEnd = markerEnd + marker[0];
            length += (dataEnd - dataStart + 1) * marker[1];
            i = dataEnd;
        }
        return length;
    }

    private static int[] parseMarker(String marker) {
        var parts = marker.split("x");
        var length = Integer.parseInt(parts[0]);
        var repeat = Integer.parseInt(parts[1]);
        return new int[] {length, repeat};
    }
}
