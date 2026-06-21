# Mirl Extensions (0.0.0-alpha)

### Mext - Library defining only traits and macros to silently enhance the rust experience

<details>
<summary>Flags</summary>

### Default:

**Core**

- `std` (Default)
  ~~- `c_compatible`~~ Only traits and impls are defined
- `all`

**Custom**
TODO

</details>

### Entry Points:

- `use mirl_extensions::*;`

### Purpose:

- Patching holes (Like conversions)
- Giving structure to data (Universal Map type, universal Vec type, etc)
- Reimplementing functions lost from `#![no_std]`
- Expand available functions on objects to reduce code duplication

Most structure defining traits are dyn-compatible

### Disclaimer

This lib is not done, more and more can and will be added to this crate.

Support with other mirl crates are also subject to reorganize for better maintainability

### Sub Crates:

- `mirl_extensions_core` - Macros and traits that don't auto impl
- `mirl_extensions_conversion` - `FromValue`, `TryFromValue` traits to convert between more items than rusts default `From` and `TryFrom`
- `mirl_extensions_math` - Math related constants and calculation traits

### Origin

While writing code, it occurred to me that I couldn't convert a `f64` into a `f32` using `.into()`.
That's strange, f64 can safely fit into f32 and all values that don't fit can just turn into `Inf`; what other conversions don't exist even though they're 'logically' possible?

Float (Lossy):

- u32 into f32
- i64 into f32
- i64 into f64
- u64 into f32
- u64 into f64
- i128 into f32
- i128 into f64
- u128 into f32
- u128 into f64
- isize into f32
- isize into f64
- usize into f32
- usize into f64
- f32 into f64

Int:

- u16 into isize
- i32 into isize
- u32 into usize
- isize into i64
- isize into i128
- usize into u64
- usize into i128
- usize into u128

And while for the floats I understand that loss of precision may be inconvenient, making the conversion exclusive to the `as` keyword means you cannot use them to create functions with generics as inputs/outputs. For the other missing conversions, I don't see why they are left open.

And so it began as `into_number`, and when more and more conversions were added, it was renamed to `into_value`. (I won't go into detail but `try_into_value` also exists with which you can convert any number into any other as long as the value fits)
