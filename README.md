### System Info

- CPU: `Ryzen 7 5700 (3.8GHz - 4.6GHz)`

### Run Benchmark

> `cargo r -rp bench`


```
Iter: 1
[serde_derive: simple.rs] serialize: 304.9µs
[serde_derive_next: simple.rs] serialize: 148.7µs


Iter: 10
[serde_derive: simple.rs] serialize: 556.1µs
[serde_derive_next: simple.rs] serialize: 358.9µs


Iter: 100
[serde_derive: simple.rs] serialize: 4.8053ms
[serde_derive_next: simple.rs] serialize: 3.1388ms


Iter: 10000
[serde_derive: simple.rs] serialize: 473.2502ms
[serde_derive_next: simple.rs] serialize: 303.6848ms
```