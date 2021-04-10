# `AnyMap`, a safe and convenient store for one value of each type

[![Build Status](https://travis-ci.org/chris-morgan/anymap.svg?branch=master)](https://travis-ci.org/chris-morgan/anymap)

`AnyMap` is a wrapper around a `HashMap<TypeId, Box<Any>>`, exposing a typed interface which is safe and robust.

This means in an `AnyMap` you may store zero or one value for every type.

## Instructions

```toml
anymap = "0.13.0"
```

## `unsafe` Code

This library uses a fair bit of unsafe code for several reasons:

- To support `Any` and `CloneAny`, `unsafe` code is required (because of how the `downcast` methods are defined in `impl Any` rather than being trait methods; I think this is kind of a historical detail of the structure of `std::any::Any`); if you wanted to ditch `Clone` support this unsafety could be removed.

- In the interests of performance, skipping various checks that are unnecessary because of the invariants of the data structure (no need to check the type ID when it’s been statically ensured by being used as the hash map key) and simplifying hashing (type IDs are already good hashes, no need to mangle them through SipHash).

It’s not possible to remove all unsafety from this library without also removing some of the functionality. Still, at the cost of the `CloneAny` functionality, the raw interface and maybe the concurrency support, you can definitely remove all `unsafe` code. Here’s how you could do it:

- Remove the genericness of it all;
- Merge `anymap::raw` into the normal interface, flattening it;
- Change things like `.map(|any| unsafe { any.downcast_unchecked() })` to `.and_then(|any| any.downcast())` (performance cost: one extra superfluous type ID comparison, indirect);
- Ditch the `TypeIdHasher` since transmuting a `TypeId` is right out (cost: SIP mangling of a u64 on every access).

Yeah, the performance costs of going safe are quite small. The more serious matters are the loss of `Clone` and maybe `Send + Sync`.

But frankly, if you wanted to do all this it’d be easier and faster to write it from scratch. The core of the library is actually really simple and perfectly safe, as can be seen in [`src/lib.rs` in the first commit](https://github.com/chris-morgan/anymap/tree/a294948f57dee47bb284d6a3ae1b8f61a902a03c/src/lib.rs) (note that that code won’t run without a few syntactic alterations; it was from well before Rust 1.0 and has things like `Any:'static` where now we have `Any + 'static`).


## Author

[Chris Morgan](http://chrismorgan.info/) ([chris-morgan](https://github.com/chris-morgan)) is the primary author and maintainer of AnyMap.


## License

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
