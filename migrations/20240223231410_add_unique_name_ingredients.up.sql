-- Delete duplicates
DELETE FROM ingredients a USING ingredients b WHERE a.id < b.id AND a.name = b.name;
ALTER TABLE ingredients ADD CONSTRAINT unique_name UNIQUE(name);
