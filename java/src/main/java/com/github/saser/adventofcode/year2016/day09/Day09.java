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
            var decompressed = decompress(input);
            return Result.ok(Integer.toString(decompressed.length()));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static String decompress(String s) {
        var sb = new StringBuilder(s.length());
        for (var i = 0; i < s.length(); i++) {
            var markerStart = s.indexOf('(', i);
            if (markerStart == -1) {
                sb.append(s.substring(i));
                break;
            }
            sb.append(s, i, markerStart);
            var markerEnd = s.indexOf(')', markerStart);
            var marker = parseMarker(s.substring(markerStart + 1, markerEnd));
            var dataStart = markerEnd + 1;
            var dataEnd = markerEnd + 1 + marker[0];
            var data = s.substring(dataStart, dataEnd);
            for (var r = 0; r < marker[1]; r++) {
                sb.append(data);
            }
            i = dataEnd - 1;
        }
        return sb.toString();
    }

    private static int[] parseMarker(String marker) {
        var parts = marker.split("x");
        var length = Integer.parseInt(parts[0]);
        var repeat = Integer.parseInt(parts[1]);
        return new int[] {length, repeat};
    }
}
