
CREATE TABLE caustic_user (
    user_id TEXT PRIMARY KEY,
    email TEXT NOT NULL,
    created TEXT NOT NULL
);

CREATE TABLE caustic_project (
    project_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL,
    created TEXT NOT NULL,
    last_modified TEXT NOT NULL,
    FOREIGN KEY (owner_user_id) REFERENCES caustic_user(user_id)
);

CREATE TABLE caustic_project_file (
    project_file_id TEXT PRIMARY KEY,
    filename TEXT NOT NULL,
    created TEXT NOT NULL,
    last_modified TEXT NOT NULL
);
