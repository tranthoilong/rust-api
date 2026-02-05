#!/bin/bash

# Load environment variables from .env if it exists
if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
fi

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
  echo "Error: DATABASE_URL is not set."
  exit 1
fi

echo "Initializing database..."

# Run migrations using sqlx
if command -v sqlx &> /dev/null; then
    sqlx migrate run
    echo "Database initialized successfully (via sqlx)!"
elif command -v psql &> /dev/null; then
    # Fallback to psql if sqlx is not available - finding latest migration
    LATEST_MIGRATION=$(ls migrations/*.sql | sort | tail -n 1)
    if [ -n "$LATEST_MIGRATION" ]; then
        psql "$DATABASE_URL" -f "$LATEST_MIGRATION"
        echo "Database initialized successfully (via psql using $LATEST_MIGRATION)!"
    else
        echo "No migration files found in migrations/"
        exit 1
    fi
else
    echo "Error: Neither sqlx nor psql found."
    exit 1
fi
