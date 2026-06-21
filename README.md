# Mirl Extensions (0.0.0-alpha)

### Mext - Library defining only traits and macros to silently enhance the rust experience

> Sub-crates: `mirl_extensions_core` - Macros and traits that don't auto impl

**Purpose:**

- Patching holes (Like conversions)
- Giving structure to data (Universal Map type, universal Vec type, etc)
- Reimplementing functions lost from `#![no_std]`
- Expand available functions on objects to reduce code duplication

Most structure defining traits are dyn-compatible



**Entry Points:**

- `use mirl_extensions::*;`
