# concord-server

## Developing

```bash
cargo run
```

## Building

```bash
cargo build --release
```

#### Sqlx-cli
used to manage migrations
```rs
cargo install sqlx-cli --no-default-features --features rustls,sqlite
```

## migrations
create new migration
```sh
sqlx migrate add -r <name>
```

migrations are applied automatically when the project is run, however, if you want to apply them manually, it would be like this

```sh
sqlx migrate run
cargo sqlx prepare
```

Dual-licensed under [MIT](../LICENSE-MIT) or the [UNLICENSE](../UNLICENSE).