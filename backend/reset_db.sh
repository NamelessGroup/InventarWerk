#!/bin/bash

# Stop on error
set -e

# Go to the script directory (backend/)
cd "$(dirname "$0")"

echo "Using .env file in $(pwd)"

# Check if sqlx is installed
if ! command -v sqlx &> /dev/null; then
    echo "sqlx command not found!"
    echo "The project uses sqlx for database management."
    echo "Aborting. Please install sqlx-cli manually. You may try `cargo install sqlx-cli --no-default-features --features postgres`"
fi

# Drop the database
echo "Dropping database..."
if sqlx database drop -y; then
    echo "Database dropped successfully."
else
    echo "Database drop failed (it might not exist). Proceeding..."
fi

# Create the database
echo "Creating database..."
sqlx database create

echo "Done. The database has been recreated and is empty."
echo "You can now run migrations, for example:"
echo "cd repositories && sqlx migrate run"
