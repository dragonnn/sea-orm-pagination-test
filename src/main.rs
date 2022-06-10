use sea_orm::prelude::*;
use sea_orm::PaginatorTrait;
use std::marker::PhantomData;

pub struct TableRequestPrepared<'a, T: EntityTrait> {
    phantom: PhantomData<&'a T>,
}

impl<'a, T: EntityTrait> TableRequestPrepared<'a, T> {
    pub fn new(orm: &'a DatabaseConnection, select: Select<T>) {
        select.paginate(orm, 20);
    }
}

fn main() {
    println!("Hello, world!");
}
