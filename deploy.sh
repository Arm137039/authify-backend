#!/bin/bash
set -e

APP_DIR="/opt/apps/authify/backend"
cd $APP_DIR

echo "🔨 Building Rust application..."
cargo build --release

echo "🔄 Restarting PM2 process..."
pm2 restart authify-api || pm2 start ecosystem.config.js

echo "💾 Saving PM2 configuration..."
pm2 save

echo "✅ Deployment complete!"
pm2 status authify-api
