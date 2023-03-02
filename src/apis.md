# Questions about designing APIs for others

See also the excellent [Rust API guidelines](https://rust-lang.github.io/api-guidelines/about.html).
The document you're reading aims to provide extra hints which may be especially
useful to folk coming from C++, but that's the canonical reference.

## When should my type implement `Default`?

Whenever you'd provide a default constructor in C++.

## When should my type implement `From`, `Into` and `TryFrom`?

You should think of these as equivalent to implicit conversions in C++. Just
as with C++, if there are _multiple_ ways to convert from your thing to another
thing, don't implement these, but if there's a single obvious conversion, do.

Usually, don't implement `Into` but instead implement `From`.

## How should I expose constructors?

See the previous two answers: where it's simple and obvious, use the standard
traits to make your object behavior predictable.

If you need to go beyond that, remember you've got a couple of extra toys in Rust:

* A "constructor" could return a `Result<Self>`
* Your constructors can have names, e.g. `Vec::with_capacity`, `Box::pin`

## When should my type implement `AsRef`?

If you have a type which contains another type, provide `AsRef` especially
so that people can clone the inner type. It's good practice to provide explicit
versions as well (for example, `String` implements `AsRef<str>` but also
provides `.as_str()`.)

## When should I implement `Copy`?

> Anything that is integer-like or reference-like should be `Copy`; other things
> shouldn’t. - MY

> When it's efficient and when it’s an API contact you're willing to uphold. - AH

Generally speaking, types which are plain-old-data can be `Copy`. Anything
more nuanced with any type of state shouldn't be.

## Should I have `Arc` or `Rc` in my API?

> It’s a code smell to have reference counts in your API design. You should hide
> it. - TM.

If you must, you will need to decide between `Rc` and `Arc` - see the next
answer for some considerations. Also, consider taking a look at the
[`Archery` crate](https://docs.rs/archery/latest/archery/).

## Should my API be thread-safe? What does that mean?

In C++, a thread-safe API usually means that you can expect your API's
consumers to use objects from multiple threads. This is difficult to make safe
and therefore substantial extra engineering is required to make an API
thread-safe.

In Rust, things differ:

* it's more normal to do things across multiple threads;
* you don't have to worry about your callers making mistakes here because
  the compiler won't let them;
* you can often rely on `Send` rather than `Sync`.

You certainly shouldn't be putting a `Mutex` around all your types. If your
caller attempts to use the type from multiple threads, the compiler will
simply stop them. It is the responsibility of the caller to use things
safely.

> If the library has `Arc` or `Rc` in the APIs, it may be making choices about
> how you should instantiate stuff, and that’s rude. - AF

There's a reasonable chance that your API can be used in parallel threads
by virtue of `Send` and `Sync` being automatically derived. But - you should
think through the usage model for your API clients and ensure that's true.

```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;

// Absolutely no effort required to pizza, cooking or eating procedures
// to be usable in a threaded context
mod pizza_api {

    use std::thread;
    use std::time::Duration;

    pub struct Pizza {
        // automatically 'Send'
        _anchovies: u32,
        _pepperoni: u32,
    }

    pub fn make_pizza() -> Pizza {
        println!("cooking...");
        thread::sleep(Duration::from_millis(10));
        Pizza {
            _anchovies: 0, // yuck
            _pepperoni: 32,
        }
    }

    pub fn eat_pizza(_pizza: Pizza) {
        println!("yum")
    }
}

fn main() {
    let pizza_queue = Mutex::new(RefCell::new(VecDeque::new()));
    thread::scope(|s| {
        s.spawn(|| {
            let mut pizzas_eaten = 0;
            while pizzas_eaten < 100 {
                if let Some(pizza) = pizza_queue.lock().unwrap().borrow_mut().pop_front() {
                    pizza_api::eat_pizza(pizza);
                    pizzas_eaten += 1;
                }
            }
        });
        s.spawn(|| {
            for _ in 0..100 {
                let pizza = pizza_api::make_pizza();
                pizza_queue.lock().unwrap().borrow_mut().push_back(pizza);
            }
        });
    });
}
```

## What should I `Derive` to make my code optimally usable?

The [official guidelines say to be eager](https://rust-lang.github.io/api-guidelines/interoperability.html#types-eagerly-implement-common-traits-c-common-traits).

But don't overpromise:

> Equality can suddenly become expensive later - don’t make types comparable
> unless you intend people to be able to [compare instances of the type].
> Allowing people to pattern match on enums is usually better. - MY

Note that [`syn` is a rare case](https://docs.rs/syn/latest/syn/) in that it
has so many types, and is so extensively depended upon by the rest of the Rust
ecosystem, that it avoids deriving the standard traits unless explicitly
commanded to do so via a cargo feature. This is an unusual pattern and should
not normally be followed.

## How should I think about API design, differently from C++?

> Make the most of the fact that everything is immutable by default. Things
> which are mutable should stick out. - AF

> Think about things which should take self and return self. - AF

Refactoring is less expensive in Rust than C++ due to compiler safeguards, but
_rearchitecting_ is expensive in any language. Think about "one way doors"
and "two way doors" in the design space: can you undo a change later?
