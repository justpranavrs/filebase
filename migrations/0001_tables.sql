-- Copyright 2026 Pranav R S.
-- Use of this source code is governed by the MIT license that can be found in the LICENSE file.

-- Table: folders
-- Stores the folder structure of the files stored in table "files"
CREATE TABLE IF NOT EXISTS folders (
    id              INTEGER PRIMARY KEY,
    parent_id       INTEGER REFERENCES folders(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at      TEXT
);

-- Table: files
-- Stores the metadata of the files which are stored in a Object Storage.
CREATE TABLE IF NOT EXISTS files (
    id              INTEGER PRIMARY KEY,
    folder_id       INTEGER NOT NULL REFERENCES folders(id) ON DELETE CASCADE,
    object_key      TEXT NOT NULL UNIQUE,
    name            TEXT NOT NULL,
    size            INTEGER NOT NULL,
    content_type    TEXT,
    checksum        TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
    deleted_at      TEXT 
);

-- An Index on the checksum field of the files table.
CREATE INDEX IF NOT EXISTS files_checksum_idx ON files(checksum);

-- Foreign Key Indexes and Name Collisions in the Same Folders.
CREATE UNIQUE INDEX IF NOT EXISTS files_folder_id_name_uq ON files(folder_id, name)
    WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS folders_parent_id_name_uq ON folders(parent_id, name)
    WHERE deleted_at IS NULL;

-- Create a Root Folder for the Filesystem
INSERT INTO folders (id, name) VALUES (1, 'filebase-root');
