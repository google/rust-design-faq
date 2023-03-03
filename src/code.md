# Questions about code in function bodies

## How can I avoid the performance penalty of bounds checks?

Rust array and list accesses are all bounds checked. You may be worried about a performance penalty. How can you avoid that?

> Contort yourself a little bit to use iterators. - MY

Rust gives you choices around functional versus imperative style, but things often work better in a functional style. Specifically - if you've got something iterable, then there are probably functional methods to do what you want.

For instance, suppose you need to work out what food to get at the petshop. Here's code that does this in an imperative style:

```rust
{{#include pets.rs}}
fn make_shopping_list_a() -> HashSet<&'static str> {
    let mut meals_needed = HashSet::new();
    for n in 0..PETS.len() { // ugh
        if PETS[n].is_hungry {
            meals_needed.insert(PETS[n].meal_needed);
        }
    }
    meals_needed
}
```

The loop index is verbose and error-prone. Let's get rid of it and loop over an iterator instead:

```rust
{{#include pets.rs}}
fn make_shopping_list_b() -> HashSet<&'static str>  {
    let mut meals_needed = HashSet::new();
    for animal in PETS.iter() { // better...
        if animal.is_hungry {
            meals_needed.insert(animal.meal_needed);
        }
    }
    meals_needed
}
```

We're accessing the loop through an iterator, but we're still processing the elements inside a loop. It's often more idiomatic to replace the loop with a chain of iterators:

```rust
{{#include pets.rs}}
fn make_shopping_list_c() -> HashSet<&'static str> {
    PETS.iter()
        .filter(|animal| animal.is_hungry)
        .map(|animal| animal.meal_needed)
        .collect() // best...
}
```

The obvious advantage of the third approach is that it's more concise, but less obviously:

* The first solution may require Rust to do array bounds checks inside each iteration of the loop, making Rust potentially slower than C++. In this sort of simple example, it likely wouldn't, but functional pipelines simply don't require bounds checks.
* The final container (a `HashSet` in this case) may be able to allocate roughly the right size at the outset, using the [size_hint](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) of a Rust iterator.
* If you use iterator-style code rather than imperative code, it's more likely the Rust compiler will be able to [auto-vectorize using SIMD instructions](https://medium.com/swlh/an-adventure-in-simd-b0e8db4ccca7).
* There is no mutable state within the function. This makes it easier to verify that the code is correct and to avoid introducing bugs when changing it. In this simple example it may be obvious that calling the `HashSet::insert` is the only mutation to the set, but in more complex scenarios it is quite easy to lose the overview.
* And as a new arrival from C++, you may find this hard to believe: For an experienced Rustacean it'll be more readable.

Here are some more iterator techniques to help avoid materializing a collection:

* You can [chain two iterators together](https://doc.rust-lang.org/std/iter/struct.Chain.html) to make a longer one.
* If you need to iterate two lists, [zip them together](https://doc.rust-lang.org/std/iter/struct.Zip.html) to avoid bounds checks on either.
* If you want to feed all your animals, and also feed a nearby duck, just chain the iterator to `std::iter::once`:

  ```rust
  # use std::collections::HashSet;
  # struct Animal {
  #     kind: &'static str,
  #     is_hungry: bool,
  #     meal_needed: &'static str,
  # }
  # static PETS: [Animal; 0] = [];
  #  static NEARBY_DUCK: Animal = Animal {
  #         kind: "Duck",
  #         is_hungry: true,
  #         meal_needed: "pondweed",
  #     };
  fn make_shopping_list_d() -> HashSet<&'static str> {
      PETS.iter()
          .chain(std::iter::once(&NEARBY_DUCK))
          .filter(|animal| animal.is_hungry)
          .map(|animal| animal.meal_needed)
          .collect()
  }
  ```
  (Similarly, if you want to add one more item to the shopping list - maybe you're hungry, as well as your menagerie? - just add it after the `map`).
* `Option` is iterable.
  ```rust
  # use std::collections::HashSet;
  # struct Animal {
  #     kind: &'static str,
  #     is_hungry: bool,
  #     meal_needed: &'static str,
  # }
  # static PETS: [Animal; 0] = [];
  # struct Pond;
  # static MY_POND: Pond = Pond;
  fn pond_inhabitant(pond: &Pond) -> Option<&Animal> {
      // ...
  #    None
  }

  fn make_shopping_list_e() -> HashSet<&'static str> {
      PETS.iter()
          .chain(pond_inhabitant(&MY_POND))
          .filter(|animal| animal.is_hungry)
          .map(|animal| animal.meal_needed)
          .collect()
  }
  ```

  Here's a diagram showing how data flows in this iterator pipeline:

  ```mermaid
  flowchart LR
    %%{ init: { 'flowchart': { 'nodeSpacing': 40, 'rankSpacing': 15 } } }%%
      Pets
      Filter([filter by hunger])
      Map([map to noms])
      Meals
      uniqueify([uniqueify])
      shopping[Shopping list]
      Pets ---> Filter
      Pond
      Pond ---> inhabitant
      inhabitant[Optional pond inhabitant]
      inhabitant ---> Map
      Filter ---> Map
      Map ---> Meals
      Meals ---> uniqueify
      uniqueify ---> shopping
  ```

* Here are other iterator APIs that will come in useful:
    * [cloned](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)
    * [flatten](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)

C++20 recently introduced [ranges](https://en.cppreference.com/w/cpp/ranges), a feature that allows you to pipeline operations on a collection similar to the way Rust iterators do, so this style of programming is likely to become more common in C++ too.

To summarize: While in C++ you tend to operate on collections by performing a series of operations on each individual item, in Rust you'll typically apply a pipeline of operations to the whole collection. Make this mental switch and your code will not just become more idiomatic but more efficient, too.

## Isn't it confusing to use the same variable name twice?

In Rust, it's common to reuse the same name for multiple variables in a function. For a C++ programmer, this is weird, but there are two good reasons to do it:

* You may no longer need to change a mutable variable after a certain point, and if your code is sufficiently complex you might want the compiler to guarantee this for you:

    ```rust
    # fn spot_ate_my_slippers() -> bool {
    #     false
    # }
    # fn feed(_: &str) {}
    let mut good_boy = "Spot";
    if spot_ate_my_slippers() {
        good_boy = "Rover";
    }
    let good_boy = good_boy; // never going to change my dog again, who's a good boy
    feed(&good_boy);
    ```

* Another common pattern is to retain the same variable name as you gradually unwrap things to a simpler type:

    ```rust
    # let url = "http://foo.com:1234";
    let port_number = url.split(":").skip(2).next().unwrap();
        // hmm, maybe somebody else already wrote a better URL parser....? naah, probably not
    let port_number = port_number.parse::<u16>().unwrap();
    ```

## How can I avoid the performance penalty of `unwrap()`?

C++ has no equivalent to Rust's `match`, so programmers coming from C++ often underuse it.

A heuristic: if you find yourself `unwrap()`ing, _especially_ in an `if`/`else` statement, you should restructure your code to use a more sophisticated `match`.

For example, note the `unwrap()` in here (implying some runtime branch):

```rust
# fn test_parse() -> Result<u64,std::num::ParseIntError> {
# let s = "0x64a";
if s.starts_with("0x") {
    u64::from_str_radix(s.strip_prefix("0x").unwrap(), 16)
} else {
    s.parse::<u64>()
}
# }
```

and no extra `unwrap()` here:

```rust
# fn test_parse() -> Result<u64,std::num::ParseIntError> {
# let s = "0x64a";
match s.strip_prefix("0x") {
    None => s.parse::<u64>(),
    Some(remainder) => u64::from_str_radix(remainder, 16),
}
# }
```

`if let` and `matches!` are just as good as `match` but sometimes a little more concise. `cargo clippy` will usually tell you if you're using a `match` which can be simplified to one of those other two constructions.

## How do I access variables from within a spawned thread?

Use [`std::thread::scope`](https://doc.rust-lang.org/nightly/std/thread/fn.scope.html).

## When should I use runtime checks vs jumping through hoops to do static checks?

Everyone learns Rust a different way, but it's said that some people reach a
point of "trait mania" where they try to encode _too much_ via the type
system, and get in a mess. So, in learning Rust, you will want to strike a
balance between runtime checks (easy) or static compile-time checks (more
efficient but requires deeper understanding.)

> It’s very personal - some people learn better if they opt out of
> language features, others not. - MG

Some heuristics for how to keep things simple during the beginning of your
Rust journey:

* It's OK to start with lots of `.unwrap()`, cloning and `Arc`/`Rc`.
* Start to use more advanced language features when you feel annoyed with
  the amount of boilerplate. (As an expert, you'll switch to a different
  strategy which is to consider the virality of your choices through the
  codebase.)
* Don't use traits until you have to. You might (for instance) need to use
  a trait to make some code unit testable, but overoptimizing for that too
  soon is a mistake. Some say that it's wise initially to avoid defining
  any new traits at all.
* Try to keep types smaller.

Specifically on reference counting,

> If using Rc means you can avoid a lifetime parameter which is in half the
> APIs in the project, that’s a very reasonable choice. If it avoids a single
> lifetime somewhere, probably not a good idea. But measure before deciding. - MG

If you want to bail out of the complexity of static checks, which runtime checks
are OK?

* `unwrap()` and `Option` is mostly fine.
* `Arc` and `Rc` is also fine in most cases.
* Extensive use of `clone()` is fine but will have a performance impact.
* `Cell` is regarded as a code smell and suggests you don't understand your
  lifetimes - it should be used sparingly.
* `unsafe` is definitely not OK. It's harder to write `unsafe` Rust than to write
  C or C++, because Rust has additional aliasing rules. If you're reaching for
  `unsafe` to work around the complexity of Rust's static type system, as a
  relative Rust beginner, please reconsider and look into the other options
  listed above.

Doing lifetime magic — where "magic" means annotating a function or complex
type with more than 1 lifetime, or other wizardry — is often an optimization
that you can defer until later. In the beginning, and when writing small
programs that you only intend to use a few times ('scripts'), copying is fine.
