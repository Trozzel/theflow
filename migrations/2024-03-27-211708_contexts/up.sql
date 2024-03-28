CREATE TABLE
  contexts (
    unique_id BIGINT PRIMARY KEY AUTO_INCREMENT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    parent_id BIGINT DEFAULT NULL,
    status CHAR(7) NOT NULL DEFAULT "Active" CHECK (
      status = "Active"
      OR status = "OnHold"
      OR status = "Dropped"
    ),
    created DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    notes TEXT,
    CONSTRAINT FOREIGN KEY fk_contexts_parent_id (parent_id) REFERENCES contexts (unique_id) ON DELETE SET NULL
  );
