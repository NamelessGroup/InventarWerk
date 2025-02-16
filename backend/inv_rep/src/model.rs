use chrono::NaiveDateTime;

pub struct User {
    pub uuid: String,
    pub name: String,
    pub avatar: String,
    pub dm: i32,
    pub creation: Option<NaiveDateTime>
}

pub struct FullInventory {
    pub uuid: String,
    pub owner_uuid: String,
    pub money: i32,
    pub name: String,
    pub reader: Vec<String>,
    pub writer: Vec<String>,
    pub creation: Option<NaiveDateTime>
}

pub struct RawInventory {
    pub uuid: String,
    pub owner_uuid: String,
    pub money: i32,
    pub name: String,
    pub creation: Option<NaiveDateTime>
}

pub struct InventoryReader {
    pub user_uuid: String,
    pub inventory_uuid: String,
    pub creation: Option<NaiveDateTime>
}

pub struct InventoryWriter {
    pub user_uuid: String,
    pub inventory_uuid: String,
    pub creation: Option<NaiveDateTime>
}

pub struct ItemPreset {
    pub uuid: String,
    pub name: String,
    pub price: i32,
    pub weight: f32,
    pub description: String,
    pub creator: String,
    pub item_type: String,
    pub creation: Option<NaiveDateTime>
}

pub struct InventoryItem {
    pub inventory_uuid: String,
    pub item_preset_uuid: String,
    pub dm_note: String,
    pub amount: i32,
    pub sorting: i32,
    pub inventory_item_note: String,
    pub creation: Option<NaiveDateTime>
}
