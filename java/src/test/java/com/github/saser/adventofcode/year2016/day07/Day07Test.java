package com.github.saser.adventofcode.year2016.day07;

import java.io.FileReader;
import java.io.IOException;
import java.io.InputStreamReader;

import org.junit.Test;
import org.junit.Assert;

public class Day07Test {
    @Test
    public void part1Example() {
        var input = new InputStreamReader(this.getClass().getResourceAsStream("p1example"));
        var output = "2";
        var result = Day07.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/07")) {
            var output = "";
            var result = Day07.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

    // @Test
    // public void part2Example() {
    //     var input = new StringReader("");
    //     var output = "";
    //     var result = Day07.part2(input);
    //     Assert.assertEquals("no error", "", result.error);
    //     Assert.assertEquals("correct output", output, result.answer);
    // }

    // @Test
    // public void part2Actual() throws IOException {
    //     try (var input = new FileReader("inputs/2016/07")) {
    //         var output = "";
    //         var result = Day07.part2(input);
    //         Assert.assertEquals("no error", "", result.error);
    //         Assert.assertEquals("correct output", output, result.answer);
    //     }
    // }
}
