package com.github.saser.adventofcode.year2016.day08;

import java.io.BufferedReader;
import java.io.Reader;
import java.util.Arrays;
import java.util.regex.Pattern;

import com.github.saser.adventofcode.Result;

public final class Day08 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var grid = new boolean[6][50];
        var it = new BufferedReader(r).lines().iterator();
        while (it.hasNext()) {
            var line = it.next();
            var abMatcher = Pattern.compile("(\\d+)[^0-9]+(\\d+)").matcher(line);
            if (!abMatcher.find()) {
                return Result.err(String.format("invalid input line: %s", line));
            }
            var a = Integer.parseInt(abMatcher.group(1));
            var b = Integer.parseInt(abMatcher.group(2));
            if (line.contains("rect")) {
                grid = turnOn(grid, a, b);
            } else if (line.contains("rotate row")) {
                grid = rotateRowRight(grid, a, b);
            } else if (line.contains("rotate column")) {
                grid = rotateColumnDown(grid, a, b);
            } else {
                return Result.err(String.format("invalid input line: %s", line));
            }
        }
        var count = Arrays.stream(grid)
                .mapToInt((row) -> {
                    var c = 0;
                    for (var b : row) {
                        if (b) {
                            c++;
                        }
                    }
                    return c;
                })
                .sum();
        return Result.ok(Long.toString(count));
    }

    private static boolean[][] clone(boolean[][] grid) {
        var nRows = grid.length;
        var nCols = grid[0].length;
        var gridCopy = new boolean[nRows][nCols];
        for (var row = 0; row < nRows; row++) {
            gridCopy[row] = grid[row].clone();
        }
        return gridCopy;
    }

    private static boolean[][] transpose(boolean[][] grid) {
        var nRows = grid.length;
        var nCols = grid[0].length;
        var gridT = new boolean[nCols][nRows];
        for (var row = 0; row < nRows; row++) {
            for (var col = 0; col < nCols; col++) {
                gridT[col][row] = grid[row][col];
            }
        }
        return gridT;
    }

    private static boolean[][] turnOn(boolean[][] grid, int width, int height) {
        var copy = clone(grid);
        for (var row = 0; row < height; row++) {
            for (var col = 0; col < width; col++) {
                copy[row][col] = true;
            }
        }
        return copy;
    }

    private static boolean[][] rotateRowRight(boolean[][] grid, int row, int n) {
        var nCols = grid[row].length;
        var rotated = clone(grid);
        for (var col = 0; col < nCols; col++) {
            rotated[row][(col + n) % nCols] = grid[row][col];
        }
        return rotated;
    }

    private static boolean[][] rotateColumnDown(boolean[][] grid, int col, int n) {
        return transpose(rotateRowRight(transpose(grid), col, n));
    }
}
