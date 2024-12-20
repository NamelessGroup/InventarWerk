-- Your SQL goes here
CREATE TABLE Inventory (
    uuid TEXT NOT NULL PRIMARY KEY,
    owner_uuid TEXT NOT NULL,
    money INTEGER NOT NULL,
    name TEXT NOT NULL,
    FOREIGN KEY (owner_uuid) REFERENCES User(uuid)
);

CREATE TABLE InventoryReader (
    user_uuid TEXT NOT NULL,
    inventory_uuid TEXT NOT NULL,
    FOREIGN KEY(user_uuid) REFERENCES User(uuid),
    FOREIGN KEY(inventory_uuid) REFERENCES Inventory(uuid),
    PRIMARY KEY(user_uuid, inventory_uuid)
);

CREATE TABLE InventoryWriter (
    user_uuid TEXT NOT NULL,
    inventory_uuid TEXT NOT NULL,
    FOREIGN KEY(useruser_uuiduuid) REFERENCES User(uuid),
    FOREIGN KEY(inventory_uuid) REFERENCES Inventory(uuid),
    PRIMARY KEY(user_uuid, inventory_uuid)
);


CREATE TABLE ItemPreset (
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    price INTEGER NOT NULL,
    description TEXT NOT NULL,
    creator TEXT NOT NULL,
    item_type TEXT NOT Null
);

CREATE TABLE User (
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    dm INTEGER NOT NULL
);

CREATE TABLE InventoryItem(
    inventory_uuid TEXT NOT NULL,
    item_preset_uuid TEXT NOT NULL,
    dm_note TEXT NOT NULL,
    amount INTEGER NOT NULL,
    PRIMARY KEY(inventory_uuid, item_preset_uuid),
    FOREIGN KEY(inventory_uuid) REFERENCES Inventory(uuid),
    FOREIGN KEY(item_preset_uuid) REFERENCES ItemPreset(uuid)
)
