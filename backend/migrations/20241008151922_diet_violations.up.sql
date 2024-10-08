-- Add up migration script here
ALTER TABLE ingredients 
RENAME COLUMN diet_friendly TO diet_violations;
