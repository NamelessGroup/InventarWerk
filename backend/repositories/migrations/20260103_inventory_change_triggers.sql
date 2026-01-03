-- Add migration script here
-- Create trigger function that sends NOTIFY on inventory changes
CREATE OR REPLACE FUNCTION notify_inventory_change()
RETURNS TRIGGER AS $$
BEGIN
    -- Send notification with inventory UUID
    PERFORM pg_notify('inventory_changes', NEW.uuid::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for changes to inventory table (INSERT/UPDATE)
CREATE OR REPLACE TRIGGER inventory_change_trigger
AFTER INSERT OR UPDATE ON inventory
FOR EACH ROW
EXECUTE FUNCTION notify_inventory_change();

-- Create trigger function for inventory_item changes
CREATE OR REPLACE FUNCTION notify_inventory_change_from_item()
RETURNS TRIGGER AS $$
BEGIN
    -- Send notification with inventory UUID
    PERFORM pg_notify('inventory_changes', COALESCE(NEW.inventory_uuid, OLD.inventory_uuid)::text);
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

-- Trigger for inventory_item INSERT/UPDATE/DELETE
CREATE OR REPLACE TRIGGER inventory_item_change_trigger
AFTER INSERT OR UPDATE OR DELETE ON inventory_item
FOR EACH ROW
EXECUTE FUNCTION notify_inventory_change_from_item();
