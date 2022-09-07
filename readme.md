# static-noise

A simple crate for generation static noise at compile time.

## Example

```rust
let random_company_name: &'static str = static_noise::company_name!();
let random_super_power: &'static str = static_noise::super_power!();
let random_uuid: &'static str = static_noise::uuid!();
```

License: MIT
