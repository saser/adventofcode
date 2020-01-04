package com.github.saser.adventofcode.year2016.day05;

import java.io.BufferedReader;
import java.io.Reader;
import java.security.MessageDigest;
import java.util.stream.Collector;
import java.util.stream.Stream;

import com.github.saser.adventofcode.Result;

public final class Day05 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        try {
            var br = new BufferedReader(r);
            var doorID = br.readLine();
            var digests = interestingDigests(doorID);
            return Result.ok(password(digests));
        } catch (Exception e) {
            e.printStackTrace();
            return Result.err(e.getMessage());
        }
    }

    private static Stream<byte[]> interestingDigests(String doorID) throws Exception {
        var md = MessageDigest.getInstance("MD5");
        return Stream.iterate(0, (i) -> i + 1)
            .map((i) -> md.digest((doorID + i).getBytes()))
            .filter(Day05::isInteresting);
    }

    private static boolean isInteresting(byte[] digest) {
        return digest[0] == 0x00 && digest[1] == 0x00 && digest[2] >= 0x00 && digest[2] <= 0x0f;
    }

    private static String password(Stream<byte[]> digests) {
        return digests
            .limit(8)
            .map((digest) -> Character.forDigit(digest[2], 16))
            .collect(Collector.of(StringBuilder::new,
                                  StringBuilder::append,
                                  StringBuilder::append,
                                  StringBuilder::toString));
    }
}
