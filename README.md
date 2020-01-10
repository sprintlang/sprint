![](logo.png)

> Please note that the language is highly experimental and is designed for testing. It is currently in active development and we expect many breaking changes as it evolves. Stay tuned for exciting new announcements and updates :musical_note:

## Syntax

Programs are represented as combinations of contracts, inspired by [this paper](https://www.microsoft.com/en-us/research/publication/composing-contracts-an-adventure-in-financial-engineering/) by Simon Peyton Jones, with a Haskell-like syntax. `main` is the entrypoint for all programs, a simple example is

```haskell
scaleK :: Word -> Contract -> Contract
scaleK k c = scale (konst k) c

main :: Contract
main = scaleK 10 one
```

## Semantics

### Primitives

These are the lowest-level of the contract combinators and are similar to those introduced by Jones in his paper referenced above. However, this is not exhaustive list and we plan to add more primitive combinators in the near future!

#### zero

A trivial contract where neither the party nor the counterparty have any rights and obligations, for example

```haskell
zero :: Contract

main :: Contract
main = zero
```

#### one

A contract where the party is paid a single microLibra from the funds of the contract, for example

``` haskell
one :: Contract

main :: Contract
main = one
```

#### before

A contract allowing the party to acquire an inner contract *before* a given date, for example

```haskell
before :: Date -> Contract -> Contract

main :: Contract
main = before 2020-12-25T00:00:00Z one
```

#### after

A contract allowing the party to acquire an inner contract *after* a given date, for example

```haskell
after :: Date -> Contract -> Contract

main :: Contract
main = after 2020-12-25T00:00:00Z one
```

#### anytime

A contract allowing the party to acquire an inner contract at any time, for example

```haskell
anytime :: Contract -> Contract

main :: Contract
main = anytime one
```

#### give

A contract reversing the rights and obligations (between the party and counterparty) of an inner contract, for example

```haskell
give :: Contract -> Contract

main :: Contract
main = give one
```

#### or

A contract allowing the party to acquire one of two inner contracts but not both, for example

```haskell
or :: Contract -> Contract -> Contract

main :: Contract
main = or zero one
```

#### and

A contract allowing the party to acquire both of two inner contracts, for example

```haskell
and :: Contract -> Contract -> Contract

main :: Contract
main = and zero one
```

#### scale

A contract where the inner contract is scaled by a given observable, for example

```haskell
scale :: Observable Word -> Contract -> Contract

main :: Contract
main = scale (konst 10) one
```
