#!/usr/bin/zsh

n_iters=20
benchtime="200ms"

baseline_commit=a600c26
candidate_commit=01862fd

git checkout "$baseline_commit"

for i in {1..$n_iters}; do
    echo go test -bench=Split -benchtime="$benchtime" -benchmem ./internal/striter \>"baseline$i.txt"
    time go test -bench=Split -benchtime="$benchtime" -benchmem ./internal/striter >"baseline$i.txt"
done

cat baseline*.txt >full_baseline.txt

git checkout "$candidate_commit"

for i in {1..$n_iters}; do
    echo go test -bench=Split -benchtime="$benchtime" -benchmem ./internal/striter \>"candidate$i.txt"
    time go test -bench=Split -benchtime="$benchtime" -benchmem ./internal/striter >"candidate$i.txt"
done

cat candidate*.txt >full_candidate.txt
