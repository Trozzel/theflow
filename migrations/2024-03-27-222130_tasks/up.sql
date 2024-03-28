CREATE TABLE
  tasks (
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
    context_id BIGINT DEFAULT NULL,
    project_id BIGINT DEFAULT NULL,
    flagged boolean NOT NULL DEFAULT false,
    deferred DATETIME,
    due DATETIME,
    is_repeating boolean NOT NULL DEFAULT false,
    repeat_from CHAR(8) NOT NULL DEFAULT "Due" CHECK (
      repeat_from = "Due"
      OR repeat_from = "Deferred"
    ),
    repeat_schedule CHAR(50) DEFAULT '30 16 * * *' NOT NULL,
    task_type CHAR(10) NOT NULL DEFAULT "Parallel" CHECK (
      task_type = "Parallel"
      OR task_type = "Sequential"
    ),
    CONSTRAINT FOREIGN KEY fk_tasks_parent_id (parent_id) REFERENCES tasks (unique_id) ON DELETE SET NULL,
    CONSTRAINT FOREIGN KEY fk_tasks_context_id (context_id) REFERENCES contexts (unique_id) ON DELETE SET NULL,
    CONSTRAINT FOREIGN KEY fk_tasks_project_id (project_id) REFERENCES projects (unique_id) ON DELETE SET NULL
  );

