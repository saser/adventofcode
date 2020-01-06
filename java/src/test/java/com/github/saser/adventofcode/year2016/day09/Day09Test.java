package com.github.saser.adventofcode.year2016.day09;

import java.io.FileReader;
import java.io.IOException;
import java.io.StringReader;

import org.junit.Test;
import org.junit.Assert;

public class Day09Test {
    @Test
    public void part1Example1() {
        var input = new StringReader("ADVENT");
        var output = "6";
        var result = Day09.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Example2() {
        var input = new StringReader("A(1x5)BC");
        var output = "7";
        var result = Day09.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Example3() {
        var input = new StringReader("(3x3)XYZ");
        var output = "9";
        var result = Day09.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Example4() {
        var input = new StringReader("A(2x2)BCD(2x2)EFG");
        var output = "11";
        var result = Day09.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Example5() {
        var input = new StringReader("(6x1)(1x3)A");
        var output = "6";
        var result = Day09.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Example6() {
        var input = new StringReader("X(8x2)(3x3)ABCY");
        var output = "18";
        var result = Day09.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/09")) {
            var output = "";
            var result = Day09.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

    // @Test
    // public void part2Example() {
    //     var input = new StringReader("");
    //     var output = "";
    //     var result = Day09.part2(input);
    //     Assert.assertEquals("no error", "", result.error);
    //     Assert.assertEquals("correct output", output, result.answer);
    // }

    // @Test
    // public void part2Actual() throws IOException {
    //     try (var input = new FileReader("inputs/2016/09")) {
    //         var output = "";
    //         var result = Day09.part2(input);
    //         Assert.assertEquals("no error", "", result.error);
    //         Assert.assertEquals("correct output", output, result.answer);
    //     }
    // }
}
