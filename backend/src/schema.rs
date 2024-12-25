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
    inventoryItems (id) {
        id -> Integer,
        itemuuid -> Text,
        inventoryuuid -> Text,
    }
}

diesel::table! {
    inventoryReaders (useruuid, inventoryuuid) {
        useruuid -> Text,
        inventoryuuid -> Text,
    }
}

diesel::table! {
    inventoryWriters (useruuid, inventoryuuid) {
        useruuid -> Text,
        inventoryuuid -> Text,
    }
}

diesel::table! {
    item (uuid) {
        uuid -> Text,
        presetReference -> Text,
        amount -> Integer,
        description -> Text,
        dmNote -> Text,
        inventoryuuid -> Text,
    }
}

diesel::table! {
    itempreset (uuid) {
        uuid -> Text,
        name -> Text,
        price -> Integer,
        description -> Text,
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
diesel::joinable!(inventoryItems -> inventory (inventoryuuid));
diesel::joinable!(inventoryItems -> item (itemuuid));
diesel::joinable!(inventoryReaders -> inventory (inventoryuuid));
diesel::joinable!(inventoryReaders -> user (useruuid));
diesel::joinable!(inventoryWriters -> inventory (inventoryuuid));
diesel::joinable!(inventoryWriters -> user (useruuid));
diesel::joinable!(item -> inventory (inventoryuuid));
diesel::joinable!(item -> itempreset (presetReference));

diesel::allow_tables_to_appear_in_same_query!(
    inventory,
    inventoryItems,
    inventoryReaders,
    inventoryWriters,
    item,
    itempreset,
    user,
);
