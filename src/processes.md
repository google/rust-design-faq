# Questions about your development processes

## How should I use tools differently from C++?

* *Use `rustfmt` automatically everywhere.* While in C++ there are many
  different coding styles, the Rust community is in agreement (at least,
  they're in agreement that it's a good idea to be in agreement). That
  is codified in `rustfmt`. Use it, automatically, on every submission.
* *Use `clippy` somewhere*. Its lints are useful.
* *Use IDEs more liberally*. Even staunch vim-adherents (your author!)
  prefer to use an IDE with Rust, because it's simply invaluable to show
  type annotations. Type information is typically invisible in the language
  so in Rust you're more reliant on tooling assistance.
* *Deny unsafe code* by default. (`#![forbid(unsafe_code)]`).
