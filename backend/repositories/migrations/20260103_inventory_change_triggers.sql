-- Add migration script here
-- Create generic trigger function that sends NOTIFY with detailed change information
CREATE OR REPLACE FUNCTION notify_inventory_change_generic()
RETURNS TRIGGER AS $$
DECLARE
    uuid TEXT;
    payload JSON;
BEGIN
    -- Find the inventory uuid based on the table
    CASE TG_TABLE_NAME
        WHEN 'inventory' THEN
            uuid := COALESCE(NEW.uuid, OLD.uuid);
        WHEN 'inventory_item' THEN
            uuid := COALESCE(NEW.inventory_uuid, OLD.inventory_uuid);
        WHEN 'inventory_reader' THEN
            uuid := COALESCE(NEW.inventory_uuid, OLD.inventory_uuid);
        WHEN 'inventory_writer' THEN
            uuid := COALESCE(NEW.inventory_uuid, OLD.inventory_uuid);
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
