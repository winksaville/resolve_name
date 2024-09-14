# Resolve a name to an IP address

## Running the code

```bash
$ cargo run google.com
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/resolve_name google.com`
Resolved address: 142.250.191.78:0
Resolved address: [2607:f8b0:4005:80e::200e]:0
```

## Installing the code

```bash
$ cargo install --path .
  Installing resolve_name v0.1.0 (/home/wink/prgs/rust/resolve_name)
   Compiling resolve_name v0.1.0 (/home/wink/prgs/rust/resolve_name)
    Finished `release` profile [optimized] target(s) in 0.15s
  Installing /home/wink/.cargo/bin/resolve_name
   Installed package `resolve_name v0.1.0 (/home/wink/prgs/rust/resolve_name)` (executable `resolve_name`)
```

## Running the installed code

```bash
$ resolve_name google.com
Resolved address: 172.217.164.110:0
Resolved address: [2607:f8b0:4005:80c::200e]:0
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
