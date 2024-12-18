// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (uuid) {
        uuid -> Text,
        owner -> Text,
        money -> Integer,
    }
}

diesel::table! {
    inventoryItems (id) {
        id -> Integer,
        itemuuid -> Text,
        inventoryuuid -> Text,
    }
}

diesel::table! {
    inventoryReaders (id) {
        id -> Integer,
        useruuid -> Text,
        inventoryuuid -> Text,
    }
}

diesel::table! {
    inventoryWriters (id) {
        id -> Integer,
        useruuid -> Text,
        inventoryuuid -> Text,
    }
}

diesel::table! {
    item (uuid) {
        uuid -> Text,
        name -> Text,
        presetReference -> Text,
        amount -> Integer,
        dmNote -> Text,
    }
}

diesel::table! {
    itempreset (uuid) {
        uuid -> Text,
        name -> Text,
        price -> Integer,
        text -> Text,
        creator -> Text,
        itemType -> Text,
    }
}

diesel::table! {
    user (uuid) {
        uuid -> Text,
        name -> Text,
    }
}

diesel::joinable!(inventoryItems -> inventory (inventoryuuid));
diesel::joinable!(inventoryItems -> item (itemuuid));
diesel::joinable!(inventoryReaders -> inventory (inventoryuuid));
diesel::joinable!(inventoryReaders -> user (useruuid));
diesel::joinable!(inventoryWriters -> inventory (inventoryuuid));
diesel::joinable!(inventoryWriters -> user (useruuid));

diesel::allow_tables_to_appear_in_same_query!(
    inventory,
    inventoryItems,
    inventoryReaders,
    inventoryWriters,
    item,
    itempreset,
    user,
);
