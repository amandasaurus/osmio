# `osmio` Read and write OpenStreetMap file


The goal of this library is read and and write OpenStreetMap data files in pure Rust.

There is full read & write support for XML, OPL and read support for PBF file formats.

# Library

# Binaries

## `osmio-changeset-tags-to-sqlite`

Takes 2 arguments, a changeset file, and a filename for a SQLite database.
Creates a table `changeset`, with 2 columns, `changeset_id`, `other_tags` (a
JSON array of changeset tags).

# Copyright

Copyright MIT or Apache-2.0, 2017→2021 Amanda McCann <amanda@technomancy.org>
