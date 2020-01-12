package com.github.saser.adventofcode.year2016.day19;

import java.io.FileReader;
import java.io.IOException;
import java.io.StringReader;

import org.junit.Test;
import org.junit.Assert;

public class Day19Test {
    @Test
    public void part1Example() {
        var input = new StringReader("5");
        var output = "3";
        var result = Day19.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/2016/19")) {
            var output = "1830117";
            var result = Day19.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

     @Test
     public void part2Example() {
         var input = new StringReader("5");
         var output = "2";
         var result = Day19.part2(input);
         Assert.assertEquals("no error", "", result.error);
         Assert.assertEquals("correct output", output, result.answer);
     }

     @Test
     public void part2Actual() throws IOException {
         try (var input = new FileReader("inputs/2016/19")) {
             var output = "1417887";
             var result = Day19.part2(input);
             Assert.assertEquals("no error", "", result.error);
             Assert.assertEquals("correct output", output, result.answer);
         }
     }
}
