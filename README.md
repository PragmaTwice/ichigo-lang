# Ichigo Lang

![ichigo-logo](doc/logo.png)

*to commemorate those fucking times*

## Intro

- It's trivial, just a toy, used to familiarize with Rust
- **NO DENPENDENT TYPE**

## Quickview

```ichigo
ℕ = τ {
    0    : ℕ
    succ : ℕ → ℕ
}

plus = λ x : ℕ. λ { 
    (succ y) : ℕ. succ (plus x y)
    0        : ℕ. x
}

ℕ𝓁 = τ { 
    nil : ℕ𝓁
    cons: ℕ𝓁 → ℕ → ℕ𝓁
}

𝔹 = τ { 
    true  : 𝔹
    false : 𝔹
}

```
