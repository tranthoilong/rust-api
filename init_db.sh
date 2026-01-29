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

# Run psql using the DATABASE_URL
if command -v psql &> /dev/null; then
    psql "$DATABASE_URL" -f init_schema.sql
    echo "Database initialized successfully!"
else
    echo "Error: psql is not installed or not in PATH."
    exit 1
fi
