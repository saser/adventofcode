package com.github.saser.adventofcode.year2016.day19;

import java.io.BufferedReader;
import java.io.Reader;

import com.github.saser.adventofcode.Result;

public final class Day19 {
    public static Result part1(Reader r) {
        return solve(r, 1);
    }

    public static Result part2(Reader r) {
        return solve(r, 2);
    }

    private static Result solve(Reader r, int part) {
        var input = new BufferedReader(r)
                .lines()
                .findFirst()
                .get();
        var n = Integer.parseInt(input);
        int winner;
        if (part == 1) {
            winner = play1(n);
        } else {
            winner = play2(n);
        }
        return Result.ok(Integer.toString(winner));
    }

    // I found this solution after several insights.
    // What I did first was that I wrote the elves using zero-indexing, instead
    // of 1-indexing. So for a ring with `n` elves, they are numbered as 0
    // through `n - 1`.
    // My first insight was that every other elf is removed, meaning that the
    // size of the ring is halved after one "loop" (loop meaning that the elves
    // are about to wrap around to the beginning of the ring, i.e., elf 0). What
    // happens when `n` is odd, then? It turns out that if `n` is odd, the last
    // elf in the ring (with the number `n - 1`) will play, removing elf
    // 0. Then, we have a new ring with an even number of elves, but where the
    // starting elf has number 2 and the elves' numbers increase by 2, instead
    // of 1. I noticed that this was a new instance of the same problem, but
    // with a new starting number and a new increase. So we can just solve it
    // recursively! This was my second insight.
    //
    // Call the number of the starting elf `s`, and the difference between the
    // numbers of the elves is `d`. The number of elves in the ring is `n`. If
    // `n` is odd, then elf `s` will eventually be removed, yielding a new ring
    // starting at `s + d`, and with a difference of `2 * d`. I wrote something
    // along the lines of this pseudocode for my first solution:
    //
    //     s := 0
    //     d := 2
    //     while (n > 1) {
    //         if (n % 2 == 1) {
    //             s += d
    //         }
    //         d *= 2
    //         n /= 2
    //     }
    //     return s + 1
    //
    // But wait, I recognized this... This is just summing up bits in the binary
    // representation of `n`! But something is a little off: the first bit
    // (meaning, the first iteration of the `while` loop) represents `2^1`, and
    // we ignore the last bit since the loop terminates when `n <= 1`. But then
    // we add 1 to the final result, so that is where the last bit comes back!
    // What we have done is basically a rotation of the bits in `n`: the most
    // significant bit is shifted around and becomes the least significant
    // bit. So if we can do that rotation, we are done! This was my third insight.
    //
    // Since `int`s are signed 32-bit integers in Java, we have to find the most
    // significant bit, call it `k`. Bits are 1-indexed: the first bit, meaning
    // the bit that represents `2^0`, has index 1. `Integer.highestOneBit`
    // returns a number equal to `1 << k`. We can remove the most significant
    // bit by simply calculating `n - k`. Then we shift the resulting number
    // left one step (this is where the rotation happens), and then we add on
    // the shifted out bit as the least significant bit (which is what `+ 1`
    // represents).
    private static int play1(int n) {
        int k = Integer.highestOneBit(n);
        return ((n - k) << 1) + 1;
    }

    // After trying out my original solution for all inputs up to 100, I started
    // to notice a pattern. Excerpts from the output (`n` is left, answer is right):
    //     ...
    //      7 -> 5
    //      8 -> 7
    //      9 -> 9
    //     10 -> 1
    //     11 -> 2
    //     ...
    //     17 -> 8
    //     18 -> 9
    //     19 -> 11
    //     20 -> 13
    //     ...
    // My conclusion was this: if `n` is equal to a power of 3, then `n` is the
    // winner. Otherwise, calculate the largest power of 3 less than `n`, and
    // call that `p`. If `n <= 2 * p`, then the answer is `n - p`. Otherwise,
    // meaning `n > 2 * p`, then the answer was `n - p + (n % (2 * p))`. Since
    // `n > 2 * p`, then `n % (2 * p) = n - 2 * p`, so we can simplify:
    //        n - p + (n % 2 * p)
    //     -> n - p + (n - 2 * p).
    // Therefore, the answer is `n - p + max(n - 2 * p, 0)`.
    // I am not sure how to prove that this is the correct answer, but it turns
    // out that it is.
    private static int play2(int n) {
        var p = (int) (Math.pow(3, (int) (Math.log(n) / Math.log(3))));
        if (p == n) {
            return n;
        }
        return n - p + Math.max(n - 2 * p, 0);
    }
}
