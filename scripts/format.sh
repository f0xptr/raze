#!/bin/bash

echo "Running cargo fmt..."
cargo fmt

echo "Running cargo clippy..."
cargo clippy -- -D warnings

if [ $? -eq 0 ]; then
  echo "Formatting and linting successful."
else
  echo "Formatting or linting failed. Please check the output above."
  exit 1
fi
