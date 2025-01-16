-- Your SQL goes here
CREATE TABLE user (
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    dm INTEGER NOT NULL
);

CREATE TABLE item_preset (
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    price INTEGER NOT NULL,
    weight REAL NOT NULL,
    description TEXT NOT NULL,
    creator TEXT NOT NULL,
    item_type TEXT NOT Null
);

CREATE TABLE inventory (
    uuid TEXT NOT NULL PRIMARY KEY,
    owner_uuid TEXT NOT NULL,
    money INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (owner_uuid) REFERENCES user(uuid) ON DELETE CASCADE
);

CREATE TABLE inventory_reader (
    user_uuid TEXT NOT NULL,
    inventory_uuid TEXT NOT NULL,
    FOREIGN KEY(user_uuid) REFERENCES user(uuid) ON DELETE CASCADE,
    FOREIGN KEY(inventory_uuid) REFERENCES inventory(uuid) ON DELETE CASCADE,
    PRIMARY KEY(user_uuid, inventory_uuid)
);

CREATE TABLE inventory_writer (
    user_uuid TEXT NOT NULL,
    inventory_uuid TEXT NOT NULL,
    FOREIGN KEY(user_uuid) REFERENCES user(uuid) ON DELETE CASCADE,
    FOREIGN KEY(inventory_uuid) REFERENCES inventory(uuid) ON DELETE CASCADE,
    PRIMARY KEY(user_uuid, inventory_uuid)
);


CREATE TABLE inventory_item(
    inventory_uuid TEXT NOT NULL,
    item_preset_uuid TEXT NOT NULL,
    dm_note TEXT NOT NULL,
    amount INTEGER NOT NULL,
    sorting INTEGER NOT NULL,
    inventory_item_note TEXT NOT NULL,
    PRIMARY KEY(inventory_uuid, item_preset_uuid),
    FOREIGN KEY(inventory_uuid) REFERENCES inventory(uuid) ON DELETE CASCADE,
    FOREIGN KEY(item_preset_uuid) REFERENCES item_preset(uuid) ON DELETE CASCADE
);
