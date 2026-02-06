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
    # Fallback: chạy tất cả migration theo thứ tự (1., 2., 3....)
    if [ -d "migrations" ]; then
        for f in $(ls migrations/*.sql 2>/dev/null | sort); do
            echo "Running migration: $f"
            psql "$DATABASE_URL" -f "$f" || exit 1
        done
        echo "Database initialized successfully (via psql)!"
    else
        echo "No migrations/ directory found."
        exit 1
    fi
else
    echo "Error: Neither sqlx nor psql found."
    exit 1
fi
