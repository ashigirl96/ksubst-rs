# ksubst

[![crates.io](https://img.shields.io/crates/v/ksubst.svg)](https://crates.io/crates/ksubst)
[![Documentation](https://docs.rs/ksubst/badge.svg)](https://docs.rs/ksubst)

A simple Rust library for variable substitution.

This library provides helper functions for string manipulation,
taking values from a context **env**ironment map and **subst**ituting
all matching placeholders.

Its name and logic are similar to the [`ksubst`] GNU utility, but this supports braces-delimited variables (i.e., `${foo}`, `${foo.}`, `${foo-}`) and takes replacement values from an explicit map of variables.


[`ksubst`]: https://www.gnu.org/software/gettext/manual/html_node/ksubst-Invocation.html

## Fork Acknowledgment

This project is a fork of [ksubst-rs](https://github.com/ashigirl96/ksubst-rs). We greatly respect and appreciate the original work done by the `ksubst-rs` maintainers.

## Example

You can run an example to see how the library works by executing:

```bash
cargo run --example basic
```

This will produce the following output:

```
VAR=hoge VAR.=hoge. VAR-=hoge-
VAR2= VAR2.= VAR-=
```

### Example Code

Here's the code used in the example:

```rust
fn main() {
    let template = "VAR=${VAR} VAR.=${VAR.} VAR-=${VAR-}\nVAR2=${VAR2} VAR2.=${VAR2.} VAR-=${VAR2-}";
    let mut variables = HashMap::new();
    variables.insert("VAR".to_string(), "hoge".to_string());
    variables.insert("VAR2".to_string(), "".to_string());

    let result = substitute(template, &variables).unwrap();
    println!("{}", result);
}
```

This example demonstrates how to substitute variables in a template string using values from a context map.

### Command Line

ksubst is a tool that provides a command-line interface to replace variables in text files or streams. It retrieves values from environment variables or a specified .env file, allowing placeholders to be replaced with actual values.


```shell
> cat assets/foo.yaml  | ksubst -e env.assets

...skip...
metadata:
  name: hoge-mc
spec:
  domains:
    - hoge.example.com
...skip...
```

```shell
> ksubst --env-file env.assets -r assets assets2
> diff -u <(find assets -type f -exec cat {} +)  <(find assets2 -type f -exec cat {} +)

...skip...
 metadata:
-  name: ${FEATURE-}mc
+  name: hoge-mc
...skip...
       labels:
-        app: ${FEATURE-}name
+        app: hoge-name
```

or `ksubst -r assets assets2 --env-vars 'FEATURE=hoge2,VERSION=123'`

## License

Licensed under either of

* MIT license - <http://opensource.org/licenses/MIT>
* Apache License, Version 2.0 - <http://www.apache.org/licenses/LICENSE-2.0>

at your option.
