package com.github.saser.adventofcode.geo;

import java.util.Objects;

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
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Point2D point2D = (Point2D) o;
        return x == point2D.x &&
                y == point2D.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }

    @Override
    public String toString() {
        return String.format("(%d, %d)", this.x, this.y);
    }

    @Override
    public Point2D clone() {
        return new Point2D(this);
    }
}
