`from-pg-fit-row`

Get a struct from a `tokio-postgres` row where the columns are correctly fit, in the right order, to the fields of your struct.

```rust
use from_pg_fit_row::FromPgFitRow;

#[derive(FromPgFitRow)]
struct Foo {
    a: i64,
    b: String,
    c: Option<i32>,
}
```

The derive macro basically expands to:

```rust
impl FromPgFitRow for Foo {
    fn from_fit_pg_row(row: Row) -> Self {
        Self {
            a: row.get(0),
            b: row.get(1),
            c: row.get(2),
        }
    }
}
```

So you don't need to write it yourself. You can use it later:

```rust
let row = client.query_one("SELECT a, b, c FROM foos WHERE a = 3").await.unwrap();
let my_foo = Foo::from_pg_fit_row(row);
```

Beware though, that as this blindly uses indexes for maximum performance, it **will crash** if your SQL query fields do not match correctly (correct field order, at least as many fields as the struct). Use carefully :).

