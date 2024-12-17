-- Your SQL goes here
CREATE TABLE inventory (
    uuid TEXT NOT NULL PRIMARY KEY,
    owner TEXT NOT NULL,
    money INTEGER NOT NULL
);

CREATE TABLE inventoryItems (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    itemuuid TEXT NOT NULL,
    inventoryuuid TEXT NOT NULL,
    FOREIGN KEY(itemuuid) REFERENCES item(uuid),
    FOREIGN KEY(inventoryuuid) REFERENCES inventory(uuid)
);

CREATE TABLE inventoryReaders (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    useruuid TEXT NOT NULL,
    inventoryuuid TEXT NOT NULL,
    FOREIGN KEY(useruuid) REFERENCES user(uuid),
    FOREIGN KEY(inventoryuuid) REFERENCES inventory(uuid)
);

CREATE TABLE inventoryWriters (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    useruuid TEXT NOT NULL,
    inventoryuuid TEXT NOT NULL,
    FOREIGN KEY(useruuid) REFERENCES user(uuid),
    FOREIGN KEY(inventoryuuid) REFERENCES inventory(uuid)
);

CREATE TABLE item (
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    presetReference TEXT NOT NULL,
    amount INTEGER NOT NULL,
    dmNote TEXT NOT NULL
);

CREATE TABLE itempreset(
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    price INTEGER NOT NULL,
    text TEXT NOT NULL,
    creator TEXT NOT NULL,
    itemType TEXT NOT Null
);

CREATE TABLE user(
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);