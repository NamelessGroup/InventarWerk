-- Add migration script here

CREATE TABLE spell_preset {
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    level INTEGER NOT NULL,
    description TEXT NOT NULL,
    time TEXT NOT NULL,
    concentration INTEGER NOT NULL,
    duration TEXT NOT NULL,
    range TEXT NOT NULL

}

CREATE TABLE spell_preset_component {
    spell_preset_uuid TEXT NOT NULL,
    component_name TEXT NOT NULL
    PRIMARY KEY(spell_preset_uuid, component_name)
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid) ON DELETE CASCADE
}

CREATE TABLE spell_preset_school {
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_school TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_school)
}

CREATE TABLE spell_preset_class {
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_class TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_class)
}

CREATE TABLE spell_preset_subclass {
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_subclass TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_subclass)
}

CREATE TABLE spell_preset_subclass {
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_species TEXT NULL, 
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_species)
}

CREATE TABLE spell_preset_species {
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_species TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_species)
}

CREATE TABLE spell_preset_feats {
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_feats TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_feats)
}

CREATE TABLE spell_list {
    uuid TEXT NOT NULL,
    owner_uuid TEXT NOT NULL,
    name TEXT NOT NULL
}

CREATE TABLE spell_list_entry {
    spell_preset_uuid TEXT NOT NULL,
    spell_list_uuid TEXT NOT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    FOREIGN KEY(spell_list_uuid) REFERENCES spell_list(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_list_uuid)
}

CREATE TABLE spell_slots {
    owner_uuid TEXT NOT NULL,
    spell_slot_level INTEGER NOT NULL,
    spell_slot_maxima INTEGER NOT NULL,
    spell_slot_available INTEGER NOT NULL,
    FOREIGN KEY(owner_uuid) REFERENCES "user"(uuid),
}


