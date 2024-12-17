// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (uuid) {
        uuid -> Text,
        owneruuid -> Text,
        money -> Integer,
        name -> Text,
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
        inventoryuuid -> Text,
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
        dm -> Integer,
    }
}

diesel::joinable!(inventory -> user (owneruuid));
diesel::joinable!(inventoryReaders -> inventory (inventoryuuid));
diesel::joinable!(inventoryReaders -> user (useruuid));
diesel::joinable!(inventoryWriters -> inventory (inventoryuuid));
diesel::joinable!(inventoryWriters -> user (useruuid));
diesel::joinable!(item -> inventory (inventoryuuid));
diesel::joinable!(item -> itempreset (presetReference));

diesel::allow_tables_to_appear_in_same_query!(
    inventory,
    inventoryReaders,
    inventoryWriters,
    item,
    itempreset,
    user,
);
