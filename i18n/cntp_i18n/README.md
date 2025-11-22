# Contemporary i18n

Contemporary i18n is an ergonomic i18n library that keeps your master translation file up to date as you code.

```rust
println!(tr!("HELLO_WORLD", "Hello World!"));
```

It's as easy as that!

## Setup

### Installation

Currently, Contemporary i18n is not published on crates.io, so you'll need to add it as a git dependency. In your
Cargo.toml:

```toml
[dependencies]
cntp_i18n = { git = "https://github.com/vicr123/contemporary-rs" }
```

In order to allow cntp-i18n to also generate your translation files automatically, you will also need to add
`cntp_i18n_gen` as a build dependency

```toml
[build-dependencies]
cntp_i18n_gen = { git = "https://github.com/vicr123/contemporary-rs" }
```

### Project Setup

To configure generation to run on every build, create a `build.rs` file next to your Cargo.toml (if you don't already
have one) and add the following to it:

```rust
use std::{env, path::PathBuf};

fn main() {
    let path: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR is not set")
        .into();

    cntp_i18n_gen::generate_default(&path);

    println!("cargo::rerun-if-changed=Contemporary.toml");
}
```

And finally, configure your translations to be loaded when your application starts up:

```rust
use cntp_i18n::{I18N_MANAGER, tr_load};

fn main() {
    I18N_MANAGER.write().unwrap().load_source(tr_load!());
}
```

Now, you can use the `tr!` macro to mark translatable text.

## Working with Contemporary i18n

### Marking text as translatable

Every human-readable string in your application should be wrapped in a call to `tr!`. For example:

```rust
let output = tr!("HELLO_WORLD", "Hello World!");
println!("{output}");
```

### Translation Files

When generation is run, files are placed in the `translations` directory next to your Cargo.toml:

| File        | Description                                                                                 |
|-------------|---------------------------------------------------------------------------------------------|
| `meta.json` | Contains metadata about each translation string                                             |
| `en.json`   | Contains a mapping between each translation key and the string for the language in question |

A translation file looks like this:

```json
{
  "HELLO_WORLD": "Hello World!"
}
```

To add a new language to your project, simply drop a file named with the ISO-639 language code into the `translations`
directory. If you want the translation file to be region specific, specify the language and then the region separated
by a dash. The following table lists examples of valid files:

| File              | Translation for                               |
|-------------------|-----------------------------------------------|
| `en.json`         | English - Region Agnostic                     |
| `fr.json`         | French - Region Agnostic                      |
| `pt-BR.json`      | Portuguese - Brazil                           |
| `sr-Latn.json`    | Serbian - Latin Script                        |
| `zh-Hant-CN.json` | Mandarin - Traditional Chinese Script - China |

### Substitution

When you need to leave a placeholder for some text, you can insert a `{{substitution}}` into the source text.
You need to specify the text to be substituted with after the source text.

```rust
let name = "Victor";
let city = "Sydney";
tr!(
    "MY_NAME",
    "My name is {{name}}, and I live in {{city}}.",
    name=name,
    city=city
);

// Output: My name is Victor, and I live in Sydney.
```

### Pluralisation

To mark a string as requiring a pluralised translation, use the `trn!` macro in place of the `tr!` macro:

```rust
let apples_eaten = 2;
trn!(
    "APPLES_EATEN",
    "You ate {{count}} apple today",
    "You ate {{count}} apples today.",
    count=apples_eaten
);

// Output: You ate 2 apples today.
```

The corresponding JSON file generated for this is:

```json
{
  "APPLES_EATEN": {
    "one": "You ate {{count}} apple today.",
    "other": "You ate {{count}} apples today."
  }
}
```

## Optional Features

If you so desire, you can turn on the following
optional features:

| Feature | Description                                                                                                             |
|---------|-------------------------------------------------------------------------------------------------------------------------|
| `gpui`  | Adds support for automatically converting looked-up vales to a GPUI `SharedString` for convenience when using with GPUI |

