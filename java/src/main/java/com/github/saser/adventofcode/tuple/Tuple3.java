package com.github.saser.adventofcode.tuple;

import java.util.Objects;

public class Tuple3<T1, T2, T3> {
    public final T1 v1;
    public final T2 v2;
    public final T3 v3;

    public Tuple3(T1 v1, T2 v2, T3 v3) {
        this.v1 = v1;
        this.v2 = v2;
        this.v3 = v3;
    }

    @Override
    public String toString() {
        var s = new String[] {
                this.v1.toString(),
                this.v2.toString(),
                this.v3.toString(),
        };
        return "(" + String.join(", ", s) + ")";
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Tuple3<?, ?, ?> tuple3 = (Tuple3<?, ?, ?>) o;
        return v1.equals(tuple3.v1) &&
                v2.equals(tuple3.v2) &&
                v3.equals(tuple3.v3);
    }

    @Override
    public int hashCode() {
        return Objects.hash(v1, v2, v3);
    }
}
