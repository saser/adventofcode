package com.github.saser.adventofcode.geo;

public final class Point2D {
    public int x;
    public int y;

    public Point2D(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public Point2D(Point2D other) {
        this.x = other.x;
        this.y = other.y;
    }

    public int manhattanDistance() {
        return Math.abs(this.x) + Math.abs(this.y);
    }

    public void add(Point2D other) {
        this.x += other.x;
        this.y += other.y;
    }

    @Override
    public Point2D clone() {
        return new Point2D(this);
    }
}
