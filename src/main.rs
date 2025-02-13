#[macro_use]
extern crate diesel;

use diesel::prelude::*;

table! {
    my_tables (id) {
        id -> Int4,
        ip_address -> Nullable<Inet>,
    }
}

#[derive(Insertable, Debug)]
pub struct MyTable {
    pub id: i32,
    pub ip_address: Option<ipnetwork::IpNetwork>,
}

fn main() {
    println!("Hello, world!");
}
