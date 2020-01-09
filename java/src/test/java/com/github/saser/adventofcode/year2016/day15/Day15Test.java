package com.github.saser.adventofcode.year2016.day15;

import java.io.FileReader;
import java.io.IOException;
import java.io.InputStreamReader;

import org.junit.Test;
import org.junit.Assert;

public class Day15Test {
    @Test
    public void part1Example() {
        var input = new InputStreamReader(this.getClass().getResourceAsStream("example"));
        var output = "5";
        var result = Day15.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/15")) {
            var output = "148737";
            var result = Day15.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

     @Test
     public void part2Actual() throws IOException {
         try (var input = new FileReader("inputs/2016/15")) {
             var output = "2353212";
             var result = Day15.part2(input);
             Assert.assertEquals("no error", "", result.error);
             Assert.assertEquals("correct output", output, result.answer);
         }
     }
}
