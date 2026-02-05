-- Seed initial roles (base)
INSERT INTO roles (name)
VALUES 
  ('admin'),
  ('editor'),
  ('user')
ON CONFLICT (name) DO NOTHING;

-- Seed permissions (chuẩn hóa theo module & action)
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
  ('user:read_profile'),
  ('user:update_profile'),
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
  ('media:read'),
  ('media:delete'),
  -- settings
  ('settings:read'),
  ('settings:update'),
  -- banner
  ('banner:create'),
  ('banner:read'),
  ('banner:update'),
  ('banner:delete'),
  -- category
  ('category:create'),
  ('category:read'),
  ('category:update'),
  ('category:delete'),
  -- tag
  ('tag:create'),
  ('tag:read'),
  ('tag:update'),
  ('tag:delete'),
  -- post
  ('post:create'),
  ('post:read'),
  ('post:read_all'),
  ('post:update'),
  ('post:delete'),
  -- audit log
  ('audit_log:read')
ON CONFLICT (name) DO NOTHING;

-- Seed users (password mặc định: 123@Long!!)
INSERT INTO users (name, email, password)
VALUES
  ('Long Devlor', 'longdevlor@gmail.com', '$2b$12$26ee7ST00bNgEZwcb9OziOMZzA8hcd/VvBwU48i/e6PcxsHenClDC'),
  ('John Editor', 'editor@example.com', '$2b$12$26ee7ST00bNgEZwcb9OziOMZzA8hcd/VvBwU48i/e6PcxsHenClDC'),
  ('Jane User', 'user@example.com', '$2b$12$26ee7ST00bNgEZwcb9OziOMZzA8hcd/VvBwU48i/e6PcxsHenClDC')
ON CONFLICT (email) DO NOTHING;

-- Assign admin role to the admin user (full quyền)
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u
JOIN roles r ON r.name = 'admin'
WHERE u.email = 'longdevlor@gmail.com'
  AND NOT EXISTS (
    SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id AND ur.role_id = r.id
  );

-- Assign editor role cho John Editor
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u
JOIN roles r ON r.name = 'editor'
WHERE u.email = 'editor@example.com'
  AND NOT EXISTS (
    SELECT 1 FROM user_roles ur WHERE ur.user_id = u.id AND ur.role_id = r.id
  );

-- Assign user role cho Jane User (role mặc định cho end-user)
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u
JOIN roles r ON r.name = 'user'
WHERE u.email = 'user@example.com'
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

-- ============================================================
-- Seed dữ liệu đa ngôn ngữ & nội dung demo
-- ============================================================

-- Ngôn ngữ hệ thống
INSERT INTO languages (code, name, is_default)
VALUES
  ('vi', 'Tiếng Việt', TRUE),
  ('en', 'English', FALSE)
ON CONFLICT (code) DO NOTHING;

-- Category cho bài viết (type = 'post')
INSERT INTO categories (parent_id, name, slug, type, description)
VALUES
  (NULL, 'Tin tức', 'news', 'post', 'Tin tức chung'),
  (NULL, 'Blog kỹ thuật', 'tech-blog', 'post', 'Bài viết kỹ thuật'),
  (NULL, 'Hướng dẫn', 'guides', 'post', 'Hướng dẫn sử dụng hệ thống')
ON CONFLICT (slug, type) DO NOTHING;

-- Tag cho bài viết (type = 'post')
INSERT INTO tags (name, slug, type, description)
VALUES
  ('Rust', 'rust', 'post', 'Bài viết về Rust'),
  ('Backend', 'backend', 'post', 'Bài viết về backend'),
  ('DevOps', 'devops', 'post', 'CI/CD, Docker, Kubernetes'),
  ('Tutorial', 'tutorial', 'post', 'Bài hướng dẫn')
ON CONFLICT (slug, type) DO NOTHING;

-- Bài viết demo (posts) - sử dụng author là admin nếu tồn tại, fallback editor
INSERT INTO posts (author_id, category_id, title, slug, excerpt, content, status, published_at)
SELECT
  -- author: ưu tiên admin
  COALESCE(
    (SELECT id FROM users WHERE email = 'longdevlor@gmail.com'),
    (SELECT id FROM users WHERE email = 'editor@example.com')
  ) AS author_id,
  c.id AS category_id,
  'Chào mừng đến với Rust API Starter' AS title,
  'welcome-to-rust-api-starter' AS slug,
  'Giới thiệu nhanh về base project Rust API.' AS excerpt,
  'Đây là bài viết demo giới thiệu về base project Rust API với RBAC, i18n, banner, post, tag, category...' AS content,
  'published'::post_status AS status,
  NOW() AS published_at
FROM categories c
WHERE c.slug = 'news' AND c.type = 'post'
  AND NOT EXISTS (
    SELECT 1 FROM posts p WHERE p.slug = 'welcome-to-rust-api-starter'
  );

-- Gán tag cho bài viết demo
INSERT INTO post_tags (post_id, tag_id)
SELECT p.id, t.id
FROM posts p
JOIN tags t ON t.slug IN ('rust', 'backend', 'tutorial') AND t.type = 'post'
WHERE p.slug = 'welcome-to-rust-api-starter'
  AND NOT EXISTS (
    SELECT 1 FROM post_tags pt WHERE pt.post_id = p.id AND pt.tag_id = t.id
  );

-- Bản dịch bài viết demo (vi/en)
INSERT INTO post_translations (post_id, language_code, title, slug, excerpt, content)
SELECT p.id, 'vi', 
       'Chào mừng đến với Rust API Starter',
       'chao-mung-den-voi-rust-api-starter',
       'Giới thiệu nhanh về base project Rust API.',
       'Đây là bản dịch tiếng Việt cho bài viết demo Rust API Starter.'
FROM posts p
WHERE p.slug = 'welcome-to-rust-api-starter'
  AND NOT EXISTS (
    SELECT 1 FROM post_translations pt WHERE pt.post_id = p.id AND pt.language_code = 'vi'
  );

INSERT INTO post_translations (post_id, language_code, title, slug, excerpt, content)
SELECT p.id, 'en', 
       'Welcome to Rust API Starter',
       'welcome-to-rust-api-starter-en',
       'Quick introduction to the Rust API base project.',
       'This is the English translation for the Rust API Starter demo post.'
FROM posts p
WHERE p.slug = 'welcome-to-rust-api-starter'
  AND NOT EXISTS (
    SELECT 1 FROM post_translations pt WHERE pt.post_id = p.id AND pt.language_code = 'en'
  );

-- Banner demo
INSERT INTO banners (title, slug, "key", description, is_active, starts_at, ends_at)
VALUES
  ('Trang chủ - Hero Banner', 'home-hero', 'home_hero', 'Banner chính ở trang chủ', TRUE, NOW(), NOW() + INTERVAL '30 days')
ON CONFLICT (slug) DO NOTHING;

-- Item cho banner demo
INSERT INTO banner_items (banner_id, title, subtitle, image_url, link_url, position)
SELECT b.id,
       'Bắt đầu với Rust API',
       'Base project đã cấu hình sẵn auth, RBAC, i18n...',
       '/images/banners/rust-api-starter.png',
       '/docs/getting-started',
       1
FROM banners b
WHERE b.slug = 'home-hero'
  AND NOT EXISTS (
    SELECT 1 FROM banner_items bi WHERE bi.banner_id = b.id AND bi.position = 1
  );

-- Bản dịch cho banner demo (vi/en)
INSERT INTO banner_translations (banner_id, language_code, title, description)
SELECT b.id, 'vi',
       'Trang chủ - Hero Banner',
       'Banner chính giới thiệu Rust API Starter'
FROM banners b
WHERE b.slug = 'home-hero'
  AND NOT EXISTS (
    SELECT 1 FROM banner_translations bt WHERE bt.banner_id = b.id AND bt.language_code = 'vi'
  );

INSERT INTO banner_translations (banner_id, language_code, title, description)
SELECT b.id, 'en',
       'Home - Hero Banner',
       'Main hero banner for Rust API Starter'
FROM banners b
WHERE b.slug = 'home-hero'
  AND NOT EXISTS (
    SELECT 1 FROM banner_translations bt WHERE bt.banner_id = b.id AND bt.language_code = 'en'
  );

