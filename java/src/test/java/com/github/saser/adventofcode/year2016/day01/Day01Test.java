package com.github.saser.adventofcode.year2016.day01;

import java.io.FileReader;
import java.io.IOException;
import java.io.StringReader;

import org.junit.Test;
import org.junit.Assert;

public class Day01Test {
    @Test
    public void part1Example1() {
        var input = new StringReader("R2, L3");
        var output = "5";
        var result = Day01.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Example2() {
        var input = new StringReader("R2, R2, R2");
        var output = "2";
        var result = Day01.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Example3() {
        var input = new StringReader("R5, L5, R5, R3");
        var output = "12";
        var result = Day01.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/01")) {
            var output = "243";
            var result = Day01.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

    @Test
    public void part2Example() {
        var input = new StringReader("R8, R4, R4, R8");
        var output = "4";
        var result = Day01.part2(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part2Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/01")) {
            var output = "142";
            var result = Day01.part2(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }
}
