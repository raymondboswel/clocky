# Migrations

This directory contains SQL files for creating and updating the database schema.

To apply the migrations, you can use a tool like `sqlite3`:

```sh
sqlite3 clocky.db < migrations/2024XXXX_create_sessions_table.sql
