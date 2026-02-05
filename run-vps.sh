#!/bin/bash

# Dừng khi có lỗi
set -e

IMAGE_NAME="longdevlor/baserust:latest"

# Các biến môi trường cần trên VPS
PORT="${PORT:-8000}"
DATABASE_URL="${DATABASE_URL:-postgres://rust:rust2026@localhost:5432/rust}"
JWT_SECRET="${JWT_SECRET:-changeme_jwt_secret}"

echo "🐳 Pull image từ Docker Hub..."
docker pull "$IMAGE_NAME"

echo "🧹 Dừng container cũ (nếu có)..."
docker rm -f baserust_app || true

echo "🚀 Chạy container mới..."
docker run -d \
  --name baserust_app \
  -p "$PORT:$PORT" \
  -e PORT="$PORT" \
  -e DATABASE_URL="$DATABASE_URL" \
  -e JWT_SECRET="$JWT_SECRET" \
  "$IMAGE_NAME"

echo "✅ Container đang chạy trên cổng $PORT"

