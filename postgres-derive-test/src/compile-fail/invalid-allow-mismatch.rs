use postgres_types::{FromSql, ToSql};

#[derive(ToSql, Debug)]
#[postgres(allow_mismatch)]
struct ToSqlAllowMismatchStruct {
    a: i32,
}

#[derive(FromSql, Debug)]
#[postgres(allow_mismatch)]
struct FromSqlAllowMismatchStruct {
    a: i32,
}

#[derive(ToSql, Debug)]
#[postgres(allow_mismatch)]
struct ToSqlAllowMismatchTupleStruct(i32, i32);

#[derive(FromSql, Debug)]
#[postgres(allow_mismatch)]
struct FromSqlAllowMismatchTupleStruct(i32, i32);

fn main() {}
