package com.github.saser.adventofcode.tuple;

import java.util.Objects;

public class Tuple2<T1, T2> {
    public final T1 v1;
    public final T2 v2;

    public Tuple2(T1 v1, T2 v2) {
        this.v1 = v1;
        this.v2 = v2;
    }

    @Override
    public String toString() {
        var s = new String[] {
                this.v1.toString(),
                this.v2.toString(),
        };
        return "(" + String.join(", ", s) + ")";
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Tuple2<?, ?> tuple2 = (Tuple2<?, ?>) o;
        return v1.equals(tuple2.v1) &&
                v2.equals(tuple2.v2);
    }

    @Override
    public int hashCode() {
        return Objects.hash(v1, v2);
    }
}
