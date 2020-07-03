# into-iter-collect-optimization

Benchmark trying to (dis-) prove whether Rustc or LLVM can optimize away a
superfluous `some_collection.into_iter().collect::<SameCollection>()` across
function boundaries.

```bash
$ cargo bench

into_iterator_collect_optimization/identity
                        time:   [4.7218 ns 4.7423 ns 4.7630 ns]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
  
  
into_iterator_collect_optimization/identity_as_iterator
                        time:   [2.0384 us 2.0455 us 2.0532 us]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
```
