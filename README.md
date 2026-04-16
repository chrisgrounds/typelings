# Typelings

Advanced functional and type-level programming exercises in Rust.

| Content |
| --- |
| Initial Encoding, Tagless Final |

**Task**: Run tests and pass them.

```
cargo test
```

## Initial and Final Encodings

It makes most sense to proceed in the following order: 

1. `initial_encoding_01.rs`
2. `initial_encoding_02.rs`
3. `tagless_final_01.rs`
4. `tagless_final_02.rs`

### Overview

Initial encoding is a pattern whereby we define EDSLs by their syntax. In Rust we can do that with enums/sum types. Tagless final is a pattern whereby we define EDSLs by their semantics (i.e. trait interfaces) and leave them "tagless" as they have no syntactic "tag" associated with them. Tagless final is an attempt to solve the "expression problem" and satisfy the Open-Closed Principle. There are advantages and disadvantages of both patterns, which hopefully become clear through the exercises.

#### References

- [Tagless Final, Partially Evaluated](https://okmij.org/ftp/tagless-final/JFP.pdf)
- [Initial and Final Encodings](https://peddie.github.io/encodings/encodings-text.html)
