### System Info

- CPU: `Ryzen 7 5700 (3.8GHz - 4.6GHz)`

### Run Benchmark

> `cargo r -rp bench`

```
Iter: 1
[serde_derive: simple.rs] serialize: 253.5µs
[serde_derive_next: simple.rs] serialize: 120µs

[serde_derive: user.rs] serialize: 339.9µs
[serde_derive_next: user.rs] serialize: 210.9µs

Iter: 10
[serde_derive: simple.rs] serialize: 536µs
[serde_derive_next: simple.rs] serialize: 331.8µs

[serde_derive: user.rs] serialize: 2.8184ms
[serde_derive_next: user.rs] serialize: 1.6602ms

Iter: 100
[serde_derive: simple.rs] serialize: 4.6072ms
[serde_derive_next: simple.rs] serialize: 2.8672ms

[serde_derive: user.rs] serialize: 27.6456ms
[serde_derive_next: user.rs] serialize: 15.1838ms

Iter: 10000
[serde_derive: simple.rs] serialize: 447.8222ms
[serde_derive_next: simple.rs] serialize: 282.0575ms

[serde_derive: user.rs] serialize: 2.3379857s
[serde_derive_next: user.rs] serialize: 1.3935152s
```

### Result

| **Iteration** | **File**  | **serde_derive** | **serde_derive_next** | **Improvement** | **Faster** |
| ------------- | --------- | ---------------- | --------------------- | --------------- | ---------- |
| **1**         | simple.rs | 253.5µs          | 120µs                 | **52.66%**      | **2.11x**  |
|               | user.rs   | 339.9µs          | 210.9µs               | **37.96%**      | **1.61x**  |
| **10**        | simple.rs | 536µs            | 331.8µs               | **38.06%**      | **1.61x**  |
|               | user.rs   | 2.8184ms         | 1.6602ms              | **41.09%**      | **1.70x**  |
| **100**       | simple.rs | 4.6072ms         | 2.8672ms              | **37.77%**      | **1.61x**  |
|               | user.rs   | 27.6456ms        | 15.1838ms             | **45.08%**      | **1.82x**  |
| **10,000**    | simple.rs | 447.8222ms       | 282.0575ms            | **37.03%**      | **1.59x**  |
|               | user.rs   | 2.3379857s       | 1.3935152s            | **40.43%**      | **1.68x**  |
