-- Add migration script here

CREATE TABLE spell_preset (
    uuid TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    level INTEGER NOT NULL,
    description TEXT NOT NULL,
    time TEXT NOT NULL,
    concentration INTEGER NOT NULL,
    duration TEXT NOT NULL,
    range TEXT NOT NULL
);

CREATE TABLE spell_preset_component (
    spell_preset_uuid TEXT NOT NULL,
    component_name TEXT NOT NULL,
    PRIMARY KEY(spell_preset_uuid, component_name),
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid) ON DELETE CASCADE
);

CREATE TABLE spell_preset_school (
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_school TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_school)
);

CREATE TABLE spell_preset_class (
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_class TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_class)
);

CREATE TABLE spell_preset_subclass (
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_subclass TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_subclass)
);

CREATE TABLE spell_preset_species (
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_species TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_species)
);

CREATE TABLE spell_preset_feats (
    spell_preset_uuid TEXT NOT NULL,
    spell_preset_feats TEXT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_preset_feats)
);

CREATE TABLE spell_list (
    uuid TEXT NOT NULL PRIMARY KEY,
    owner_uuid TEXT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE spell_list_entry (
    spell_preset_uuid TEXT NOT NULL,
    spell_list_uuid TEXT NOT NULL,
    FOREIGN KEY(spell_preset_uuid) REFERENCES spell_preset(uuid),
    FOREIGN KEY(spell_list_uuid) REFERENCES spell_list(uuid),
    PRIMARY KEY(spell_preset_uuid, spell_list_uuid)
);

CREATE TABLE spell_slots (
    owner_uuid TEXT NOT NULL,
    spell_slot_level INTEGER NOT NULL,
    spell_slot_maxima INTEGER NOT NULL,
    spell_slot_available INTEGER NOT NULL,
    FOREIGN KEY(owner_uuid) REFERENCES "user"(uuid),
    PRIMARY KEY(owner_uuid, spell_slot_level)
);

CREATE TABLE concentration (
    owner_uuid TEXT NOT NULL,
    concentration INT NOT NULL,
    FOREIGN KEY(owner_uuid) REFERENCES "user"(uuid),
    PRIMARY KEY(owner_uuid)
);

CREATE OR REPLACE FUNCTION notify_inventory_change_generic()
RETURNS TRIGGER AS $$
DECLARE
    uuid TEXT;
    payload JSON;
BEGIN
    -- Find the relevant uuid based on the table
    CASE TG_TABLE_NAME
        -- Inventory-related tables
        WHEN 'inventory' THEN
            uuid := COALESCE(NEW.uuid, OLD.uuid);
        WHEN 'inventory_item' THEN
            uuid := COALESCE(NEW.inventory_uuid, OLD.inventory_uuid);
        WHEN 'inventory_reader' THEN
            uuid := COALESCE(NEW.inventory_uuid, OLD.inventory_uuid);
        WHEN 'inventory_writer' THEN
            uuid := COALESCE(NEW.inventory_uuid, OLD.inventory_uuid);
        -- Item preset table
        WHEN 'item_preset' THEN
            uuid := COALESCE(NEW.uuid, OLD.uuid);
        -- Spell-related tables
        WHEN 'spell_preset' THEN
            uuid := COALESCE(NEW.uuid, OLD.uuid);
        WHEN 'spell_preset_component' THEN
            uuid := COALESCE(NEW.spell_preset_uuid, OLD.spell_preset_uuid);
        WHEN 'spell_preset_school' THEN
            uuid := COALESCE(NEW.spell_preset_uuid, OLD.spell_preset_uuid);
        WHEN 'spell_preset_class' THEN
            uuid := COALESCE(NEW.spell_preset_uuid, OLD.spell_preset_uuid);
        WHEN 'spell_preset_subclass' THEN
            uuid := COALESCE(NEW.spell_preset_uuid, OLD.spell_preset_uuid);
        WHEN 'spell_preset_species' THEN
            uuid := COALESCE(NEW.spell_preset_uuid, OLD.spell_preset_uuid);
        WHEN 'spell_preset_feats' THEN
            uuid := COALESCE(NEW.spell_preset_uuid, OLD.spell_preset_uuid);
        WHEN 'spell_list' THEN
            uuid := COALESCE(NEW.uuid, OLD.uuid);
        WHEN 'spell_list_entry' THEN
            uuid := COALESCE(NEW.spell_list_uuid, OLD.spell_list_uuid);
        WHEN 'spell_slots' THEN
            uuid := COALESCE(NEW.owner_uuid, OLD.owner_uuid);
        WHEN 'concentration' THEN
            uuid := COALESCE(NEW.owner_uuid, OLD.owner_uuid);
        -- Add more tables here as needed
        ELSE
            RAISE EXCEPTION 'Unhandled table: %', TG_TABLE_NAME;
    END CASE;
    
    -- Build JSON payload with uuid, source table, and operation type
    payload := json_build_object(
        'uuid', uuid,
        'source', TG_TABLE_NAME,
        'type', TG_OP  -- INSERT, UPDATE, or DELETE
    );
    
    -- Send NOTIFY with JSON payload
    PERFORM pg_notify('inventory_changes', payload::text);
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Trigger for inventory table
CREATE OR REPLACE TRIGGER inventory_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON inventory
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for inventory_item table
CREATE OR REPLACE TRIGGER inventory_item_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON inventory_item
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for inventory_reader table
CREATE OR REPLACE TRIGGER inventory_reader_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON inventory_reader
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for inventory_writer table
CREATE OR REPLACE TRIGGER inventory_writer_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON inventory_writer
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for item_preset table
CREATE OR REPLACE TRIGGER item_preset_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON item_preset
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_preset table
CREATE OR REPLACE TRIGGER spell_preset_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_preset
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_preset_component table
CREATE OR REPLACE TRIGGER spell_preset_component_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_preset_component
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_preset_school table
CREATE OR REPLACE TRIGGER spell_preset_school_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_preset_school
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_preset_class table
CREATE OR REPLACE TRIGGER spell_preset_class_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_preset_class
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_preset_subclass table
CREATE OR REPLACE TRIGGER spell_preset_subclass_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_preset_subclass
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_preset_species table
CREATE OR REPLACE TRIGGER spell_preset_species_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_preset_species
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_preset_feats table
CREATE OR REPLACE TRIGGER spell_preset_feats_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_preset_feats
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_list table
CREATE OR REPLACE TRIGGER spell_list_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_list
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_list_entry table
CREATE OR REPLACE TRIGGER spell_list_entry_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_list_entry
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for spell_slots table
CREATE OR REPLACE TRIGGER spell_slots_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON spell_slots
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();

-- Trigger for concentration table
CREATE OR REPLACE TRIGGER concentration_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON concentration
FOR EACH ROW EXECUTE FUNCTION notify_inventory_change_generic();
