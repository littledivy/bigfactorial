## `bigfactorial`

Tiny compile-time macro for calculating factorial of numbers upto 99 with
`num_bigint`.

> Results are shipped precalculated.

### `usage`

```rust
use bigfactorial::fact;
use num_bigint::BigInt;

fact!(45)?; // Bigint(119622220865480194561963161495657715064383733760000000000)
```

### license

MIT license
