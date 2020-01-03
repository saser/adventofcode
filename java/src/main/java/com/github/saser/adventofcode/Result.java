package com.github.saser.adventofcode;

public final class Result {
    public final String answer;
    public final String error;

    private Result(String answer, String error) {
        this.answer = answer;
        this.error = error;
    }

    public static Result ok(String answer) {
        return new Result(answer, "");
    }

    public static Result err(String error) {
        return new Result("", error);
    }
}
