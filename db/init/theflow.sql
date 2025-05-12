-- ENUM type for status
CREATE TYPE status_enum AS ENUM ('active', 'onhold', 'dropped');

-- Trigger function for timestamps
CREATE OR REPLACE FUNCTION set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  IF TG_OP = 'INSERT' THEN
    NEW.created := now();
  END IF;
  NEW.modified := now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================
-- Contexts Table
-- =============================
CREATE TABLE contexts (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  parent_id UUID REFERENCES contexts(id) ON UPDATE CASCADE ON DELETE SET NULL,
  status status_enum NOT NULL DEFAULT 'active',
  created TIMESTAMP,
  modified TIMESTAMP,
  notes TEXT
);

CREATE TRIGGER trg_contexts_timestamp
BEFORE INSERT OR UPDATE ON contexts
FOR EACH ROW EXECUTE FUNCTION set_timestamp();

CREATE INDEX idx_contexts_status ON contexts(status);
CREATE INDEX idx_contexts_parent_id ON contexts(parent_id);

-- =============================
-- Folders Table
-- =============================
CREATE TABLE folders (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  parent_id UUID REFERENCES folders(id) ON UPDATE CASCADE ON DELETE SET NULL,
  status status_enum NOT NULL DEFAULT 'active',
  created TIMESTAMP,
  modified TIMESTAMP,
  notes TEXT
);

CREATE TRIGGER trg_folders_timestamp
BEFORE INSERT OR UPDATE ON folders
FOR EACH ROW EXECUTE FUNCTION set_timestamp();

CREATE INDEX idx_folders_status ON folders(status);
CREATE INDEX idx_folders_parent_id ON folders(parent_id);

-- =============================
-- Projects Table
-- =============================
CREATE TABLE projects (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  parent_id UUID REFERENCES projects(id) ON UPDATE CASCADE ON DELETE SET NULL,
  folder_id UUID REFERENCES folders(id) ON UPDATE CASCADE ON DELETE SET NULL,
  status status_enum NOT NULL DEFAULT 'active',
  created TIMESTAMP,
  modified TIMESTAMP,
  notes TEXT,
  flagged BOOLEAN DEFAULT false,
  deferred TIMESTAMP,
  due TIMESTAMP,
  is_repeating BOOLEAN DEFAULT false,
  repeat_from TEXT CHECK (repeat_from IN ('due', 'deferred')),
  repeat_sched TEXT,
  complete_with_last BOOLEAN DEFAULT false
);

CREATE TRIGGER trg_projects_timestamp
BEFORE INSERT OR UPDATE ON projects
FOR EACH ROW EXECUTE FUNCTION set_timestamp();

CREATE INDEX idx_projects_status ON projects(status);
CREATE INDEX idx_projects_parent_id ON projects(parent_id);
CREATE INDEX idx_projects_folder_id ON projects(folder_id);
CREATE INDEX idx_projects_flagged ON projects(flagged);
CREATE INDEX idx_projects_deferred ON projects(deferred);
CREATE INDEX idx_projects_due ON projects(due);

-- =============================
-- Tasks Table
-- =============================
CREATE TABLE tasks (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  parent_id UUID REFERENCES tasks(id) ON UPDATE CASCADE ON DELETE SET NULL,
  project_id UUID REFERENCES projects(id) ON UPDATE CASCADE ON DELETE SET NULL,
  status status_enum NOT NULL DEFAULT 'active',
  created TIMESTAMP,
  modified TIMESTAMP,
  notes TEXT,
  flagged BOOLEAN DEFAULT false,
  deferred TIMESTAMP,
  due TIMESTAMP,
  is_repeating BOOLEAN DEFAULT false,
  repeat_from TEXT CHECK (repeat_from IN ('due', 'deferred')),
  repeat_sched TEXT
);

CREATE TRIGGER trg_tasks_timestamp
BEFORE INSERT OR UPDATE ON tasks
FOR EACH ROW EXECUTE FUNCTION set_timestamp();

CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_parent_id ON tasks(parent_id);
CREATE INDEX idx_tasks_project_id ON tasks(project_id);
CREATE INDEX idx_tasks_flagged ON tasks(flagged);
CREATE INDEX idx_tasks_deferred ON tasks(deferred);
CREATE INDEX idx_tasks_due ON tasks(due);

-- =============================
-- Join Tables for Contexts
-- =============================
CREATE TABLE project_contexts (
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  context_id UUID REFERENCES contexts(id) ON DELETE CASCADE,
  PRIMARY KEY (project_id, context_id)
);

CREATE TABLE task_contexts (
  task_id UUID REFERENCES tasks(id) ON DELETE CASCADE,
  context_id UUID REFERENCES contexts(id) ON DELETE CASCADE,
  PRIMARY KEY (task_id, context_id)
);

-- Optional: indexes on join tables for faster lookups
CREATE INDEX idx_project_contexts_context_id ON project_contexts(context_id);
CREATE INDEX idx_task_contexts_context_id ON task_contexts(context_id);

