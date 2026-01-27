module.exports = {
  apps: [{
    name: 'authify-api',
    script: '/opt/apps/authify/backend/target/release/authify-api',
    cwd: '/opt/apps/authify/backend',
    instances: 1,
    autorestart: true,
    watch: false,
    max_memory_restart: '500M',
    env: {
      RUST_LOG: 'info',
      PORT: '8081'
    },
    env_file: '/opt/apps/authify/backend/.env',
    error_file: '/opt/apps/authify/logs/api-error.log',
    out_file: '/opt/apps/authify/logs/api-out.log',
    log_date_format: 'YYYY-MM-DD HH:mm:ss Z',
    merge_logs: true
  }]
};
