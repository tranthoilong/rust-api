-- Seed initial roles
INSERT INTO roles (name)
VALUES ('admin'), ('user')
ON CONFLICT (name) DO NOTHING;

INSERT INTO permissions (name)
VALUES
  -- user
  ('user:login'),
  ('user:create'),
  ('user:read'),
  ('user:read_all'),
  ('user:update'),
  ('user:delete'),
  ('user:assign_role'),
  ('user:revoke_role'),
  -- role
  ('role:create'),
  ('role:read'),
  ('role:read_all'),
  ('role:update'),
  ('role:delete'),
  ('role:assign_permission'),
  ('role:revoke_permission'),
  -- permission
  ('permission:create'),
  ('permission:read'),
  ('permission:read_all'),
  ('permission:update'),
  ('permission:delete'),
  -- media
  ('media:create'),
  ('media:read')
ON CONFLICT (name) DO NOTHING;

-- Seed users (password mặc định: 123@Long!!)
INSERT INTO users (name, email, password)
VALUES
  ('Long Devlor', 'longdevlor@gmail.com', '$2b$12$26ee7ST00bNgEZwcb9OziOMZzA8hcd/VvBwU48i/e6PcxsHenClDC'),
  ('John Doe', 'john@example.com', '$2b$12$26ee7ST00bNgEZwcb9OziOMZzA8hcd/VvBwU48i/e6PcxsHenClDC'),
  ('Jane Smith', 'jane@example.com', '$2b$12$26ee7ST00bNgEZwcb9OziOMZzA8hcd/VvBwU48i/e6PcxsHenClDC'),
  ('Demo User', 'demo@example.com', '$2b$12$26ee7ST00bNgEZwcb9OziOMZzA8hcd/VvBwU48i/e6PcxsHenClDC')
ON CONFLICT (email) DO NOTHING;

-- Assign admin role to the admin user
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u
JOIN roles r ON r.name = 'admin'
WHERE u.email = 'longdevlor@gmail.com'
  AND NOT EXISTS (
    SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id AND ur.role_id = r.id
  );

-- Assign user role to sample users
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u
JOIN roles r ON r.name = 'user'
WHERE u.email = 'john@example.com'
  AND NOT EXISTS (
    SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id AND ur.role_id = r.id
  );

INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u
JOIN roles r ON r.name = 'user'
WHERE u.email = 'jane@example.com'
  AND NOT EXISTS (
    SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id AND ur.role_id = r.id
  );

INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u
JOIN roles r ON r.name = 'user'
WHERE u.email = 'demo@example.com'
  AND NOT EXISTS (
    SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id AND ur.role_id = r.id
  );

-- Grant all permissions to admin role
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
JOIN permissions p ON true
WHERE r.name = 'admin'
  AND NOT EXISTS (
    SELECT 1 FROM role_permissions rp WHERE rp.role_id = r.id AND rp.permission_id = p.id
  );
