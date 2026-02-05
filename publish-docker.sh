#!/bin/bash

# Stop execution on error
set -e

echo "🐳 Login to Docker Hub..."
docker login

echo "🛠️  Building Docker Image (Platform: linux/amd64)..."
# Build for linux/amd64 explicitly to ensure compatibility with VPS
docker build --platform linux/amd64 -t longdevlor/baserust:latest .

echo "🚀 Pushing image to Docker Hub..."
docker push longdevlor/baserust:latest

echo "✅ Done! You can now run the image on your VPS."

