#!/usr/bin/env bash
set -euo pipefail

DIST_DIR="dist"
PORT=3000

rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

echo ":: Building release binary..."
cargo build --release

echo ":: Starting server on port $PORT..."
./target/release/rust_portfolio &
SERVER_PID=$!

cleanup() { kill "$SERVER_PID" 2>/dev/null || true; wait "$SERVER_PID" 2>/dev/null || true; }
trap cleanup EXIT

echo ":: Waiting for server..."
for _ in $(seq 1 30); do
  curl -sf "http://localhost:$PORT" >/dev/null 2>&1 && break
  sleep 0.5
done

echo ":: Fetching rendered pages..."
curl -sf "http://localhost:$PORT/" -o "$DIST_DIR/index.html"

echo ":: Copying static assets..."
cp -r static "$DIST_DIR/static"

echo ":: Done! Output in $DIST_DIR/"
