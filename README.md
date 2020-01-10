![](logo.png)

## Syntax

Programs are represented as combinations of contracts, inspired by [this paper](https://www.microsoft.com/en-us/research/publication/composing-contracts-an-adventure-in-financial-engineering/) by Simon Peyton Jones. `main` is the entrypoint for all programs, a simple example is

```haskell
scaleK :: Word -> Contract -> Contract
scaleK k c = scale (konst k) c

main :: Contract
main = scaleK 10 one
```
