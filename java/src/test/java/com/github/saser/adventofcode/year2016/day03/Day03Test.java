package com.github.saser.adventofcode.year2016.day03;

import java.io.FileReader;
import java.io.IOException;
import java.io.StringReader;

import org.junit.Test;
import org.junit.Assert;

public class Day03Test {
    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/03")) {
            var output = "993";
            var result = Day03.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

    @Test
    public void part2Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/03")) {
            var output = "1849";
            var result = Day03.part2(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }
}
