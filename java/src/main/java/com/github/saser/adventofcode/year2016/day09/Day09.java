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
            var length = decompressedLength(input, part == 2);
            return Result.ok(Long.toString(length));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static long decompressedLength(String s, boolean recursive) {
        var length = 0L;
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
            var data = s.substring(dataStart, dataEnd + 1);
            var dataLength = recursive ? decompressedLength(data, recursive) : data.length();
            length += dataLength * marker[1];
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
