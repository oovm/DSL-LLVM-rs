Domain-Specific Language -> LLVM
================================

A example of how DSL translated to llvm.

Take a ts-like language as example.


## Lexer & Parser

```ts
let x = 1
```

## Rust AST

```
LetStatement(Symbol("x"), Expression([Integer(1)]))
```

## LLVM Object

```

```


# Possible problems

Use `rustc -Vv` to check your llvm version.

```log
rustc 1.37.0-nightly (24a9bcbb7 2019-07-04)
binary: rustc
commit-hash: 24a9bcbb7cb0d8bdc11b8252a9c13f7562c7e4ca
commit-date: 2019-07-04
host: x86_64-pc-windows-msvc
release: 1.37.0-nightly
LLVM version: 8.
```

Check `llvm-config --version` to match the version.

For the `MacOS`/ `Linux` user this is very simple, but it's more complicated on `Win`.
 
Notice that there's no `llvm-config.exe` in windows release, build form source by yourself.
Then you need to build `lld-link.exe` instead of MSVC's `link.exe`,
and then you need to find the `include/llvm-c/Config` folder and put it in the same path as `bin`,
finally you can complete the basic configurations.