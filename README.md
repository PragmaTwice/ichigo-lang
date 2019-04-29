# Ichigo Lang

![ichigo-logo](doc/logo.png)

*to commemorate those fucking times*

## Intro

- It's trivial, just a toy, used to familiarize with Rust
- **NO DENPENDENT TYPE**

## Quickview

```ichigo
ℕ := { 0    : nat
       succ : nat → nat
     }

plus = λ x : ℕ. λ { (succ y) : ℕ. succ (plus x y)
                    0        : ℕ. x
                  }

```
