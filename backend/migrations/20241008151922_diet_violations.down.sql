-- Add down migration script here
ALTER TABLE ingredients 
RENAME COLUMN diet_violations TO diet_friendly;
