package com.github.saser.adventofcode.{{.FullYear}}.{{.FullDay}};

import java.io.FileReader;
import java.io.IOException;
import java.io.StringReader;

import org.junit.Test;
import org.junit.Assert;

public class Day{{.PaddedDay}}Test {
    @Test
    public void part1Example() {
        var input = new StringReader("");
        var output = "";
        var result = Day{{.PaddedDay}}.part1(input);
        Assert.assertEquals("no error", "", result.error);
        Assert.assertEquals("correct output", output, result.answer);
    }

    @Test
    public void part1Actual() throws IOException {
        try (var input = new FileReader("inputs/{{.Year}}/{{.PaddedDay}}")) {
            var output = "";
            var result = Day{{.PaddedDay}}.part1(input);
            Assert.assertEquals("no error", "", result.error);
            Assert.assertEquals("correct output", output, result.answer);
        }
    }

    // @Test
    // public void part2Example() {
    //     var input = new StringReader("");
    //     var output = "";
    //     var result = Day{{.PaddedDay}}.part2(input);
    //     Assert.assertEquals("no error", "", result.error);
    //     Assert.assertEquals("correct output", output, result.answer);
    // }

    // @Test
    // public void part2Actual() throws IOException {
    //     try (var input = new FileReader("inputs/{{.Year}}/{{.PaddedDay}}")) {
    //         var output = "";
    //         var result = Day{{.PaddedDay}}.part2(input);
    //         Assert.assertEquals("no error", "", result.error);
    //         Assert.assertEquals("correct output", output, result.answer);
    //     }
    // }
}
