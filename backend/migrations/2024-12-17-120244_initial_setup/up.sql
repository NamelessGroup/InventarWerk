-- Your SQL goes here
CREATE TABLE inventory (
    uuid TEXT NOT NULL PRIMARY KEY,
    owneruuid TEXT NOT NULL,
    money INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (owneruuid) REFERENCES user(uuid)
);

CREATE TABLE inventoryReaders (
    useruuid TEXT NOT NULL,
    inventoryuuid TEXT NOT NULL,
    FOREIGN KEY(useruuid) REFERENCES user(uuid),
    FOREIGN KEY(inventoryuuid) REFERENCES inventory(uuid),
    PRIMARY KEY(useruuid, inventoryuuid)
);

CREATE TABLE inventoryWriters (
    useruuid TEXT NOT NULL,
    inventoryuuid TEXT NOT NULL,
    FOREIGN KEY(useruuid) REFERENCES user(uuid),
    FOREIGN KEY(inventoryuuid) REFERENCES inventory(uuid),
    PRIMARY KEY(useruuid, inventoryuuid)
);

CREATE TABLE item (
    uuid TEXT NOT NULL PRIMARY KEY,
    presetReference TEXT NOT NULL,
    amount INTEGER NOT NULL,
    description TEXT NOT NULL,
    dmNote TEXT NOT NULL,
    inventoryuuid TEXT NOT NULL,
    FOREIGN KEY(presetReference) REFERENCES itempreset(uuid),
    FOREIGN KEY(inventoryuuid) REFERENCES inventory(uuid)
);

CREATE TABLE itempreset(
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    price INTEGER NOT NULL,
    description TEXT NOT NULL,
    creator TEXT NOT NULL,
    itemType TEXT NOT Null
);

CREATE TABLE user(
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    dm INTEGER NOT NULL
);
