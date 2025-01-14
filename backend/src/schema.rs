// @generated automatically by Diesel CLI.

diesel::table! {
    inventory (uuid) {
        uuid -> Text,
        owner_uuid -> Text,
        money -> Integer,
        name -> Text,
    }
}

diesel::table! {
    inventory_item (inventory_uuid, item_preset_uuid) {
        inventory_uuid -> Text,
        item_preset_uuid -> Text,
        dm_note -> Text,
        amount -> Integer,
        weight -> Integer,
        sorting -> Integer,
        inventory_item_note -> Text,
    }
}

diesel::table! {
    inventory_reader (user_uuid, inventory_uuid) {
        user_uuid -> Text,
        inventory_uuid -> Text,
    }
}

diesel::table! {
    inventory_writer (user_uuid, inventory_uuid) {
        user_uuid -> Text,
        inventory_uuid -> Text,
    }
}

diesel::table! {
    item_preset (uuid) {
        uuid -> Text,
        name -> Text,
        price -> Integer,
        description -> Text,
        creator -> Text,
        item_type -> Text,
    }
}

diesel::table! {
    user (uuid) {
        uuid -> Text,
        name -> Text,
        dm -> Integer,
    }
}

diesel::joinable!(inventory -> user (owner_uuid));
diesel::joinable!(inventory_item -> inventory (inventory_uuid));
diesel::joinable!(inventory_item -> item_preset (item_preset_uuid));
diesel::joinable!(inventory_reader -> inventory (inventory_uuid));
diesel::joinable!(inventory_reader -> user (user_uuid));
diesel::joinable!(inventory_writer -> inventory (inventory_uuid));
diesel::joinable!(inventory_writer -> user (user_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    inventory,
    inventory_item,
    inventory_reader,
    inventory_writer,
    item_preset,
    user,
);
