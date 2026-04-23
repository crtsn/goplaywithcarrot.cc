-- Migration number: 0003 	 2026-04-23T02:13:32.862Z
ALTER TABLE player RENAME TO players;
INSERT INTO players VALUES (0, 'rabbit', 0, 0);
INSERT INTO players VALUES (1, 'rabbit2', 0, 0);
INSERT INTO players VALUES (2, 'frog', 0, 0);
INSERT INTO players VALUES (3, 'hedgehog', 0, 0);
