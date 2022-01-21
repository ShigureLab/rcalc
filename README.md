# rcalc <sup>[Experimental]</sup>

<p align="center">
   <a href="https://github.com/ShigureLab"><img src="https://img.shields.io/badge/ShigureLab-cyan?style=flat-square" alt="ShigureLab"></a>
   <a href="https://actions-badge.atrox.dev/ShigureLab/rcalc/goto?ref=main"><img alt="Build Status" src="https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2FShigureLab%2Frcalc%2Fbadge%3Fref%3Dmain&label=Tests&style=flat-square" /></a>
   <a href="https://gitmoji.dev"><img src="https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67?style=flat-square" alt="Gitmoji"></a>
   <a href="LICENSE"><img alt="LICENSE" src="https://img.shields.io/github/license/ShigureLab/rcalc?style=flat-square"></a>
</p>

一个使用 Rust 编写的简单计算器，可通过 LLVM 进行 JIT 编译。

探索博客见[nyakku.moe](https://nyakku.moe/posts/2022/01/21/lets-make-a-calculator-using-rust-and-llvm.html)

## Usage

```bash
cargo run -- -a=1 -b=-2 "a + b / PI" --jit
```

## References

-  [Rusty Calc](https://michael-f-bryan.github.io/calc/book/html/intro.html)
-  [inkwell examples](https://github.com/TheDan64/inkwell)
-  [building-fast-interpreters-in-rust](https://www.slideshare.net/RReverser/building-fast-interpreters-in-rust)
