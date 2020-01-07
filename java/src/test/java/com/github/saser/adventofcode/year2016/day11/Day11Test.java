package com.github.saser.adventofcode.year2016.day11;

import java.io.FileReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.StringReader;

import org.junit.Test;
import org.junit.Assert;

public class Day11Test {
    @Test
    public void part1Example() {
        var input = new InputStreamReader(this.getClass().getResourceAsStream("example"));
        var output = "11";
        var result = Day11.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/11")) {
            var output = "31";
            var result = Day11.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

     @Test
     public void part2Actual() throws IOException {
         try (var input = new FileReader("inputs/2016/11")) {
             var output = "55";
             var result = Day11.part2(input);
             Assert.assertEquals("no error", "", result.error);
             Assert.assertEquals("correct output", output, result.answer);
         }
     }
}
