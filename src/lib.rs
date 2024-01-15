use tokio_postgres::Row;

pub use from_pg_fit_row_impl::*;


/// Beware, check your SQL for correctness! Fields should be in the exact correct order 
pub trait FromPgFitRow {
    fn from_fit_pg_row(row: Row) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(FromPgFitRow)]
    struct Foo {
        a: String,
        b: i64,
        c: Option<i16>,
        d: Option<String>,
    }

    #[test]
    fn compiles() {}
}
