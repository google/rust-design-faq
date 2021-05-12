# Questions about your code

This chapter suggests how you might approach writing the code _within_ your functions.

## How can I avoid the performance penalty of bounds checks?

Rust array and list accesses are all bounds checked. You may be worried about a performance penalty. How can you avoid that?

> Contort yourself a little bit to use iterators. - MY

Rust gives you choices around functional versus imperative style, but things often work better in a functional style. Specifically - if you've got something iterable, then there are probably functional methods to do what you want.

For instance, supposing you need to work out what food to get at the petshop:

```mermaid
flowchart LR
	Pets
	Filter([filter by hunger])
	Map([map to noms])
	Meals
    uniqueify([uniqueify])
	shopping[Shopping list]
	Pets ---> Filter
	Filter ---> Map
	Map ---> Meals
	Meals ---> uniqueify
	uniqueify ---> shopping
```

```rust
# use std::collections::HashSet;
# struct Animal {
# 	kind: &'static str,
# 	is_hungry: bool,
# 	meal_needed: &'static str,
# }
#
# static PETS: [Animal; 4] = [
# 	Animal {
# 		kind: "Dog",
# 		is_hungry: true,
# 		meal_needed: "Kibble",
# 	},
# 	Animal {
# 		kind: "Python",
# 		is_hungry: false,
# 		meal_needed: "Cat",
# 	},
# 	Animal {
# 		kind: "Cat",
# 		is_hungry: true,
# 		meal_needed: "Kibble",
# 	},
# 	Animal {
# 		kind: "Lion",
# 		is_hungry: false,
# 		meal_needed: "Kibble",
# 	},
#  ];
fn make_shopping_list_a() -> HashSet<&'static str> {
	let mut meals_needed = HashSet::new();
	for n in 0..PETS.len() { // ugh
		if PETS[n].is_hungry {
			meals_needed.insert(PETS[n].meal_needed);
		}
	}
	meals_needed
}

fn make_shopping_list_b() -> HashSet<&'static str>  {
	let mut meals_needed = HashSet::new();
	for animal in PETS.iter() { // better...
		if animal.is_hungry {
			meals_needed.insert(animal.meal_needed);
		}
	}
	meals_needed
}

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
* Rust may be better able to [auto-vectorize code to use SIMD instructions](https://www.minimalrust.com/an-adventure-in-simd/).
* There is no mutable state within the function. This makes it easier to verify that the code is correct and to avoid introducing bugs when changing it. In this simple example it may be obvious that calling the `HashSet::insert` is the only mutation to the set, but in more complex scenarios it is quite easy to lose the overview.
* And as a new arrival from C++, you may find this hard to believe... but for an experienced Rustacean it'll be more readable.

There are other cases where in C++ you might materialize a collection, whereas in Rust you just don't need to:

* You can [chain two iterators together](https://doc.rust-lang.org/std/iter/struct.Chain.html) to make a longer one.
* If you need to iterate two lists, [zip them together](https://doc.rust-lang.org/std/iter/struct.Zip.html) to avoid bounds checks on either.
* If you want to feed all your animals, and also feed a nearby duck, just chain the iterator to `std::iter::once`:

  ```rust
  # use std::collections::HashSet;
  # struct Animal {
  # 	kind: &'static str,
  # 	is_hungry: bool,
  # 	meal_needed: &'static str,
  # }
  # static PETS: [Animal; 0] = [];
  #  static NEARBY_DUCK: Animal = 	Animal {
  # 		kind: "Duck",
  # 		is_hungry: true,
  # 		meal_needed: "pondweed",
  # 	};
  fn make_shopping_list_d() -> HashSet<&'static str> {
	  PETS.iter()
	  	.filter(|animal| animal.is_hungry)
	  	.chain(std::iter::once(&NEARBY_DUCK))
	  	.map(|animal| animal.meal_needed)
	  	.collect()
  } 
  ```
  (Similarly, if you want to add one more item to the shopping list - maybe you're hungry, as well as your menagerie? - just add it after the `map`).
* `Option` is iterable.
  ```rust
  # use std::collections::HashSet;
  # struct Animal {
  # 	kind: &'static str,
  # 	is_hungry: bool,
  # 	meal_needed: &'static str,
  # }
  # static PETS: [Animal; 0] = [];
  # struct Pond;
  # static MY_POND: Pond = Pond;
  # fn pond_is_inhabited(pond: &Pond) -> Option<&Animal> {
  # 	// ...
  #    None
  # }
  fn make_shopping_list_d() -> HashSet<&'static str> {
  	PETS.iter()
  		.filter(|animal| animal.is_hungry)
  		.chain(pond_is_inhabited(&MY_POND))
  		.map(|animal| animal.meal_needed)
  		.collect()
  }
  ```

  ```mermaid
  flowchart LR
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

A list of other iterator-related APIs to become comfortable with:
* [cloned](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)
* [flatten](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)

## Isn't it confusing to use the same variable name twice?

For a C++ programmer, it's weird that you can use the same variable name again. There are two good reasons to do this. First, you might decide you no longer need a change a variable again, and if your code is sufficiently complex you can ask the compiler to guarantee this for you:

```rust
# fn spot_ate_my_slippers() -> bool {
# 	false
# }
# fn feed(_: &str) {}
let mut good_boy = "Spot";
if spot_ate_my_slippers() {
	good_boy = "Rover";
}
let good_boy = good_boy; // never going to change my dog again, who's a good boy
feed(&good_boy);
```

The second common pattern is to retain the same variable name as you gradually unwrap things to a simpler type.

```rust
# let url = "http://foo.com:1234";
let port_number = url.split(":").skip(2).next().unwrap();
	// hmm, maybe somebody else already wrote a better URL parser....? naah, probably not
let port_number = port_number.parse::<u16>().unwrap();
```

## How can I avoid the performance penalty of `unwrap()`?

Programmers arriving from the `match`-less wastelands of C++ often underuse `match`.

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

(`if let` and `matches!` are just as good as `match` but sometimes a little more concise. `cargo clippy` will usually tell you if you're using a `match` which can be simplified to one of those other two constructions.)