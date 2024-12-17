use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::inventory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Inventory {
    pub uuid: string,
    pub owner: String,
    pub money: i32,
    pub reader: string[],
    pub writer: string[],
    pub items: string[]
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::inventory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    name: string,
    uuid: string,
    presetReference: string,
    amount: number,
    dmNote: string
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::inventory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ItemPreset {
    uuid: string,
    name: string,
    price: number,
    text: string,
    creator: string,
    itemType: string,
}