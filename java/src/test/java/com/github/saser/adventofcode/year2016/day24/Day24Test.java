package com.github.saser.adventofcode.year2016.day24;

import java.io.FileReader;
import java.io.IOException;
import java.io.InputStreamReader;

import org.junit.Test;
import org.junit.Assert;

public class Day24Test {
    @Test
    public void part1Example() {
        var input = new InputStreamReader(this.getClass().getResourceAsStream("example"));
        var output = "14";
        var result = Day24.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/24")) {
            var output = "470";
            var result = Day24.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

    // @Test
    // public void part2Example() {
    //     var input = new StringReader("");
    //     var output = "";
    //     var result = Day24.part2(input);
    //     Assert.assertEquals("no error", "", result.error);
    //     Assert.assertEquals("correct output", output, result.answer);
    // }

    // @Test
    // public void part2Actual() throws IOException {
    //     try (var input = new FileReader("inputs/2016/24")) {
    //         var output = "";
    //         var result = Day24.part2(input);
    //         Assert.assertEquals("no error", "", result.error);
    //         Assert.assertEquals("correct output", output, result.answer);
    //     }
    // }
}
