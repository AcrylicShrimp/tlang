# t-lang

A self-contained, modernized C-like programming language.

## Status

`t-lang` is under prototyping. Expect frequent changes and unstable behavior.

## Project Goals

- Keep the language self-contained, with no hidden compiler-special language features.
- Make the language bootstrappable.
- Support FFI as a first-class capability.
- Use Cranelift as the compiler backend.
- Support WebAssembly as much as practical.

## Non-Goals (Current)

- Extensive standard library support.
- Maximizing raw performance as a primary objective.

## Build Requirements

- Rust toolchain (`cargo`, `rustc`).
- Toolchain version is not fixed yet.
- Even after bootstrapping, the initial compiler build path still requires Rust tooling.

## Quickstart

```bash
cargo build
```

Example:

```tl
use core.printf;

fn main() -> void {
    printf("Hello, world!\n");
}
```

## Limitations

- Prototype quality and evolving behavior.
- Optimizations may be limited.

## Contributing

PRs are welcome. There is no formal contribution guideline yet.

## License

MIT
