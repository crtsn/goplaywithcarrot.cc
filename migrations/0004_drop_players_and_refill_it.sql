-- Migration number: 0004 	 2026-04-23T12:59:50.992Z
DROP TABLE players;
CREATE TABLE players
(
	id int,
	name char(30),
	x int,
	y int
);
INSERT INTO players VALUES (0, 'rabbit', 0, 0);
INSERT INTO players VALUES (1, 'rabbit2', 0, 0);
INSERT INTO players VALUES (2, 'frog', 0, 0);
INSERT INTO players VALUES (3, 'hedgehog', 0, 0);
