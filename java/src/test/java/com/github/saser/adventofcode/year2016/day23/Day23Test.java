package com.github.saser.adventofcode.year2016.day23;

import java.io.FileReader;
import java.io.IOException;

import org.junit.Test;
import org.junit.Assert;

public class Day23Test {
    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/23")) {
            var output = "10440";
            var result = Day23.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

     @Test
     public void part2Actual() throws IOException {
         try (var input = new FileReader("inputs/2016/23")) {
             var output = "479007000";
             var result = Day23.part2(input);
             Assert.assertEquals("no error", "", result.error);
             Assert.assertEquals("correct output", output, result.answer);
         }
     }
}
