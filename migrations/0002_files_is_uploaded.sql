-- Copyright 2026 Pranav R S.
-- Use of this source code is governed by the MIT license that can be found in the LICENSE file.

-- Alter Table: files
-- Stores the upload completion status of the file stored in Disk Storage.
ALTER TABLE files ADD COLUMN is_uploaded BOOLEAN NOT NULL DEFAULT 0;

-- An Index on the is_uploaded field of the files table.
CREATE INDEX IF NOT EXISTS files_is_uploaded_idx ON files(is_uploaded);

-- Mark Existing Files as Uploaded
UPDATE files SET is_uploaded = 1;
