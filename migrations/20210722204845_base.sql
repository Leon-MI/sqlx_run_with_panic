DROP TABLE IF EXISTS node;
CREATE TABLE node (
	id INTEGER NOT NULL, 
	parent_id INTEGER,
    name VARCHAR NOT NULL,

	PRIMARY KEY (id), 
	FOREIGN KEY(parent_id) REFERENCES node (id)
);

INSERT INTO node (id, parent_id, name) VALUES (1, NULL, "root_node");
INSERT INTO node (id, parent_id, name) VALUES (2, 1, "child_node");

