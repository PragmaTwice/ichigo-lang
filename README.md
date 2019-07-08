# Ichigo Lang

![ichigo-logo](doc/logo.png)

*to commemorate those fucking times*

[![Build Status](https://travis-ci.com/PragmaTwice/ichigo-lang.svg?branch=master)](https://travis-ci.com/PragmaTwice/ichigo-lang)
[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Intro

- It's trivial, just a toy, used to familiarize with Rust
- **NO DEPENDENT TYPE**

## Quick Start
```sh
cargo run example/hello.ichigo
```

## TODO List
- [x] parser
- [x] type checker
- [x] type inference
- [ ] evaluator

## Language Feature
- Algebraic data type
- Lambda calculus
- Pattern matching
- Unicode symbol

## Quick View

```ichigo
ℕ = σ {
    0    : ℕ
    1+   : ℕ → ℕ
}

+ = λ x : ℕ . λ { 
    (1+ y) . 1+ (+ x y)
    0      . x
}

ℕ𝓁 = σ {
    ∅  : ℕ𝓁
    ++ : ℕ → ℕ𝓁 → ℕ𝓁
}

take = λ {
    0      . λ x : ℕ𝓁. ∅
    (1+ n) . λ {
            ∅         . ∅
            (++ x xs) . take n xs
    }
}

```
