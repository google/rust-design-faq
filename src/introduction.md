# Introduction

So, you're coming from C++ and want to write Rust? Great!

You have questions? We have answers.

This book is a collection of frequently asked questions for those arriving from existing C++ codebases. It guides you on how to adapt your C++ thinking to the new facilities available in Rust. It should help you if you're coming from other object-oriented languages such as Java too.

Although it's structured as questions and answers, it can also be read front-to-back, to give you hints about how to adapt your C++/Java thinking to a more idiomatically Rusty approach.

It does not aim to teach you Rust - there are [many better resources](https://www.rust-lang.org/learn). It doesn't aim to talk about Rust idioms _in general_ - [there are great existing guides for that](https://rust-unofficial.github.io/patterns/idioms/index.html). This guide is specifically about transitioning from some other traditionally OO language. If you're coming from such a language, you'll have questions about how to achieve the same outcomes in idiomatic Rust. That's what this guide is for.

# Structure

The guide starts with idioms at the small scale - answering questions about how you'd write a few lines of code - and moves towards ever larger patterns - answering questions about how you'd structure your whole codebase.

# Contributors

The following awesome people helped write the answers here, and they're sometimes quoted using the abbreviations given.

Thanks to Adam Perry[(@\_\_anp\_\_)](https://twitter.com/__anp__) (AP), Alyssa Haroldsen [(@kupiakos)](https://twitter.com/kupiakos) (AH), Augie Fackler [(@durin42)](https://twitter.com/durin42) (AF), David Tolnay [(@davidtolnay)](https://twitter.com/davidtolnay) (DT), Łukasz Anforowicz (LA), Manish Goregaokar [(@ManishEarth)](https://twitter.com/ManishEarth) (MG), Mike Forster (MF), Miguel Young de la Sota [(@DrawsMiguel)](https://twitter.com/DrawsMiguel) (MY), and Tyler Mandry [(@tmandry)](https://twitter.com/tmandry) (TM).

Their views have been edited and collated by Adrian Taylor [(@adehohum)](https://twitter.com/adehohum), Chris Palmer, [danakj@chromium.org](mailto:danakj@chromium.org) and Martin Brænne. Any errors or misrepresentations are ours.

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
