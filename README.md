December is one of the busiest months of the year so there's practically no chance I'll complete Advent Of Code.
So instead I'll have some fun :)

# Day01
To spice up day01, I implemented benchmarking with Criterion. I used the benchmarking to rewrite the solution
to star01 several times to explore the space of possible solutions and refine the solution to be as fast as
possible in wall clock time. The result was a 10x speedup from my initial solution:

```
$> cargo bench
...
extract_number          time:   [20.921 ns 21.010 ns 21.095 ns]
                        change: [-0.4626% +0.3540% +1.1606%] (p = 0.41 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

extract_number2         time:   [10.217 ns 10.283 ns 10.357 ns]
                        change: [+2.9173% +3.8817% +4.9291%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

extract_number3         time:   [8.2167 ns 8.2530 ns 8.2915 ns]
                        change: [-1.4631% -0.5633% +0.3263%] (p = 0.22 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

extract_number4         time:   [3.3919 ns 3.4242 ns 3.4583 ns]
                        change: [+20.334% +22.106% +23.811%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) low severe
  2 (2.00%) high mild

extract_number5         time:   [2.8194 ns 2.8326 ns 2.8471 ns]
                        change: [+0.2194% +0.8988% +1.6400%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

extract_number6         time:   [5.0942 ns 5.1403 ns 5.1849 ns]
                        change: [+4.0625% +5.1441% +6.2147%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

Additionally I leveraged doc tests which I haven't used much in the past.
