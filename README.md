# Sprint

Sprint is a functional programming language for writing smart contracts. It compiles into Move for Libra.

## Compiling a contract from sprint to Move IR

Clone this repository. In its root directory, write a sprint program in a new file, such as `example.sprint`. Then you can run:

`cargo run -- example.sprint`

This compiles the sprint code and saves it into a file `example.mvir`. Have a look at the generated Move code!
