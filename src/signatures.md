# Questions about your function signatures

## Should I return an iterator or a collection?

> Pretty much always return an iterator. - AH

We suggested you [use iterators a lot in your code](./code.md#how-can-i-avoid-the-performance-penalty-of-bounds-checks). Share the love! Give iterators to your callers too.

If you *know* your caller will store the items you're returning in a concrete collection, such as a `Vec` or a `HashSet`, you may want to return that. In all other cases, return an iterator.

Your caller might:
* Collect the iterator into a `Vec`
* Collect it into a `HashSet` or some other specialized container
* Loop over the items
* Filter them or otherwise completely ignore some

Collecting the items into vector will only turn out to be right in one of these cases. In the other cases, you're wasting memory and CPU time by building a concrete collection.

This is weird for C++ programmers because iterators don't usually have robust references into the underlying data. Even Java iterators are scary, throwing `ConcurrentModificationExceptions` when you least expect it. Rust prevents that, at compile time. If you _can_ return an iterator, you should.

```mermaid
flowchart LR
    subgraph Caller
    it_ref[reference to iterator]
    end
    subgraph it_outer[Iterator]
    it[Iterator]
    it_ref --reference--> it
    end
    subgraph data[Underlying data]
    dat[Underlying data]
    it --reference--> dat
    end
```

## How flexible should my parameters be?

Which of these is best?

```rust
fn a(params: &[String]) {
    // ...
}

fn b(params: &[&str]) {
    // ...
}

fn c(params: &[impl AsRef<str>]) {
    // ...
}
```

(You'll need to make an equivalent decision in other cases, e.g. `Path` versus `PathBuf` versus `AsRef<Path>`.)

None of the options is clearly superior; for each option, there's a case it can't handle that the others can:

```rust
# fn a(params: &[String]) {
# }
# fn b(params: &[&str]) {
# }
# fn c(params: &[impl AsRef<str>]) {
# }
fn main() {
    a(&[]);
    // a(&["hi"]); // doesn't work
    a(&vec![format!("hello")]);

    b(&[]);
    b(&["hi"]);
    // b(&vec![format!("hello")]); // doesn't work

    // c(&[]); // doesn't work
    c(&["hi"]);
    c(&vec![format!("hello")]);
}
```

So you have a variety of interesting ways to _slightly_ annoy your callers under different circumstances. Which is best?

`AsRef` has some advantages: if a caller has a `Vec<String>`, they can use that directly, which would be impossible with the other options. But if they want to pass an empty list, they'll have to explicitly specify the type (for instance `&Vec::<String>::new()`).

> Not a huge fan of AsRef everywhere - it's just saving the caller typing. If you have lots of AsRef then nothing is object-safe. - MG

TL;DR: choose the middle option, `&[&str]`. If your caller happens to have a vector of `String`, it's relatively little work to get a slice of `&str`:

```rust
# fn b(params: &[&str]) {
# }

fn main() {
    // Instead of b(&vec![format!("hello")]);
    let hellos = vec![format!("hello")];
    b(&hellos.iter().map(String::as_str).collect::<Vec<_>>());
}
```

## How do I overload constructors?

You can't do this:

```rust
# struct BirthdayCard {}
impl BirthdayCard {
    fn new(name: &str) -> Self {
#       Self{}
        // ...
    }

    // Can't add more overloads:
    //
    // fn new(name: &str, age: i32) -> BirthdayCard { ... }
    //
    // fn new(name: &str, text: &str) -> BirthdayCard { ... }
}
```

If you have a default constructor, and a few variants for other cases, you can simply write them as different static methods. An idiomatic way to do this is to write a `new()` constructor and then `with_foo()` constructors that apply the given "foo" when constructing.

```rust
# struct Racoon {}
impl Racoon {
    fn new() -> Self {
#       Self{}
        // ...
    }
    fn with_age(age: usize) -> Self {
#       Self{}
        // ...
    }
}
```

If you have have a bunch of constructors and no default, it may make sense to instead provide a set of `new_foo()` constructors.

```rust
# struct Animal {}
impl Animal {
    fn new_squirrel() -> Self {
#       Self{}
        // ...
    }
    fn new_badger() -> Self {
#       Self{}
        // ...
    }
}
```

For a more complex situation, you may use [the builder pattern](https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder). The builder has a set of methods which take `&mut self` and return `&mut Self`. Then add a `build()` that returns the final constructed object.

```rust
struct BirthdayCard {}

struct BirthdayCardBuilder {}
impl BirthdayCardBuilder {
    fn new(name: &str) -> Self {
#       Self{}
        // ...
    }

    fn age(&mut self, age: i32) -> &mut Self {
#         self
        // ...
    }

    fn text(&mut self, text: &str) -> &mut Self {
#         self
        // ...
    }

    fn build(&mut self) -> BirthdayCard { BirthdayCard { /* ... */ } }
}
```

You can then [chain these](https://rust-lang.github.io/api-guidelines/type-safety.html#non-consuming-builders-preferred) into short or long constructions, passing parameters as necessary:

```rust
# struct BirthdayCard {}
#
# struct BirthdayCardBuilder {}
# impl BirthdayCardBuilder {
#     fn new(name: &str) -> BirthdayCardBuilder {
#       Self{}
#       // ...
#     }
#
#     fn age(&mut self, age: i32) -> &mut BirthdayCardBuilder {
#         self
#         // ...
#      }
#
#     fn text(&mut self, text: &str) -> &mut BirthdayCardBuilder {
#         self
#         // ...
#      }
#
#     fn build(&mut self) -> BirthdayCard { BirthdayCard { /* ... */ } }
# }
#
fn main() {
    let card = BirthdayCardBuilder::new("Paul")
        .age(64)
        .text("Happy Valentine's Day!")
        .build();
}
```

Note another advantage of builders: Overloaded constructors often don't provide all possible combinations of parameters, whereas with the builder pattern, you can combine exactly the parameters you want.

## When must I use `#[must_use]`?

> Use it on Results and mutex locks. - MG

`#[must_use]` causes a compile error if the caller ignores the return value.

Rust functions are often single-purpose. They either:

* Return a value without any side effects; or
* Do something (i.e. have side effects) and return nothing.

In neither case do you need to think about `#[must_use]`. (In the first case,
nobody would call your function unless they were going to use the result.)

`#[must_use]` is useful for those rarer functions which return a result _and_
have side effects. In most such cases, it's wise to specify `#[must_use]`, unless
the return value is truly optional (for example in
[`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)).

## When should I take parameters by value?

Move semantics are more common in Rust than in C++.

> In C++ moves tend to be an optimization, whereas in Rust they're a key semantic part of the program. - MY

To a first approximation, you should assume similar performance when passing
things by (moved) value or by reference. It's true that a move may turn out to
be a `memcpy`, but it's often optimized away.

> Express the ownership relationship in the type system, instead of trying to second-guess the compiler for efficiency. - AF

The moves are, of course, destructive - and unlike in C++, the compiler
enforces that you don't reuse a variable that has been moved.
Some C++ objects become toxic after they've moved; that's not a
risk in Rust.

So here's the heuristic: if a caller shouldn't be able to use an object again,
pass it via move semantics in order to consume it.

An extreme example: a UUID is supposed to be globally unique - it might cause a
logic error for a caller to retain knowledge of a UUID after passing it to a callee.

More generally, consume data enthusiastically to avoid logical errors during future
refactorings. For instance, if some command-line options are overridden by a
runtime choice, consume those old options - then any future refactoring which
uses them after that point will give you a compile error. This pattern is
surprisingly effective at spotting errors in your assumptions.

## Should I ever take `self` by value?

Sometimes. If you've got a member function which destroys or transforms a thing,
it should take `self` by value. Examples:

* Closing a file and returning a result code.
* A builder-pattern object which spits out the thing it was building. ([Example](https://docs.rs/bindgen/0.59.0/bindgen/struct.Builder.html#method.generate)).

## Should I return an error, or panic?

Panics should be used only for invariants, never for anything that you believe
might happen. That's especially true [for libraries](https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell)
- panicking (or asserting) should be reserved for the 'top level' code driving
the application.

> Libraries which panic are super-rude and I hate them - MY

Even in your own application code, panicking might not be wise:

> Panicking in application logic for recoverable errors makes it way harder to librarify some code - AP

If you really must have an API which can panic, add a `try_` equivalent too.

## What should my error type be?

[Rust's `Result` type](https://doc.rust-lang.org/std/result/) is parameterized
over an error type. What should you use?

For app code, consider [anyhow](https://docs.rs/anyhow/). For library code,
use your own `enum` of error conditions - you can use [thiserror](https://docs.rs/thiserror/)
to make this more pleasant.

## When should I take or return `dyn Trait`?

In either C++ or Rust, you can choose between monomorphization (that is, building
code multiple times for each permutation of parameter types) or dynamic dispatch (i.e.
looking up the correct implementation using vtables).

In C++ the syntax is completely different - templates vs virtual functions.
In Rust the syntax is almost identical - in some cases it's as simple as
exchanging the `impl` keyword with the `dyn` keyword.

Given this flexibility to switch strategies, which should you start with?

In both languages, monomorphization tends to result in a quicker program (partly
due to better inlining). It's arguably true that inlining is more important in
Rust, due to its functional nature and pervasive use of iterators. Whether or
not that's the reason, experienced Rustaceans usually start with `impl`:

> It's best practice to start with monomorphization and move to `dyn`... - MG

The main cost of monomorphization is larger binaries. There are cases where
large amounts of code can end up being duplicated (the marvellous [serde](https://serde.rs/)
is one).

You _can_ choose to do things the other way round:

> ... it’s workable practice to start with `dyn` and then move to `impl` when you have problems. - MG

`dyn` can be awkward, and potentially expensive in different ways:

> One thing to note about pervasive `dyn` is that because it unsizes the types it wraps, you need to box it if you want to store it by value. You end up with a good bit more allocator pressure if you try to have `dyn` field types. - AP

## `<'a>`I seem to have lots of named lifetimes. Am `<'b>`I doing something wrong?

Some say that if you have a significant number of named lifetimes, you're
overcomplicating things.

There are some scenarios where multiple named lifetimes make perfect sense - for example
if you're dealing with an arena, or major phases of a process (the Rust compiler
has `'gcx` and `'tcx` lifetimes relating to the output of certain compile phases.)

But otherwise, it may be that you've got lifetimes because you're trying _too
hard_ to avoid a copy. You may be better off simply switching to runtime
checking (e.g. `Rc`, `Arc`) or even cloning.

Are named lifetimes even a "code smell"?

> My experience has been that the extent to which they're a smell varies a good bit based on the programmer's experience level, which has led me towards increased skepticism over time. Lots of people learning Rust have experienced the pain of first not wanting to `.clone()` something, immediately putting lifetimes everywhere, and then feeling the pain of lifetime subtyping and variance. I don't think they're nearly as odorous as unsafe, for example, but treating them as a bit of a smell does I think lead to code that's easier to read for a newcomer and to refactor around the stack. - AP
