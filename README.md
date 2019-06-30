# Ichigo Lang

![ichigo-logo](doc/logo.png)

*to commemorate those fucking times*

[![Build Status](https://travis-ci.com/PragmaTwice/ichigo-lang.svg?branch=master)](https://travis-ci.com/PragmaTwice/ichigo-lang)
[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

## Intro

- It's trivial, just a toy, used to familiarize with Rust
- **NO DEPENDENT TYPE**

## Quickstart
```sh
cargo run example/hello.ichigo
```

## Quickview

```ichigo
ℕ = σ {
    0    : ℕ
    1+   : ℕ → ℕ
}

+ = λ x : ℕ. λ { 
    (1+ y) : ℕ. 1+ (+ x y)
    0      : ℕ. x
}

ℕ𝓁 = σ {
    ∅  : ℕ𝓁
    ++ : ℕ → ℕ𝓁 → ℕ𝓁
}

take = λ {
    0      : ℕ. λ x : ℕ𝓁. ∅
    (1+ n) : ℕ. λ {
            ∅         : ℕ𝓁. ∅
            (++ x xs) : ℕ𝓁. take n xs
    }
}

```