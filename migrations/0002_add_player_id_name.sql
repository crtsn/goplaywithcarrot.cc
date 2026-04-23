-- Migration number: 0002 	 2026-04-23T01:37:57.538Z
ALTER TABLE player ADD COLUMN id int;
ALTER TABLE player ADD COLUMN name char(30);
