package com.github.saser.adventofcode.year2016.day08;

import java.io.FileReader;
import java.io.IOException;
import java.io.StringReader;

import org.junit.Test;
import org.junit.Assert;

public class Day08Test {
    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/08")) {
            var output = "116";
            var result = Day08.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

    // @Test
    // public void part2Example() {
    //     var input = new StringReader("");
    //     var output = "";
    //     var result = Day08.part2(input);
    //     Assert.assertEquals("no error", "", result.error);
    //     Assert.assertEquals("correct output", output, result.answer);
    // }

    // @Test
    // public void part2Actual() throws IOException {
    //     try (var input = new FileReader("inputs/2016/08")) {
    //         var output = "";
    //         var result = Day08.part2(input);
    //         Assert.assertEquals("no error", "", result.error);
    //         Assert.assertEquals("correct output", output, result.answer);
    //     }
    // }
}
