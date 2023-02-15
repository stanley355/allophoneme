-- Add up migration script here
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_updated_at_trigger 
BEFORE UPDATE ON stocks 
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();