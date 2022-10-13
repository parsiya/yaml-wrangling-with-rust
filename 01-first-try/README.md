# 01-first-try

```
$ cd 01-first-try
# package name is automatically generated based on the parent direcotry but
# they cannot start with a digit so we use `--name`.
$ cargo init --name first-try
```

Modify `cargo.toml` and add the following under `[dependencies]`.

```toml
[dependencies]
serde = {version = "1.0", features = ["derive"]}
serde_yaml = "0.9"
```

Then start editing `main.rs`.