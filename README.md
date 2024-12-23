Questionable TentHash
==

An implementation of [TentHash](https://github.com/cessen/tenthash) in rust largely modeling my approach to writing Advent of Code solutions. Written for fun, and to poke at benchmarking in rust. The benchmark code is ripped off from [TentHashs benchmark logic](https://github.com/cessen/tenthash/tree/main/tenthash-rust/benches)

Results:

```
------ Rust Impl from TentHash Spec ------
tent_hash_nathan/10b_message
                        time:   [17.078 ns 17.141 ns 17.222 ns]
                        thrpt:  [553.76 MiB/s 556.37 MiB/s 558.44 MiB/s]
tent_hash_nathan/100b_message
                        time:   [24.370 ns 24.501 ns 24.669 ns]
                        thrpt:  [3.7752 GiB/s 3.8011 GiB/s 3.8216 GiB/s]
tent_hash_nathan/1kb_message
                        time:   [127.22 ns 128.06 ns 129.07 ns]
                        thrpt:  [7.2157 GiB/s 7.2728 GiB/s 7.3208 GiB/s]
tent_hash_nathan/10kb_message
                        time:   [1.2060 µs 1.2283 µs 1.2496 µs]
                        thrpt:  [7.4529 GiB/s 7.5824 GiB/s 7.7223 GiB/s]
tent_hash_nathan/100kb_message
                        time:   [12.256 µs 12.330 µs 12.411 µs]
                        thrpt:  [7.5041 GiB/s 7.5535 GiB/s 7.5988 GiB/s]
tent_hash_nathan/1mb_message
                        time:   [123.52 µs 124.09 µs 124.68 µs]
                        thrpt:  [7.4699 GiB/s 7.5051 GiB/s 7.5396 GiB/s]

------ Questionable TentHash ------
tent_hash_zaq/10b_message
                        time:   [138.46 ns 140.94 ns 144.12 ns]
                        thrpt:  [66.174 MiB/s 67.667 MiB/s 68.878 MiB/s]
tent_hash_zaq/100b_message
                        time:   [532.04 ns 533.22 ns 534.70 ns]
                        thrpt:  [178.36 MiB/s 178.85 MiB/s 179.25 MiB/s]
tent_hash_zaq/1kb_message
                        time:   [4.1125 µs 4.1213 µs 4.1303 µs]
                        thrpt:  [230.90 MiB/s 231.40 MiB/s 231.90 MiB/s]
tent_hash_zaq/10kb_message
                        time:   [40.340 µs 40.597 µs 40.894 µs]
                        thrpt:  [233.20 MiB/s 234.91 MiB/s 236.41 MiB/s]
tent_hash_zaq/100kb_message
                        time:   [399.34 µs 400.83 µs 402.42 µs]
                        thrpt:  [236.99 MiB/s 237.93 MiB/s 238.81 MiB/s]
tent_hash_zaq/1mb_message
                        time:   [4.0270 ms 4.0344 ms 4.0424 ms]
                        thrpt:  [235.92 MiB/s 236.39 MiB/s 236.82 MiB/s]
```
