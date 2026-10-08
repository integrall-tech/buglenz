-- Per-project retention periods (BugLenz, ADR-0009, package 004). A NULL column falls back to the
-- instance default; a project without a row uses the defaults for every type.
CREATE TABLE project_retention (
    project_id INTEGER PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    events_days INTEGER,
    transactions_days INTEGER,
    logs_days INTEGER,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
