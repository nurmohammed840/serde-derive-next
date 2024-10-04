### System Info

- CPU: `Ryzen 7 5700 (3.8GHz - 4.6GHz)`

### Run Benchmark

> `cargo r -rp bench`


```
Iter: 1
[serde_derive: simple.rs] serialize: 395.7µs
[serde_derive_next: simple.rs] serialize: 189.6µs

[serde_derive: user.rs] serialize: 368.4µs
[serde_derive_next: user.rs] serialize: 231.3µs


Iter: 10
[serde_derive: simple.rs] serialize: 539.8µs
[serde_derive_next: simple.rs] serialize: 320.7µs

[serde_derive: user.rs] serialize: 2.8425ms
[serde_derive_next: user.rs] serialize: 1.7323ms


Iter: 100
[serde_derive: simple.rs] serialize: 4.6661ms
[serde_derive_next: simple.rs] serialize: 2.9735ms

[serde_derive: user.rs] serialize: 27.0114ms
[serde_derive_next: user.rs] serialize: 15.3233ms


Iter: 10000
[serde_derive: simple.rs] serialize: 457.4681ms
[serde_derive_next: simple.rs] serialize: 288.3544ms

[serde_derive: user.rs] serialize: 2.4042527s
[serde_derive_next: user.rs] serialize: 1.4237032s
```