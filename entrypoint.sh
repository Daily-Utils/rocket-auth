#!/bin/sh

set -e

sleep 10

echo "Running migrations"

diesel migration run

echo "Migrations complete"

./main
